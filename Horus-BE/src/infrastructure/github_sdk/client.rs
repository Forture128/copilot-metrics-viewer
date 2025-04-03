use super::builder::{GitHubConfig, GitHubSdkBuilder};
use super::models::{CustomDeployment, CustomDeploymentStatus};
use crate::common::errors::AppError;
use crate::common::utils::process_in_chunks;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use octocrab::models::repos::RepoCommit;
use octocrab::{params, Octocrab, OctocrabBuilder};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Main GitHub SDK struct that provides access to GitHub API operations
#[derive(Clone)]
pub struct GitHubSdk {
    client: Octocrab,
    pub config: GitHubConfig,
}

impl GitHubSdk {
    /// Creates a new GitHubSdk instance
    pub fn new(client: Octocrab, config: GitHubConfig) -> Self {
        Self { client, config }
    }

    /// Creates a new builder instance
    pub fn builder() -> GitHubSdkBuilder {
        GitHubSdkBuilder::new()
    }

    /// Initialize from environment variables:
    /// - GITHUB_TOKEN: GitHub Personal Access Token
    /// - GITHUB_API_URL: Optional GitHub Enterprise URL
    /// - GITHUB_TIMEOUT_SECS: Optional timeout in seconds (default: 30)
    pub fn from_env() -> Result<Self, AppError> {
        let config = GitHubConfig::from_env().map_err(|e| {
            error!("Failed to load GitHub config: {:?}", e);
            AppError::InternalError("Failed to load GitHub configuration".to_string())
        })?;

        if config.token.is_none() {
            warn!("GitHub token is not set. Some operations may fail.");
        }

        let builder = OctocrabBuilder::new();
        let builder = if let Some(token) = &config.token {
            debug!("Initializing GitHub client with token");
            builder.personal_token(token.clone())
        } else {
            builder
        };

        let client = builder.build().map_err(|e| {
            error!("Failed to create GitHub client: {:?}", e);
            AppError::GitHubError("Failed to initialize GitHub client".to_string())
        })?;

        Ok(Self { config, client })
    }

    /// Generic function to query any GitHub API endpoint
    pub async fn query_api<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let full_url = format!("{}{}", self.config.base_url.as_ref().unwrap(), endpoint);

        tracing::debug!("Querying GitHub API: {}", full_url);

        let response = self.client.get(&full_url, None::<&()>).await;

        match response {
            Ok(result) => Ok(result),
            Err(err) => {
                let error_msg = format!("GitHub API request failed: {}", err.to_string());
                error!("{}", error_msg);
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    /// Lists repositories for the authenticated user
    pub async fn list_user_repos(&self) -> Result<Vec<octocrab::models::Repository>> {
        tracing::debug!("Listing repositories for authenticated user");
        let result = self
            .client
            .current()
            .list_repos_for_authenticated_user()
            .send()
            .await
            .context("Failed to fetch user repositories")
            .map(|page| page.items);

        match &result {
            Ok(repos) => tracing::info!("Successfully fetched {} repositories", repos.len()),
            Err(e) => tracing::error!("Failed to fetch repositories: {:#?}", e),
        }

        result
    }

    /// Gets information about a specific repository
    pub async fn get_repo(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<octocrab::models::Repository> {
        let owner = owner.as_ref();
        let repo = repo.as_ref();
        tracing::debug!("Fetching repository {}/{}", owner, repo);

        let result = self
            .client
            .repos(owner, repo)
            .get()
            .await
            .context("Failed to fetch repository information");

        match &result {
            Ok(_) => tracing::info!("Successfully fetched repository {}/{}", owner, repo),
            Err(e) => tracing::error!("Failed to fetch repository {}/{}: {:#?}", owner, repo, e),
        }

        result
    }

    /// Lists issues for a repository
    pub async fn fetch_issues(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        state: Option<params::State>,
    ) -> Result<Vec<octocrab::models::issues::Issue>> {
        let state = state.unwrap_or(params::State::All);
        let result = self
            .client
            .issues(owner.as_ref(), repo.as_ref())
            .list()
            .state(state)
            .send()
            .await
            .context("Failed to fetch repository issues")
            .map(|page| page.items);

        match &result {
            Ok(issues) => tracing::info!("Successfully fetched {} issues", issues.len()),
            Err(e) => tracing::error!("Failed to fetch issues: {:#?}", e),
        }

        result
    }

    pub async fn fetch_repo_commits(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<Vec<RepoCommit>> {
        let result = self
            .client
            .repos(owner.as_ref(), repo.as_ref())
            .list_commits()
            .send()
            .await
            .context("Failed to fetch commits")
            .map(|page| page.items);

        match &result {
            Ok(commits) => tracing::info!("Successfully fetched {} commits", commits.len()),
            Err(e) => tracing::error!("Failed to fetch commits: {:#?}", e),
        }

        result
    }

    pub async fn fetch_user_commits(
        &self,
        username: impl AsRef<str>,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<RepoCommit>> {
        let username = username.as_ref();
        tracing::debug!("Fetching commits for user {}", username);

        let repos = self.list_user_repos().await?;
        let commits: Vec<RepoCommit> = process_in_chunks(&repos, 5, |repo| {
            let owner = repo
                .owner
                .as_ref()
                .map(|o| o.login.clone())
                .unwrap_or_default();
            let repo_name = repo.name.clone();
            let client = self.client.clone();

            async move {
                client
                    .repos(owner, repo_name)
                    .list_commits()
                    .since(since.unwrap_or_default())
                    .until(until.unwrap_or_default())
                    .per_page(100)
                    .send()
                    .await
            }
        })
        .await?;

        // Filter by date if specified
        let filtered_commits = if let Some(since_date) = since {
            commits
                .into_iter()
                .filter(|commit| {
                    commit
                        .commit
                        .author
                        .as_ref()
                        .and_then(|author| author.date)
                        .map_or(false, |date| date > since_date)
                })
                .collect()
        } else {
            commits
        };

        tracing::info!(
            "Total commits fetched for user {}: {}",
            username,
            filtered_commits.len()
        );
        Ok(filtered_commits)
    }

    /// Fetch pull requests for a repository
    pub async fn fetch_pull_requests(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<Vec<octocrab::models::pulls::PullRequest>> {
        let result = self
            .client
            .pulls(owner.as_ref(), repo.as_ref())
            .list()
            .send()
            .await
            .context("Failed to fetch pull requests")
            .map(|page| page.items);
        match &result {
            Ok(prs) => tracing::info!("Successfully fetched {} pull requests", prs.len()),
            Err(e) => tracing::error!("Failed to fetch pull requests: {:#?}", e),
        }
        result
    }

    /// Lists repositories the specified user contributes to
    pub async fn list_user_contributed_repos(
        &self,
        username: impl AsRef<str>,
    ) -> Result<Vec<octocrab::models::Repository>> {
        let username = username.as_ref();
        tracing::debug!("Listing repositories user {} contributes to", username);

        // First try to get repos the user has created or is a member of
        let user_repos = self
            .client
            .users(username)
            .repos()
            .send()
            .await
            .context("Failed to fetch user repositories")
            .map(|page| page.items)
            .unwrap_or_default();

        // Then get repos the user has contributed to via PRs (search API)
        let query = format!("author:{}", username);
        let pr_search_results = self
            .client
            .search()
            .issues_and_pull_requests(&query)
            .send()
            .await
            .context("Failed to search for PRs")
            .map(|page| page.items)
            .unwrap_or_default();

        // Extract unique repository information from search results
        let mut repo_urls = Vec::new();
        for item in &pr_search_results {
            let url_str = item.repository_url.to_string();
            if !repo_urls.contains(&url_str) {
                repo_urls.push(url_str);
            }
        }

        // Fetch full repository information for each unique repo
        let mut contributed_repos = Vec::new();
        for repo_url in repo_urls {
            // Parse owner and repo name from URL
            let parts: Vec<&str> = repo_url.split('/').collect();
            if parts.len() >= 2 {
                let repo_name = parts.last().unwrap_or(&"");
                let owner = parts.get(parts.len() - 2).unwrap_or(&"");

                if !repo_name.is_empty() && !owner.is_empty() {
                    if let Ok(repo) = self.client.repos(*owner, *repo_name).get().await {
                        contributed_repos.push(repo);
                    }
                }
            }
        }

        // Combine both lists and remove duplicates
        let mut all_repos = user_repos;
        for repo in contributed_repos {
            if !all_repos.iter().any(|r| r.id == repo.id) {
                all_repos.push(repo);
            }
        }

        tracing::info!(
            "Successfully fetched {} repositories for user {}",
            all_repos.len(),
            username
        );
        Ok(all_repos)
    }

    pub async fn fetch_user_pull_requests(
        &self,
        username: impl AsRef<str>,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<octocrab::models::pulls::PullRequest>> {
        let username = username.as_ref();
        // Use the contributed repos method instead of all repos
        let repos = self.list_user_contributed_repos(username).await?;
        println!("repos: {:?} of user {}", repos, username);
        let prs: Vec<octocrab::models::pulls::PullRequest> =
            process_in_chunks(&repos, 10, |repo| {
                let owner = repo
                    .owner
                    .as_ref()
                    .map(|o| o.login.clone())
                    .unwrap_or_default();
                let repo_name = repo.name.clone();
                let client = self.client.clone();

                async move {
                    client
                        .pulls(owner, repo_name)
                        .list()
                        .state(params::State::All) // Get all PRs including closed ones
                        .per_page(100)
                        .send()
                        .await
                }
            })
            .await?;

        // Filter by date range if specified
        let filtered_prs = prs
            .into_iter()
            .filter(|pr| {
                let created_at = pr.created_at.unwrap_or_default();
                match (since, until) {
                    (Some(since_date), Some(until_date)) => {
                        created_at >= since_date && created_at <= until_date
                    }
                    (Some(since_date), None) => created_at >= since_date,
                    (None, Some(until_date)) => created_at <= until_date,
                    (None, None) => true,
                }
            })
            .collect::<Vec<_>>();

        tracing::info!(
            "Total PRs fetched for user {}: {} (after time filtering)",
            username,
            filtered_prs.len()
        );
        Ok(filtered_prs)
    }

    /// List deployments for a repository
    pub async fn fetch_deployments(
        &self,
        owner: &str,
        repo: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<CustomDeployment>, AppError> {
        info!("Fetching deployments for {}/{}", owner, repo);

        let url = format!("/repos/{}/{}/deployments", owner, repo);
        println!("url: {}", url);

        let deployments = self
            .query_api::<Vec<CustomDeployment>>(&url)
            .await
            .map_err(|e| {
                AppError::GitHubError(format!(
                    "Failed to fetch deployments from {}/{}: {}",
                    owner, repo, e
                ))
            })?;

        if deployments.is_empty() {
            // Return empty vector
            return Ok(Vec::new());
        }

        let mut filtered_deployments = Vec::new();
        for deployment in deployments {
            let created_at = deployment.created_at;
            if let (Some(since), Some(until)) = (since, until) {
                if created_at >= since && created_at <= until {
                    filtered_deployments.push(CustomDeployment::from(deployment));
                }
            } else {
                filtered_deployments.push(CustomDeployment::from(deployment));
            }
        }

        debug!(
            "Found {} deployments for {}/{}",
            filtered_deployments.len(),
            owner,
            repo
        );
        Ok(filtered_deployments)
    }

    /// List releases for a repository
    pub async fn fetch_releases(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<octocrab::models::repos::Release>, AppError> {
        let owner = owner.as_ref();
        let repo = repo.as_ref();
        tracing::debug!("Fetching release for {}/{}", owner, repo);

        let result = self
            .client
            .repos(owner, repo)
            .releases()
            .list()
            .send()
            .await
            .context("Failed to fetch releases")
            .map(|page| page.items);

        match &result {
            Ok(releases) => {
                tracing::info!("Successfully fetched {} releases", releases.len());
                Ok(releases.to_vec())
            }
            Err(e) => {
                error!("Failed to fetch releases: {:#?}", e);
                return Err(AppError::GitHubError(format!(
                    "Failed to fetch releases: {}",
                    e
                )));
            }
        }
    }

    /// List deployment statuses for a deployment
    pub async fn fetch_deployment_statuses(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        deployment_id: i64,
    ) -> Result<Vec<CustomDeploymentStatus>, AppError> {
        let endpoint = format!(
            "{}/repos/{}/{}/deployments/{}/statuses",
            self.config.base_url.as_ref().unwrap(),
            owner.as_ref(),
            repo.as_ref(),
            deployment_id
        );
        let statuses = self
            .query_api::<Vec<CustomDeploymentStatus>>(&endpoint)
            .await
            .map_err(|e| {
                AppError::GitHubError(format!("Failed to fetch deployment statuses: {}", e))
            })?;
        Ok(statuses)
    }
}
