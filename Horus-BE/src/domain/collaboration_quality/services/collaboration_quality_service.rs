use std::collections::HashSet;

use crate::domain::collaboration_quality::{
    entities::{CodeReview, CommentType, ReviewStatus, TeamInteraction},
    repositories::CollaborationQualityRepository,
    traits::{CollaborationAnalytics, GitHubCollaborationProvider},
    value_objects::{CollaborationMetrics, ReviewQualityMetrics, TeamCollaborationMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};

/// Service for analyzing team collaboration metrics and activity
pub struct CollaborationQualityAnalyzer<
    G: GitHubCollaborationProvider,
    R: CollaborationQualityRepository,
> {
    github_provider: G,
    repository: R,
}

impl<G: GitHubCollaborationProvider, R: CollaborationQualityRepository>
    CollaborationQualityAnalyzer<G, R>
{
    pub fn new(github_provider: G, repository: R) -> Self {
        Self {
            github_provider,
            repository,
        }
    }

    fn calculate_review_quality_metrics(&self, reviews: &[CodeReview]) -> ReviewQualityMetrics {
        let total_reviews = reviews.len();
        if total_reviews == 0 {
            return ReviewQualityMetrics {
                average_review_time: Duration::zero(),
                comments_per_review: 0.0,
                substantive_comments_ratio: 0.0,
                review_coverage: 0.0,
                average_iterations: 0.0,
            };
        }

        // Calculate average review time
        let review_times: Vec<Duration> = reviews
            .iter()
            .filter_map(|r| r.completed_at.map(|c| c - r.started_at))
            .collect();

        let average_review_time = if !review_times.is_empty() {
            review_times.iter().sum::<Duration>() / review_times.len() as i32
        } else {
            Duration::zero()
        };

        // Calculate comments per review
        let total_comments: usize = reviews.iter().map(|r| r.comments.len()).sum();
        let comments_per_review = total_comments as f64 / total_reviews as f64;

        // Calculate substantive comments ratio
        let substantive_comments = reviews
            .iter()
            .flat_map(|r| &r.comments)
            .filter(|c| {
                matches!(
                    c.comment_type,
                    CommentType::CodeSuggestion | CommentType::BugReport
                )
            })
            .count();
        let substantive_comments_ratio = substantive_comments as f64 / total_comments.max(1) as f64;

        // Calculate review coverage and iterations
        let reviewed_prs = reviews
            .iter()
            .filter(|r| r.status != ReviewStatus::Pending)
            .count();
        let review_coverage = reviewed_prs as f64 / total_reviews as f64;

        let total_iterations = reviews
            .iter()
            .filter(|r| r.status == ReviewStatus::ChangesRequested)
            .count();
        let average_iterations = total_iterations as f64 / total_reviews as f64 + 1.0;

        ReviewQualityMetrics {
            average_review_time,
            comments_per_review,
            substantive_comments_ratio,
            review_coverage,
            average_iterations,
        }
    }

    fn calculate_team_collaboration_metrics(
        &self,
        reviews: &[CodeReview],
        interactions: &[TeamInteraction],
    ) -> TeamCollaborationMetrics {
        let total_reviews = reviews.len();
        if total_reviews == 0 {
            return TeamCollaborationMetrics {
                cross_team_reviews: 0,
                knowledge_sharing_index: 0.0,
                review_participation_rate: 0.0,
                average_response_time: Duration::zero(),
                collaboration_network_density: 0.0,
            };
        }

        // Calculate cross-team reviews
        let cross_team_reviews = reviews.iter().filter(|r| r.reviewer != r.author).count();

        // Calculate knowledge sharing index
        let unique_reviewers: HashSet<_> = reviews.iter().map(|r| &r.reviewer).collect();
        let knowledge_sharing_index = unique_reviewers.len() as f64 / total_reviews as f64;

        // Calculate review participation rate
        let total_team_members = interactions
            .iter()
            .flat_map(|i| &i.participants)
            .collect::<HashSet<_>>()
            .len();
        let review_participation_rate =
            unique_reviewers.len() as f64 / total_team_members.max(1) as f64;

        // Calculate average response time
        let response_times: Vec<Duration> = reviews
            .iter()
            .filter_map(|r| r.comments.first().map(|c| c.created_at - r.started_at))
            .collect();
        let average_response_time = if !response_times.is_empty() {
            response_times.iter().sum::<Duration>() / response_times.len() as i32
        } else {
            Duration::zero()
        };

        // Calculate collaboration network density
        let total_possible_connections = total_team_members * (total_team_members - 1) / 2;
        let actual_connections = interactions.len();
        let collaboration_network_density =
            actual_connections as f64 / total_possible_connections.max(1) as f64;

        TeamCollaborationMetrics {
            cross_team_reviews,
            knowledge_sharing_index,
            review_participation_rate,
            average_response_time,
            collaboration_network_density,
        }
    }

    async fn fetch_and_store_code_reviews(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<CodeReview>> {
        // First try to get from repository
        let stored_reviews = self
            .repository
            .get_code_reviews(team_id, start_date, end_date)
            .await?;

        if !stored_reviews.is_empty() {
            return Ok(stored_reviews);
        }

        // If not found, fetch from GitHub and store
        let reviews = self
            .github_provider
            .fetch_code_reviews(team_id, Some(start_date), Some(end_date))
            .await?;

        // Store in repository
        for review in &reviews {
            self.repository.save_code_review(review).await?;
        }

        Ok(reviews)
    }

    async fn fetch_and_store_team_interactions(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<TeamInteraction>> {
        // First try to get from repository
        let stored_interactions = self
            .repository
            .get_team_interactions(team_id, start_date, end_date)
            .await?;

        if !stored_interactions.is_empty() {
            return Ok(stored_interactions);
        }

        // If not found, fetch from GitHub and store
        let interactions = self
            .github_provider
            .fetch_team_interactions(team_id, Some(start_date), Some(end_date))
            .await?;

        // Store in repository
        for interaction in &interactions {
            self.repository.save_team_interaction(interaction).await?;
        }

        Ok(interactions)
    }

    async fn store_metrics(
        &self,
        team_id: &str,
        review_quality: &ReviewQualityMetrics,
        team_collaboration: &TeamCollaborationMetrics,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<()> {
        // Store review quality metrics
        self.repository
            .save_review_metrics(team_id, review_quality, period_start, period_end)
            .await?;

        // Store team collaboration metrics
        self.repository
            .save_collaboration_metrics(team_id, team_collaboration, period_start, period_end)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl<
        G: GitHubCollaborationProvider + Send + Sync,
        R: CollaborationQualityRepository + Send + Sync,
    > CollaborationAnalytics for CollaborationQualityAnalyzer<G, R>
{
    async fn get_collaboration_metrics(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<CollaborationMetrics> {
        let reviews = self
            .fetch_and_store_code_reviews(team_id, start_date, end_date)
            .await?;
        let interactions = self
            .fetch_and_store_team_interactions(team_id, start_date, end_date)
            .await?;

        let review_quality = self.calculate_review_quality_metrics(&reviews);
        let team_collaboration = self.calculate_team_collaboration_metrics(&reviews, &interactions);

        // Store metrics for future use
        self.store_metrics(
            team_id,
            &review_quality,
            &team_collaboration,
            start_date,
            end_date,
        )
        .await?;

        Ok(CollaborationMetrics {
            team_id: team_id.to_string(),
            period_start: start_date,
            period_end: end_date,
            review_quality,
            team_collaboration,
        })
    }

    async fn get_code_reviews(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<CodeReview>> {
        self.fetch_and_store_code_reviews(team_id, start_date, end_date)
            .await
    }

    async fn get_team_interactions(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<TeamInteraction>> {
        self.fetch_and_store_team_interactions(team_id, start_date, end_date)
            .await
    }

    async fn get_review_quality_metrics(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<ReviewQualityMetrics> {
        let reviews = self.get_code_reviews(team_id, start_date, end_date).await?;
        Ok(self.calculate_review_quality_metrics(&reviews))
    }

    async fn get_team_collaboration_metrics(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<TeamCollaborationMetrics> {
        let reviews = self.get_code_reviews(team_id, start_date, end_date).await?;
        let interactions = self
            .get_team_interactions(team_id, start_date, end_date)
            .await?;
        Ok(self.calculate_team_collaboration_metrics(&reviews, &interactions))
    }
}
