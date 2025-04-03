use super::entities::{Deployment, DeploymentChange, Incident, Release};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DeliveryInsightsRepository: Send + Sync {
    // Deployments
    async fn save_deployment(&self, deployment: &Deployment) -> Result<()>;
    async fn get_deployments(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Deployment>>;

    #[allow(dead_code)]
    async fn get_deployment_by_id(&self, id: &str) -> Result<Option<Deployment>>;

    // Releases
    async fn save_release(&self, release: &Release) -> Result<()>;
    async fn get_releases(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Release>>;

    #[allow(dead_code)]
    async fn get_release_by_version(&self, version: &str) -> Result<Option<Release>>;

    // Incidents
    #[allow(dead_code)]
    async fn save_incident(&self, incident: &Incident) -> Result<()>;
    async fn get_incidents(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Incident>>;

    #[allow(dead_code)]
    async fn get_incident_by_id(&self, id: &str) -> Result<Option<Incident>>;

    // Deployment Changes
    #[allow(dead_code)]
    async fn save_deployment_changes(
        &self,
        deployment_id: &str,
        changes: &[DeploymentChange],
    ) -> Result<()>;
    #[allow(dead_code)]
    async fn get_deployment_changes(&self, deployment_id: &str) -> Result<Vec<DeploymentChange>>;

    // Metrics
    #[allow(dead_code)]
    async fn save_deployment_frequency(
        &self,
        repository: &str,
        metrics: &super::value_objects::DeploymentFrequency,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<()>;

    #[allow(dead_code)]
    async fn save_lead_time(
        &self,
        repository: &str,
        metrics: &super::value_objects::LeadTime,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<()>;

    #[allow(dead_code)]
    async fn save_change_failure_rate(
        &self,
        repository: &str,
        metrics: &super::value_objects::ChangeFailureRate,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<()>;

    #[allow(dead_code)]
    async fn save_mttr(
        &self,
        repository: &str,
        metrics: &super::value_objects::MeanTimeToRecover,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<()>;

    #[allow(dead_code)]
    async fn get_deployment_frequency(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::DeploymentFrequency>>;

    #[allow(dead_code)]
    async fn get_lead_time(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::LeadTime>>;

    #[allow(dead_code)]
    async fn get_change_failure_rate(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::ChangeFailureRate>>;

    #[allow(dead_code)]
    async fn get_mttr(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::MeanTimeToRecover>>;
}

pub struct InMemoryDeliveryInsightsRepository;

#[async_trait]
impl DeliveryInsightsRepository for InMemoryDeliveryInsightsRepository {
    async fn save_deployment(&self, _deployment: &Deployment) -> Result<()> {
        Ok(())
    }

    async fn get_deployments(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<Deployment>> {
        Ok(vec![])
    }

    async fn get_deployment_by_id(&self, _id: &str) -> Result<Option<Deployment>> {
        Ok(None)
    }

    async fn save_release(&self, _release: &Release) -> Result<()> {
        Ok(())
    }

    async fn get_releases(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<Release>> {
        Ok(vec![])
    }

    async fn get_release_by_version(&self, _version: &str) -> Result<Option<Release>> {
        Ok(None)
    }

    async fn save_incident(&self, _incident: &Incident) -> Result<()> {
        Ok(())
    }

    async fn get_incidents(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<Incident>> {
        Ok(vec![])
    }

    async fn get_incident_by_id(&self, _id: &str) -> Result<Option<Incident>> {
        Ok(None)
    }

    async fn save_deployment_changes(
        &self,
        _deployment_id: &str,
        _changes: &[DeploymentChange],
    ) -> Result<()> {
        Ok(())
    }

    async fn get_deployment_changes(&self, _deployment_id: &str) -> Result<Vec<DeploymentChange>> {
        Ok(vec![])
    }

    async fn save_deployment_frequency(
        &self,
        _repository: &str,
        _metrics: &super::value_objects::DeploymentFrequency,
        _period_start: DateTime<Utc>,
        _period_end: DateTime<Utc>,
    ) -> Result<()> {
        Ok(())
    }

    async fn save_lead_time(
        &self,
        _repository: &str,
        _metrics: &super::value_objects::LeadTime,
        _period_start: DateTime<Utc>,
        _period_end: DateTime<Utc>,
    ) -> Result<()> {
        Ok(())
    }

    async fn save_change_failure_rate(
        &self,
        _repository: &str,
        _metrics: &super::value_objects::ChangeFailureRate,
        _period_start: DateTime<Utc>,
        _period_end: DateTime<Utc>,
    ) -> Result<()> {
        Ok(())
    }

    async fn save_mttr(
        &self,
        _repository: &str,
        _metrics: &super::value_objects::MeanTimeToRecover,
        _period_start: DateTime<Utc>,
        _period_end: DateTime<Utc>,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_deployment_frequency(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::DeploymentFrequency>> {
        Ok(vec![])
    }

    async fn get_lead_time(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::LeadTime>> {
        Ok(vec![])
    }

    async fn get_change_failure_rate(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::ChangeFailureRate>> {
        Ok(vec![])
    }

    async fn get_mttr(
        &self,
        _repository: &str,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<super::value_objects::MeanTimeToRecover>> {
        Ok(vec![])
    }
}
