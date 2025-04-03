use crate::domain::collaboration_quality::{
    entities::{
        CodeReview, CommentType, InteractionType, ReviewComment, ReviewStatus, TeamInteraction,
    },
    traits::GitHubCollaborationProvider,
};
use crate::infrastructure::github_sdk::client::GitHubSdk;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::models::IssueState;

/// Adapter for fetching collaboration data from GitHub
pub struct GitHubCollaborationAdapter {
    github_sdk: GitHubSdk,
}

impl GitHubCollaborationAdapter {
    pub fn new(github_sdk: GitHubSdk) -> Self {
        Self { github_sdk }
    }

    fn convert_to_code_review(&self, pr: octocrab::models::pulls::PullRequest) -> CodeReview {
        let status = match pr.state {
            Some(IssueState::Open) => ReviewStatus::Pending,
            Some(IssueState::Closed) if pr.merged_at.is_some() => ReviewStatus::Approved,
            _ => ReviewStatus::Commented,
        };
        let author = pr.user.as_ref().unwrap().id.to_string();

        let comments = pr
            .comments
            .map(|_| {
                vec![ReviewComment {
                    id: format!("comment-{}", pr.number),
                    body: "".to_string(),
                    author: author.clone(),
                    created_at: pr.created_at.unwrap_or_default(),
                    line_number: None,
                    file_path: None,
                    comment_type: CommentType::General,
                }]
            })
            .unwrap_or_default();

        CodeReview {
            id: pr.id.to_string(),
            pull_request_id: pr.number.to_string(),
            reviewer: pr.assignee.as_ref().unwrap().id.to_string(),
            author: author.clone(),
            started_at: pr.created_at.unwrap_or_default(),
            completed_at: pr.closed_at,
            comments,
            status,
        }
    }

    fn convert_to_team_interaction(
        &self,
        pr: octocrab::models::pulls::PullRequest,
        team_id: &str,
    ) -> TeamInteraction {
        TeamInteraction {
            id: pr.id.to_string(),
            team_id: team_id.to_string(),
            interaction_type: InteractionType::CodeReview,
            participants: vec![
                pr.user.as_ref().unwrap().id.to_string(),
                pr.assignee.as_ref().unwrap().id.to_string(),
            ],
            timestamp: pr.created_at.unwrap_or_default(),
            context: format!("PR #{}: {}", pr.number, pr.title.unwrap_or_default()),
        }
    }
}

#[async_trait]
impl GitHubCollaborationProvider for GitHubCollaborationAdapter {
    async fn fetch_code_reviews(
        &self,
        team_id: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<CodeReview>> {
        let prs = self
            .github_sdk
            .fetch_user_pull_requests(team_id, since, until)
            .await?;

        Ok(prs
            .into_iter()
            .map(|pr| self.convert_to_code_review(pr))
            .collect())
    }

    async fn fetch_team_interactions(
        &self,
        team_id: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<TeamInteraction>> {
        let prs = self
            .github_sdk
            .fetch_user_pull_requests(team_id, since, until)
            .await?;

        Ok(prs
            .into_iter()
            .map(|pr| self.convert_to_team_interaction(pr, team_id))
            .collect())
    }
}
