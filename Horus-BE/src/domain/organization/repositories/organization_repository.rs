use crate::common::types::PaginationParams;
use crate::domain::organization::{
    CreateOrganizationRequest, Organization, OrganizationResult, UpdateOrganizationRequest,
};
use async_trait::async_trait;

#[async_trait]
pub trait OrganizationRepository {
    /// Creates a new organization
    async fn create_organization(
        &self,
        dto: CreateOrganizationRequest,
    ) -> OrganizationResult<Organization>;

    /// Updates an existing organization
    async fn update_organization(
        &self,
        id: i32,
        dto: UpdateOrganizationRequest,
    ) -> OrganizationResult<Organization>;

    /// Deletes an organization
    async fn delete_organization(&self, id: i32) -> OrganizationResult<bool>;

    /// Batch creates multiple organizations
    async fn batch_create_organizations(
        &self,
        dtos: Vec<CreateOrganizationRequest>,
    ) -> OrganizationResult<Vec<Organization>>;

    /// Batch updates multiple organizations
    async fn batch_update_organizations(
        &self,
        updates: Vec<(i32, UpdateOrganizationRequest)>,
    ) -> OrganizationResult<Vec<Organization>>;

    /// Batch deletes multiple organizations
    async fn batch_delete_organizations(&self, ids: Vec<i32>) -> OrganizationResult<usize>;

    /// Finds organizations by name pattern
    async fn find_organizations_by_name(
        &self,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<Organization>, i64)>;

    /// Gets multiple organizations by their IDs
    async fn get_organizations_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> OrganizationResult<Vec<Organization>>;

    /// Checks if an organization exists
    async fn organization_exists(&self, id: i32) -> OrganizationResult<bool>;
}
