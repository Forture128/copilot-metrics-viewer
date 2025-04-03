use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::organization::entities::{Organization, OrganizationConfig};

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationConfigResponse {
    pub id: i32,
    pub organization_id: i32,
    pub config_key: String,
    pub config_value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationListResponse {
    pub organizations: Vec<OrganizationResponse>,
    pub total: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationConfigListResponse {
    pub configs: Vec<OrganizationConfigResponse>,
    pub total: usize,
}

// Conversion implementations
impl From<Organization> for OrganizationResponse {
    fn from(org: Organization) -> Self {
        Self {
            id: org.id,
            name: org.name,
            created_at: org.created_at,
            updated_at: org.updated_at,
        }
    }
}

impl From<OrganizationConfig> for OrganizationConfigResponse {
    fn from(config: OrganizationConfig) -> Self {
        Self {
            id: config.id,
            organization_id: config.organization_id,
            config_key: config.config_key,
            config_value: config.config_value,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}
