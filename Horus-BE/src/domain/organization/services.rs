use chrono::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::entities::Organization;
use super::{
    repositories::{OrganizationConfigRepository, OrganizationRepository},
    CreateOrganizationConfigRequest, CreateOrganizationRequest, OrganizationConfig,
    OrganizationError, OrganizationResult, UpdateOrganizationConfigRequest,
    UpdateOrganizationRequest,
};
use crate::common::types::PaginationParams;
use crate::infrastructure::{database::BaseRepository, redis::Redis};

/// Service for managing organizations
pub struct OrganizationService<R, C>
where
    R: OrganizationRepository + BaseRepository<Entity = Organization, Error = OrganizationError>,
    C: OrganizationConfigRepository,
{
    repository: R,
    config_repository: C,
    redis: Redis,
    cache: Arc<RwLock<HashMap<i32, Organization>>>,
}

impl<R, C> OrganizationService<R, C>
where
    R: OrganizationRepository
        + for<'a> BaseRepository<Entity = Organization, Error = OrganizationError>,
    C: OrganizationConfigRepository,
{
    /// Creates a new instance of the organization service
    pub fn new(repository: R, config_repository: C, redis: Redis) -> Self {
        Self {
            repository,
            config_repository,
            redis,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new organization
    pub async fn create_organization(
        &self,
        org: CreateOrganizationRequest,
    ) -> OrganizationResult<Organization> {
        // Validate organization name
        if org.name.is_empty() {
            return Err(OrganizationError::ValidationError(
                "Organization name cannot be empty".into(),
            ));
        }

        let org = self.repository.create_organization(org).await?;

        // Update cache
        self.redis
            .set_ex(
                &format!("org:{}", org.id),
                &org,
                Duration::hours(1).num_seconds() as u64,
            )
            .await
            .map_err(OrganizationError::RedisError)?;
        self.cache.write().await.insert(org.id, org.clone());

        Ok(org)
    }

    /// Updates an existing organization
    pub async fn update_organization(
        &self,
        id: i32,
        org: UpdateOrganizationRequest,
    ) -> OrganizationResult<Organization> {
        // Validate organization name
        if org.name.is_empty() {
            return Err(OrganizationError::ValidationError(
                "Organization name cannot be empty".into(),
            ));
        }

        let org = self.repository.update_organization(id, org).await?;

        // Update cache
        self.redis
            .set_ex(
                &format!("org:{}", org.id),
                &org,
                Duration::hours(1).num_seconds() as u64,
            )
            .await
            .map_err(OrganizationError::RedisError)?;
        self.cache.write().await.insert(org.id, org.clone());

        Ok(org)
    }

    /// Deletes an organization
    pub async fn delete_organization(&self, id: i32) -> OrganizationResult<bool> {
        let result = self.repository.delete_organization(id).await?;
        if result {
            // Remove from cache
            self.redis
                .del(&format!("org:{}", id))
                .await
                .map_err(OrganizationError::RedisError)?;
            self.cache.write().await.remove(&id);
        }
        Ok(result)
    }

    /// Retrieves an organization by ID
    pub async fn get_organization(&self, id: i32) -> OrganizationResult<Option<Organization>> {
        // Check in-memory cache first
        if let Some(org) = self.cache.read().await.get(&id) {
            return Ok(Some(org.clone()));
        }

        // Check Redis cache
        if let Some(org) = self
            .redis
            .get::<Organization>(&format!("org:{}", id))
            .await
            .ok()
            .flatten()
        {
            // Update in-memory cache
            self.cache.write().await.insert(id, org.clone());
            return Ok(Some(org));
        }

        // Query database
        let org = match self.repository.find_by_id(id).await {
            Ok(org) => org,
            Err(e) => return Err(OrganizationError::DatabaseError(e.to_string())),
        };

        match org {
            Some(org) => {
                // Update both caches
                self.redis
                    .set_ex(
                        &format!("org:{}", id),
                        &org,
                        Duration::hours(1).num_seconds() as u64,
                    )
                    .await?;
                self.cache.write().await.insert(id, org.clone());
                Ok(Some(org))
            }
            None => Ok(None),
        }
    }

    /// Lists all organizations with pagination
    pub async fn list_organizations(
        &self,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<Organization>, i64)> {
        match self.repository.find_with_pagination(params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(OrganizationError::DatabaseError(e.to_string())),
        }
    }

    /// Batch creates multiple organizations
    pub async fn batch_create_organizations(
        &self,
        requests: Vec<CreateOrganizationRequest>,
    ) -> OrganizationResult<Vec<Organization>> {
        // Validate all requests first
        for request in &requests {
            if request.name.is_empty() {
                return Err(OrganizationError::ValidationError(
                    "Organization name cannot be empty".into(),
                ));
            }
        }

        let organizations = self.repository.batch_create_organizations(requests).await?;

        // Update cache for each new organization
        for org in &organizations {
            self.redis
                .set_ex(
                    &format!("org:{}", org.id),
                    org,
                    Duration::hours(1).num_seconds() as u64,
                )
                .await?;
            self.cache.write().await.insert(org.id, org.clone());
        }

        Ok(organizations)
    }

    /// Batch updates multiple organizations
    pub async fn batch_update_organizations(
        &self,
        updates: Vec<(i32, UpdateOrganizationRequest)>,
    ) -> OrganizationResult<Vec<Organization>> {
        // Validate all updates first
        for (_, update) in &updates {
            if update.name.is_empty() {
                return Err(OrganizationError::ValidationError(
                    "Organization name cannot be empty".into(),
                ));
            }
        }

        let organizations = self.repository.batch_update_organizations(updates).await?;

        // Update cache for each updated organization
        for org in &organizations {
            self.redis
                .set_ex(
                    &format!("org:{}", org.id),
                    org,
                    Duration::hours(1).num_seconds() as u64,
                )
                .await?;
            self.cache.write().await.insert(org.id, org.clone());
        }

        Ok(organizations)
    }

    /// Batch deletes multiple organizations
    pub async fn batch_delete_organizations(&self, ids: Vec<i32>) -> OrganizationResult<usize> {
        let count = self
            .repository
            .batch_delete_organizations(ids.clone())
            .await?;

        // Remove from cache
        for id in ids {
            self.redis.del(&format!("org:{}", id)).await?;
            self.cache.write().await.remove(&id);
        }

        Ok(count)
    }

    /// Finds organizations by name pattern
    pub async fn find_organizations_by_name(
        &self,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<Organization>, i64)> {
        self.repository
            .find_organizations_by_name(name_pattern, params)
            .await
    }

    /// Gets multiple organizations by their IDs
    pub async fn get_organizations_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> OrganizationResult<Vec<Organization>> {
        self.repository.get_organizations_by_ids(ids).await
    }

    /// Checks if an organization exists
    pub async fn organization_exists(&self, id: i32) -> OrganizationResult<bool> {
        self.repository.organization_exists(id).await
    }

    // Config-related methods
    pub async fn create_config(
        &self,
        org_id: i32,
        config: CreateOrganizationConfigRequest,
    ) -> OrganizationResult<OrganizationConfig> {
        self.config_repository.create_config(org_id, config).await
    }

    pub async fn update_config(
        &self,
        org_id: i32,
        config_id: i32,
        config: UpdateOrganizationConfigRequest,
    ) -> OrganizationResult<OrganizationConfig> {
        self.config_repository
            .update_config(org_id, config_id, config)
            .await
    }

    pub async fn delete_config(&self, org_id: i32, config_id: i32) -> OrganizationResult<bool> {
        self.config_repository
            .delete_config(org_id, config_id)
            .await
    }

    pub async fn get_config(
        &self,
        org_id: i32,
        config_id: i32,
    ) -> OrganizationResult<OrganizationConfig> {
        self.config_repository.get_config(org_id, config_id).await
    }

    pub async fn list_configs(
        &self,
        org_id: i32,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<OrganizationConfig>, i64)> {
        self.config_repository.list_configs(org_id, params).await
    }

    pub async fn batch_create_configs(
        &self,
        org_id: i32,
        configs: Vec<CreateOrganizationConfigRequest>,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        // Ensure organization exists
        if !self.repository.organization_exists(org_id).await? {
            return Err(OrganizationError::NotFound(org_id));
        }

        // Validate all configs first
        for config in &configs {
            if config.config_key.is_empty() || config.config_value.is_empty() {
                return Err(OrganizationError::ValidationError(
                    "Config key and value cannot be empty".into(),
                ));
            }
        }

        self.config_repository
            .batch_create_configs(org_id, configs)
            .await
    }

    pub async fn search_configs(
        &self,
        org_id: i32,
        key_pattern: &str,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        self.config_repository
            .find_configs_by_key(org_id, key_pattern)
            .await
    }

    pub async fn batch_delete_configs(
        &self,
        org_id: i32,
        config_ids: Vec<i32>,
    ) -> OrganizationResult<usize> {
        self.config_repository
            .batch_delete_configs(org_id, config_ids)
            .await
    }

    pub async fn batch_update_configs(
        &self,
        org_id: i32,
        configs: Vec<(i32, UpdateOrganizationConfigRequest)>,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        self.config_repository
            .batch_update_configs(org_id, configs)
            .await
    }

    pub async fn get_configs_by_ids(
        &self,
        org_id: i32,
        config_ids: Vec<i32>,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        self.config_repository
            .get_configs_by_ids(org_id, config_ids)
            .await
    }

    pub async fn find_configs_by_key(
        &self,
        org_id: i32,
        key_pattern: &str,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        self.config_repository
            .find_configs_by_key(org_id, key_pattern)
            .await
    }
}
