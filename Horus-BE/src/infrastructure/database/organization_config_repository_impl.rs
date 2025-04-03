use crate::domain::organization::{
    CreateOrganizationConfigRequest, OrganizationConfig, OrganizationConfigRepository,
    OrganizationError, OrganizationResult, UpdateOrganizationConfigRequest,
};
use crate::{
    common::types::PaginationParams, infrastructure::db::DbPool, schema::organization_configs,
};
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel_async::RunQueryDsl;

pub struct OrganizationConfigRepositoryImpl {
    pool: DbPool,
}

impl OrganizationConfigRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationConfigRepository for OrganizationConfigRepositoryImpl {
    async fn create_config(
        &self,
        org_id: i32,
        config: CreateOrganizationConfigRequest,
    ) -> OrganizationResult<OrganizationConfig> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let now = Utc::now().naive_utc();
        let result = diesel::insert_into(organization_configs::table)
            .values((
                organization_configs::organization_id.eq(org_id),
                organization_configs::config_key.eq(config.config_key),
                organization_configs::config_value.eq(config.config_value),
                organization_configs::created_at.eq(now),
                organization_configs::updated_at.eq(now),
            ))
            .get_result(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(result)
    }

    async fn update_config(
        &self,
        org_id: i32,
        config_id: i32,
        config: UpdateOrganizationConfigRequest,
    ) -> OrganizationResult<OrganizationConfig> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let result = diesel::update(
            organization_configs::table
                .filter(organization_configs::organization_id.eq(org_id))
                .find(config_id),
        )
        .set((
            organization_configs::config_value.eq(config.config_value),
            organization_configs::updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result(conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                OrganizationError::ConfigNotFound { org_id, config_id }
            }
            _ => OrganizationError::DatabaseError(e.to_string()),
        })?;
        Ok(result)
    }

    async fn delete_config(&self, org_id: i32, config_id: i32) -> OrganizationResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let count = diesel::delete(
            organization_configs::table
                .filter(organization_configs::organization_id.eq(org_id))
                .find(config_id),
        )
        .execute(conn)
        .await
        .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(count > 0)
    }

    async fn get_config(
        &self,
        org_id: i32,
        config_id: i32,
    ) -> OrganizationResult<OrganizationConfig> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let result = organization_configs::table
            .filter(organization_configs::organization_id.eq(org_id))
            .find(config_id)
            .first(conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    OrganizationError::ConfigNotFound { org_id, config_id }
                }
                _ => OrganizationError::DatabaseError(e.to_string()),
            })?;
        Ok(result)
    }

    async fn list_configs(
        &self,
        org_id: i32,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<OrganizationConfig>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let query =
            organization_configs::table.filter(organization_configs::organization_id.eq(org_id));

        let total = query
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;

        let results = query
            .offset(((params.page - 1) * params.per_page) as i64)
            .limit(params.per_page as i64)
            .load(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;

        Ok((results, total))
    }

    async fn batch_create_configs(
        &self,
        org_id: i32,
        configs: Vec<CreateOrganizationConfigRequest>,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let now = Utc::now().naive_utc();
        let values: Vec<_> = configs
            .into_iter()
            .map(|config| {
                (
                    organization_configs::organization_id.eq(org_id),
                    organization_configs::config_key.eq(config.config_key),
                    organization_configs::config_value.eq(config.config_value),
                    organization_configs::created_at.eq(now),
                    organization_configs::updated_at.eq(now),
                )
            })
            .collect();

        let results = diesel::insert_into(organization_configs::table)
            .values(&values)
            .get_results(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(results)
    }

    async fn batch_update_configs(
        &self,
        org_id: i32,
        configs: Vec<(i32, UpdateOrganizationConfigRequest)>,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let now = Utc::now().naive_utc();
        let mut results = Vec::with_capacity(configs.len());

        for (config_id, config) in configs {
            let result = diesel::update(
                organization_configs::table
                    .filter(organization_configs::organization_id.eq(org_id))
                    .find(config_id),
            )
            .set((
                organization_configs::config_value.eq(config.config_value),
                organization_configs::updated_at.eq(now),
            ))
            .get_result(conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    OrganizationError::ConfigNotFound { org_id, config_id }
                }
                _ => OrganizationError::DatabaseError(e.to_string()),
            })?;
            results.push(result);
        }
        Ok(results)
    }

    async fn batch_delete_configs(
        &self,
        org_id: i32,
        config_ids: Vec<i32>,
    ) -> OrganizationResult<usize> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let count = diesel::delete(
            organization_configs::table
                .filter(organization_configs::organization_id.eq(org_id))
                .filter(organization_configs::id.eq_any(config_ids)),
        )
        .execute(conn)
        .await
        .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(count)
    }

    async fn find_configs_by_key(
        &self,
        org_id: i32,
        key_pattern: &str,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let pattern = format!("%{}%", key_pattern);
        let results = organization_configs::table
            .filter(organization_configs::organization_id.eq(org_id))
            .filter(organization_configs::config_key.like(pattern))
            .load(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(results)
    }

    async fn get_configs_by_ids(
        &self,
        org_id: i32,
        config_ids: Vec<i32>,
    ) -> OrganizationResult<Vec<OrganizationConfig>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let results = organization_configs::table
            .filter(organization_configs::organization_id.eq(org_id))
            .filter(organization_configs::id.eq_any(config_ids))
            .load(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(results)
    }
}
