use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeploymentFrequency {
    pub total_deployments: usize,
    pub deployments_per_day: f64,
    pub deployments_per_week: f64,
    pub environment_frequencies: Vec<EnvironmentFrequency>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EnvironmentFrequency {
    pub environment: String,
    pub deployment_count: usize,
    pub success_rate: f64,
    pub average_duration: Duration,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LeadTime {
    pub average_time_to_merge: Duration,
    pub average_time_to_deploy: Duration,
    pub total_lead_time: Duration,
    pub lead_time_percentiles: LeadTimePercentiles,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LeadTimePercentiles {
    pub p50: Duration,
    pub p75: Duration,
    pub p90: Duration,
    pub p95: Duration,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangeFailureRate {
    pub total_changes: usize,
    pub failed_changes: usize,
    pub failure_rate: f64,
    pub failure_categories: Vec<FailureCategory>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FailureCategory {
    pub category: String,
    pub count: usize,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MeanTimeToRecover {
    pub average_recovery_time: Duration,
    pub incidents_count: usize,
    pub recovery_time_trend: Vec<RecoveryTimePoint>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecoveryTimePoint {
    pub timestamp: DateTime<Utc>,
    pub recovery_time: Duration,
    pub incident_severity: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeliveryMetrics {
    pub repository: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub deployment_frequency: DeploymentFrequency,
    pub lead_time: LeadTime,
    pub change_failure_rate: ChangeFailureRate,
    pub mean_time_to_recover: MeanTimeToRecover,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeploymentPipeline {
    pub stages: Vec<PipelineStage>,
    pub average_duration: Duration,
    pub success_rate: f64,
    pub bottleneck_stage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PipelineStage {
    pub name: String,
    pub average_duration: Duration,
    pub success_rate: f64,
    pub failure_reasons: Vec<String>,
}
