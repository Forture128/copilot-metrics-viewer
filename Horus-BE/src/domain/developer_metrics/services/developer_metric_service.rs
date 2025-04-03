use crate::domain::developer_metrics::{
    traits::{DeveloperAnalytics, GitHubDataProvider},
    CodeChangeMetrics, Commit, CommitFrequency, Metrics, PullRequest, ReviewMetrics,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};

/// Service for analyzing developer metrics and activity
pub struct DeveloperMetricsAnalyzer<G: GitHubDataProvider> {
    github_provider: G,
}

impl<G: GitHubDataProvider> DeveloperMetricsAnalyzer<G> {
    pub fn new(github_provider: G) -> Self {
        Self { github_provider }
    }

    fn calculate_commit_frequency(
        &self,
        commits: &[Commit],
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> CommitFrequency {
        let total_commits = commits.len();
        let days = (end_date - start_date).num_days() as f64;
        let commits_per_day = if days > 0.0 {
            total_commits as f64 / days
        } else {
            0.0
        };

        CommitFrequency {
            total_commits,
            commits_per_day,
            period_start: start_date,
            period_end: end_date,
        }
    }

    fn calculate_review_metrics(&self, pull_requests: &[PullRequest]) -> ReviewMetrics {
        let total_reviews = pull_requests.len();
        let review_times: Vec<Duration> = pull_requests
            .iter()
            .filter_map(|pr| pr.review_time)
            .collect();

        let average_review_time = if !review_times.is_empty() {
            review_times.iter().sum::<Duration>() / review_times.len() as i32
        } else {
            Duration::zero()
        };

        let reviews_per_week = total_reviews as f64 / 7.0;

        ReviewMetrics {
            total_reviews,
            average_review_time,
            reviews_per_week,
        }
    }

    fn calculate_code_changes(&self, commits: &[Commit]) -> CodeChangeMetrics {
        let lines_added: usize = commits.iter().map(|c| c.additions).sum();
        let lines_deleted: usize = commits.iter().map(|c| c.deletions).sum();
        let files_changed: usize = commits.iter().map(|c| c.files_changed).sum();
        let changes_per_commit = if !commits.is_empty() {
            (lines_added + lines_deleted) as f64 / commits.len() as f64
        } else {
            0.0
        };

        CodeChangeMetrics {
            lines_added,
            lines_deleted,
            files_changed,
            changes_per_commit,
        }
    }

    // fn calculate_circular_time(&self, commits: &[Commit]) -> CircularTime {
    //     let mut circular_time = CircularTime::new();
    //     let mut last_commit_time = None;

    //     for commit in commits {
    //         let commit_time = commit.created_at;
    //         if let Some(last_time) = last_commit_time {
    //             let time_diff = commit_time - last_time;
    //             circular_time.add_time_diff(time_diff);
    //         }
    //         last_commit_time = Some(commit_time);
    //     }

    //     circular_time
    // }
}

#[async_trait]
impl<G: GitHubDataProvider + Send + Sync> DeveloperAnalytics for DeveloperMetricsAnalyzer<G> {
    async fn get_developer_metrics(
        &self,
        username: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Metrics> {
        let commits = self.get_commits(username, start_date, end_date).await?;
        let pull_requests = self
            .get_pull_requests(username, start_date, end_date)
            .await?;

        let commit_frequency = self.calculate_commit_frequency(&commits, start_date, end_date);
        let review_metrics = self.calculate_review_metrics(&pull_requests);
        let code_changes = self.calculate_code_changes(&commits);

        Ok(Metrics {
            developer_id: username.to_string(),
            period_start: start_date,
            period_end: end_date,
            commit_frequency,
            review_metrics,
            code_changes,
        })
    }

    async fn get_commits(
        &self,
        username: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Commit>> {
        self.github_provider
            .fetch_user_commits(username, Some(start_date), Some(end_date))
            .await
    }

    async fn get_pull_requests(
        &self,
        username: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<PullRequest>> {
        self.github_provider
            .fetch_user_pull_requests(username, Some(start_date), Some(end_date))
            .await
    }
}
