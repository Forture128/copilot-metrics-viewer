use crate::common::types::PaginationParams;
use crate::domain::organization::{
    CreateOrganizationConfigRequest, OrganizationConfig, OrganizationResult,
    UpdateOrganizationConfigRequest,
};
use async_trait::async_trait;

#[async_trait]
pub trait OrganizationConfigRepository {
    /// Creates a new configuration for an organization
    async fn create_config(
        &self,
        org_id: i32,
        config: CreateOrganizationConfigRequest,
    ) -> OrganizationResult<OrganizationConfig>;

    /// Updates an existing configuration
    async fn update_config(
        &self,
        org_id: i32,
        config_id: i32,
        config: UpdateOrganizationConfigRequest,
    ) -> OrganizationResult<OrganizationConfig>;

    /// Deletes a configuration
    async fn delete_config(&self, org_id: i32, config_id: i32) -> OrganizationResult<bool>;

    /// Retrieves a configuration by ID
    async fn get_config(
        &self,
        org_id: i32,
        config_id: i32,
    ) -> OrganizationResult<OrganizationConfig>;

    /// Lists all configurations for an organization with pagination
    async fn list_configs(
        &self,
        org_id: i32,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<OrganizationConfig>, i64)>;

    /// Batch creates multiple configurations
    async fn batch_create_configs(
        &self,
        org_id: i32,
        configs: Vec<CreateOrganizationConfigRequest>,
    ) -> OrganizationResult<Vec<OrganizationConfig>>;

    /// Batch updates multiple configurations
    async fn batch_update_configs(
        &self,
        org_id: i32,
        configs: Vec<(i32, UpdateOrganizationConfigRequest)>,
    ) -> OrganizationResult<Vec<OrganizationConfig>>;

    /// Batch deletes multiple configurations
    async fn batch_delete_configs(
        &self,
        org_id: i32,
        config_ids: Vec<i32>,
    ) -> OrganizationResult<usize>;

    /// Finds configurations by key pattern
    async fn find_configs_by_key(
        &self,
        org_id: i32,
        key_pattern: &str,
    ) -> OrganizationResult<Vec<OrganizationConfig>>;

    /// Gets multiple configurations by their IDs
    async fn get_configs_by_ids(
        &self,
        org_id: i32,
        config_ids: Vec<i32>,
    ) -> OrganizationResult<Vec<OrganizationConfig>>;
}
