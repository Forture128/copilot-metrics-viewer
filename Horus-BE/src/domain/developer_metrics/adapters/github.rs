use crate::domain::developer_metrics::traits::GitHubDataProvider;
use crate::domain::developer_metrics::{Commit, Developer, PullRequest};
use crate::infrastructure::github_sdk::client::GitHubSdk;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Adapter for fetching developer data from GitHub
pub struct GitHubDeveloperAdapter {
    github_sdk: GitHubSdk,
}

impl GitHubDeveloperAdapter {
    pub fn new(github_sdk: GitHubSdk) -> Self {
        Self { github_sdk }
    }

    fn convert_to_commit(&self, github_commit: octocrab::models::repos::RepoCommit) -> Commit {
        let author = github_commit.author.unwrap();
        Commit {
            id: github_commit.sha,
            author: Developer {
                id: author.id.to_string(),
                username: author.login.to_string(),
                email: author.email.unwrap_or_default(),
                name: None,
            },
            message: github_commit.commit.message,
            timestamp: github_commit.commit.author.unwrap().date.unwrap(),
            additions: 0,
            deletions: 0,
            files_changed: 0,
        }
    }

    fn convert_to_pull_request(
        &self,
        github_pr: octocrab::models::pulls::PullRequest,
    ) -> PullRequest {
        PullRequest {
            id: github_pr.id.to_string(),
            number: github_pr.number as i32,
            title: github_pr.title.unwrap_or_default(),
            author: Developer {
                id: github_pr.user.as_ref().unwrap().id.to_string(),
                username: github_pr.user.as_ref().unwrap().login.clone(),
                email: github_pr
                    .user
                    .as_ref()
                    .unwrap()
                    .email
                    .clone()
                    .unwrap_or_default(),
                name: None,
            },
            created_at: github_pr.created_at.unwrap(),
            merged_at: github_pr.merged_at,
            review_time: None,
            commits: Vec::new(),
            comments: github_pr.comments.unwrap_or(0) as usize,
            changed_files: github_pr.changed_files.unwrap_or(0) as usize,
            additions: github_pr.additions.unwrap_or(0) as usize,
            deletions: github_pr.deletions.unwrap_or(0) as usize,
        }
    }
}

#[async_trait]
impl GitHubDataProvider for GitHubDeveloperAdapter {
    async fn fetch_user_commits(
        &self,
        username: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<Commit>> {
        let github_commits = self
            .github_sdk
            .fetch_user_commits(username, since, until)
            .await?;

        Ok(github_commits
            .into_iter()
            .map(|commit| self.convert_to_commit(commit))
            .collect())
    }

    async fn fetch_user_pull_requests(
        &self,
        username: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<PullRequest>> {
        let github_prs = self
            .github_sdk
            .fetch_user_pull_requests(username, since, until)
            .await?;

        Ok(github_prs
            .into_iter()
            .map(|pr| self.convert_to_pull_request(pr))
            .collect())
    }
}
