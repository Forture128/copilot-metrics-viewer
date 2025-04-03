use crate::{
    common::errors::AppError,
    domain::delivery_insights::{
        entities::{Deployment, Incident, Release},
        repositories::DeliveryInsightsRepository,
        traits::{DeliveryAnalytics, GitHubDeliveryProvider},
        value_objects::{
            ChangeFailureRate, DeliveryMetrics, DeploymentFrequency, LeadTime, MeanTimeToRecover,
        },
    },
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct DeliveryInsightsAnalyzer<G: GitHubDeliveryProvider, R: DeliveryInsightsRepository> {
    github_provider: G,
    repository: R,
}

impl<G: GitHubDeliveryProvider, R: DeliveryInsightsRepository> DeliveryInsightsAnalyzer<G, R> {
    pub fn new(github_provider: G, repository: R) -> Self {
        Self {
            github_provider,
            repository,
        }
    }

    async fn fetch_and_store_deployments(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Deployment>> {
        let github_deployments = self
            .github_provider
            .list_deployments(repository, Some(start_date), Some(end_date))
            .await?;

        let mut deployments = Vec::new();
        for github_deployment in github_deployments {
            // Fetch deployment statuses
            let statuses = self
                .github_provider
                .list_deployment_statuses(repository, github_deployment.id)
                .await?;

            // Get the latest status
            let latest_status = statuses.first();
            let (status, completed_at) = if let Some(status) = latest_status {
                let deployment_status = match status.state {
                    crate::infrastructure::github_sdk::models::CustomDeploymentState::Success => {
                        super::super::entities::DeploymentStatus::Success
                    }
                    crate::infrastructure::github_sdk::models::CustomDeploymentState::Failure
                    | crate::infrastructure::github_sdk::models::CustomDeploymentState::Error => {
                        super::super::entities::DeploymentStatus::Failed
                    }
                    _ => super::super::entities::DeploymentStatus::InProgress,
                };
                (deployment_status, Some(status.created_at))
            } else {
                (super::super::entities::DeploymentStatus::InProgress, None)
            };

            let deployment = Deployment {
                id: github_deployment.id.to_string(),
                repository: repository.to_string(),
                environment: github_deployment.environment,
                version: github_deployment.reference,
                status,
                started_at: github_deployment.created_at,
                completed_at,
                deployed_by: github_deployment.creator.unwrap().login,
                changes: Vec::new(),
            };

            self.repository.save_deployment(&deployment).await?;
            deployments.push(deployment);
        }

        Ok(deployments)
    }

    async fn fetch_and_store_releases(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Release>> {
        let github_releases = self
            .github_provider
            .list_releases(repository, Some(start_date), Some(end_date))
            .await?;

        let mut releases = Vec::new();
        for github_release in github_releases {
            let release = Release {
                id: github_release.id.to_string(),
                version: github_release.tag_name,
                name: github_release.name.unwrap_or_default(),
                description: github_release.body.unwrap_or_default(),
                created_at: github_release.created_at.unwrap_or_default(),
                published_at: github_release.published_at,
                author: github_release.author.unwrap().login,
                deployments: Vec::new(),
                changes: Vec::new(),
            };

            self.repository.save_release(&release).await?;
            releases.push(release);
        }

        Ok(releases)
    }

    fn calculate_deployment_frequency(
        &self,
        deployments: &[Deployment],
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> DeploymentFrequency {
        let total_deployments = deployments.len();
        let days = (end_date - start_date).num_days() as f64;
        let weeks = days / 7.0;

        let deployments_per_day = if days > 0.0 {
            total_deployments as f64 / days
        } else {
            0.0
        };

        let deployments_per_week = if weeks > 0.0 {
            total_deployments as f64 / weeks
        } else {
            0.0
        };

        // Calculate environment-specific frequencies
        let mut environment_frequencies = Vec::new();
        let environments: std::collections::HashSet<String> =
            deployments.iter().map(|d| d.environment.clone()).collect();

        for env in environments {
            let env_deployments: Vec<&Deployment> = deployments
                .iter()
                .filter(|d| d.environment == *env)
                .collect();

            let success_count = env_deployments
                .iter()
                .filter(|d| d.status == super::super::entities::DeploymentStatus::Success)
                .count();

            let success_rate = if !env_deployments.is_empty() {
                success_count as f64 / env_deployments.len() as f64
            } else {
                0.0
            };

            let total_duration = env_deployments
                .iter()
                .filter_map(|d| d.completed_at.map(|c| c - d.started_at))
                .sum::<chrono::Duration>();

            let average_duration = if !env_deployments.is_empty() {
                total_duration / env_deployments.len() as i32
            } else {
                chrono::Duration::zero()
            };

            environment_frequencies.push(super::super::value_objects::EnvironmentFrequency {
                environment: env.clone(),
                deployment_count: env_deployments.len(),
                success_rate,
                average_duration,
            });
        }

        DeploymentFrequency {
            total_deployments,
            deployments_per_day,
            deployments_per_week,
            environment_frequencies,
        }
    }

    fn calculate_lead_time(&self, releases: &[Release]) -> LeadTime {
        let time_to_merge = chrono::Duration::zero();
        let mut time_to_deploy = chrono::Duration::zero();
        let mut total_count = 0;

        for release in releases {
            for deployment in &release.deployments {
                if let Some(completed_at) = deployment.completed_at {
                    time_to_deploy += completed_at - deployment.started_at;
                    total_count += 1;
                }
            }
        }

        let average_time_to_merge = if total_count > 0 {
            time_to_merge / total_count
        } else {
            chrono::Duration::zero()
        };

        let average_time_to_deploy = if total_count > 0 {
            time_to_deploy / total_count
        } else {
            chrono::Duration::zero()
        };

        LeadTime {
            average_time_to_merge,
            average_time_to_deploy,
            total_lead_time: average_time_to_merge + average_time_to_deploy,
            lead_time_percentiles: super::super::value_objects::LeadTimePercentiles {
                p50: chrono::Duration::zero(),
                p75: chrono::Duration::zero(),
                p90: chrono::Duration::zero(),
                p95: chrono::Duration::zero(),
            },
        }
    }

    fn calculate_change_failure_rate(&self, deployments: &[Deployment]) -> ChangeFailureRate {
        let total_changes = deployments.len();
        let failed_changes = deployments
            .iter()
            .filter(|d| d.status == super::super::entities::DeploymentStatus::Failed)
            .count();

        let failure_rate = if total_changes > 0 {
            failed_changes as f64 / total_changes as f64
        } else {
            0.0
        };

        ChangeFailureRate {
            total_changes,
            failed_changes,
            failure_rate,
            failure_categories: Vec::new(),
        }
    }

    fn calculate_mttr(&self, incidents: &[Incident]) -> MeanTimeToRecover {
        let mut total_recovery_time = chrono::Duration::zero();
        let mut recovery_time_trend = Vec::new();

        let resolved_incidents: Vec<_> = incidents
            .iter()
            .filter(|i| i.resolved_at.is_some() && i.resolution_time.is_some())
            .collect();

        for incident in &resolved_incidents {
            if let Some(resolution_time) = incident.resolution_time {
                total_recovery_time += resolution_time;

                recovery_time_trend.push(super::super::value_objects::RecoveryTimePoint {
                    timestamp: incident.started_at,
                    recovery_time: resolution_time,
                    incident_severity: format!("{:?}", incident.severity),
                });
            }
        }

        let average_recovery_time = if !resolved_incidents.is_empty() {
            total_recovery_time / resolved_incidents.len() as i32
        } else {
            chrono::Duration::zero()
        };

        MeanTimeToRecover {
            average_recovery_time,
            incidents_count: incidents.len(),
            recovery_time_trend,
        }
    }
}

#[async_trait]
impl<G: GitHubDeliveryProvider + Send + Sync, R: DeliveryInsightsRepository + Send + Sync>
    DeliveryAnalytics for DeliveryInsightsAnalyzer<G, R>
{
    async fn get_delivery_metrics(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<DeliveryMetrics, AppError> {
        // print debug info
        println!("[get_delivery_metrics] start_date: {:?}", start_date);
        println!("[get_delivery_metrics] end_date: {:?}", end_date);
        println!("[get_delivery_metrics] repository: {:?}", repository);
        let deployments = self
            .get_deployments(repository, start_date, end_date)
            .await?;
        println!("[get_delivery_metrics] deployments: {:?}", deployments);
        let releases = self.get_releases(repository, start_date, end_date).await?;
        println!("[get_delivery_metrics] releases: {:?}", releases);
        let incidents = self.get_incidents(repository, start_date, end_date).await?;
        println!("[get_delivery_metrics] incidents: {:?}", incidents);

        let deployment_frequency =
            self.calculate_deployment_frequency(&deployments, start_date, end_date);
        let lead_time = self.calculate_lead_time(&releases);
        let change_failure_rate = self.calculate_change_failure_rate(&deployments);
        let mean_time_to_recover = self.calculate_mttr(&incidents);

        Ok(DeliveryMetrics {
            repository: repository.to_string(),
            period_start: start_date,
            period_end: end_date,
            deployment_frequency,
            lead_time,
            change_failure_rate,
            mean_time_to_recover,
        })
    }

    // async fn get_delivery_metrics_concurrent(
    //     &self,
    //     repository: &str,
    //     start_date: DateTime<Utc>,
    //     end_date: DateTime<Utc>,
    // ) -> Result<DeliveryMetrics> {
    //     // Fetch all data concurrently using tokio::try_join!
    //     let (deployments, releases, incidents) = tokio::try_join!(
    //         self.get_deployments(repository, start_date, end_date),
    //         self.get_releases(repository, start_date, end_date),
    //         self.get_incidents(repository, start_date, end_date)
    //     )?;

    //     // Calculate metrics
    //     let deployment_frequency =
    //         self.calculate_deployment_frequency(&deployments, start_date, end_date);
    //     let lead_time = self.calculate_lead_time(&releases);
    //     let change_failure_rate = self.calculate_change_failure_rate(&deployments);
    //     let mean_time_to_recover = self.calculate_mttr(&incidents);

    //     Ok(DeliveryMetrics {
    //         repository: repository.to_string(),
    //         period_start: start_date,
    //         period_end: end_date,
    //         deployment_frequency,
    //         lead_time,
    //         change_failure_rate,
    //         mean_time_to_recover,
    //     })
    // }

    async fn get_deployments(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Deployment>, AppError> {
        // First try to get from repository
        let stored_deployments = self
            .repository
            .get_deployments(repository, start_date, end_date)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !stored_deployments.is_empty() {
            return Ok(stored_deployments);
        }

        // If no stored deployments, fetch from GitHub and store
        self.fetch_and_store_deployments(repository, start_date, end_date)
            .await
            .map_err(|e| AppError::GitHubError(e.to_string()))
    }

    async fn get_releases(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Release>, AppError> {
        // First try to get from repository
        let stored_releases = self
            .repository
            .get_releases(repository, start_date, end_date)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !stored_releases.is_empty() {
            return Ok(stored_releases);
        }

        // If no stored releases, fetch from GitHub and store
        self.fetch_and_store_releases(repository, start_date, end_date)
            .await
            .map_err(|e| AppError::GitHubError(e.to_string()))
    }

    async fn get_incidents(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Incident>, AppError> {
        // For now, just get from repository as incidents might be tracked in a different system
        self.repository
            .get_incidents(repository, start_date, end_date)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::delivery_insights::entities::{Deployment, DeploymentStatus};
    use crate::domain::delivery_insights::repositories::MockDeliveryInsightsRepository;
    use crate::domain::delivery_insights::traits::MockGitHubDeliveryProvider;
    use chrono::{Duration, Utc};

    #[tokio::test]
    async fn test_calculate_deployment_frequency() {
        let service = DeliveryInsightsAnalyzer::new(
            MockGitHubDeliveryProvider::default(),
            MockDeliveryInsightsRepository::default(),
        );

        let start_date = Utc::now() - Duration::days(7);
        let end_date = Utc::now();

        let deployments = vec![Deployment {
            id: "1".to_string(),
            repository: "test-repo".to_string(),
            environment: "production".to_string(),
            version: "v1.0".to_string(),
            status: DeploymentStatus::Success,
            started_at: start_date,
            completed_at: Some(end_date),
            deployed_by: "user".to_string(),
            changes: Vec::new(),
        }];

        let frequency = service.calculate_deployment_frequency(&deployments, start_date, end_date);

        assert_eq!(frequency.total_deployments, 1);
        assert!(frequency.deployments_per_day > 0.0);
    }
}
