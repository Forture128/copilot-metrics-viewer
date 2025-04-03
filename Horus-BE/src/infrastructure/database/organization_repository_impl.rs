use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Timestamp};
use diesel_async::RunQueryDsl;

use crate::domain::organization::{
    CreateOrganizationRequest, Organization, OrganizationError, OrganizationResult,
    UpdateOrganizationRequest,
};
use crate::{
    common::types::PaginationParams, domain::organization::repositories::OrganizationRepository,
    infrastructure::database::base_repository::BaseRepository, infrastructure::db::DbPool,
    schema::organizations,
};

/// Implementation of the organization repository
pub struct OrganizationRepositoryImpl {
    pool: DbPool,
}

impl OrganizationRepositoryImpl {
    /// Creates a new instance of the organization repository
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseRepository for OrganizationRepositoryImpl {
    type Entity = Organization;
    type SqlType = (Integer, Text, Timestamp, Timestamp);
    type Error = OrganizationError;
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> OrganizationResult<Option<Self::Entity>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let result = organizations::table
            .find(id)
            .first::<Self::Entity>(conn)
            .await
            .optional()
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(result)
    }

    async fn find_all(&self) -> OrganizationResult<Vec<Self::Entity>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let results = organizations::table
            .load::<Self::Entity>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(results)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<Self::Entity>, i64)> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let total = organizations::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        let results = organizations::table
            .offset(((params.page - 1) * params.per_page) as i64)
            .limit(params.per_page as i64)
            .load::<Self::Entity>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok((results, total))
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn create_organization(
        &self,
        dto: CreateOrganizationRequest,
    ) -> OrganizationResult<Organization> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let now = Utc::now().naive_utc();
        let result = diesel::insert_into(organizations::table)
            .values((
                organizations::name.eq(dto.name),
                organizations::created_at.eq(now),
                organizations::updated_at.eq(now),
            ))
            .get_result(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(result)
    }

    async fn update_organization(
        &self,
        id: i32,
        dto: UpdateOrganizationRequest,
    ) -> OrganizationResult<Organization> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let result = diesel::update(organizations::table.find(id))
            .set((
                organizations::name.eq(dto.name),
                organizations::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result(conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => OrganizationError::NotFound(id),
                _ => OrganizationError::DatabaseError(e.to_string()),
            })?;
        Ok(result)
    }

    async fn delete_organization(&self, id: i32) -> OrganizationResult<bool> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let count = diesel::delete(organizations::table.find(id))
            .execute(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(count > 0)
    }

    async fn batch_create_organizations(
        &self,
        dtos: Vec<CreateOrganizationRequest>,
    ) -> OrganizationResult<Vec<Organization>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let now = Utc::now().naive_utc();
        let values: Vec<_> = dtos
            .into_iter()
            .map(|dto| {
                (
                    organizations::name.eq(dto.name),
                    organizations::created_at.eq(now),
                    organizations::updated_at.eq(now),
                )
            })
            .collect();

        let results = diesel::insert_into(organizations::table)
            .values(&values)
            .get_results(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(results)
    }

    async fn batch_update_organizations(
        &self,
        updates: Vec<(i32, UpdateOrganizationRequest)>,
    ) -> OrganizationResult<Vec<Organization>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let now = Utc::now().naive_utc();
        let mut results = Vec::with_capacity(updates.len());

        for (id, dto) in updates {
            let result = diesel::update(organizations::table.find(id))
                .set((
                    organizations::name.eq(dto.name),
                    organizations::updated_at.eq(now),
                ))
                .get_result(conn)
                .await
                .map_err(|e| match e {
                    diesel::result::Error::NotFound => OrganizationError::NotFound(id),
                    _ => OrganizationError::DatabaseError(e.to_string()),
                })?;
            results.push(result);
        }
        Ok(results)
    }

    async fn batch_delete_organizations(&self, ids: Vec<i32>) -> OrganizationResult<usize> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let count = diesel::delete(organizations::table.filter(organizations::id.eq_any(ids)))
            .execute(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(count)
    }

    async fn find_organizations_by_name(
        &self,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> OrganizationResult<(Vec<Organization>, i64)> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let pattern = format!("%{}%", name_pattern);
        let total = organizations::table
            .filter(organizations::name.ilike(&pattern))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;

        let results = organizations::table
            .filter(organizations::name.ilike(&pattern))
            .offset(((params.page - 1) * params.per_page) as i64)
            .limit(params.per_page as i64)
            .load::<Organization>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;

        Ok((results, total))
    }

    async fn get_organizations_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> OrganizationResult<Vec<Organization>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let results = organizations::table
            .filter(organizations::id.eq_any(ids))
            .load::<Organization>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(results)
    }

    async fn organization_exists(&self, id: i32) -> OrganizationResult<bool> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            OrganizationError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;
        let count = organizations::table
            .filter(organizations::id.eq(id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| OrganizationError::DatabaseError(e.to_string()))?;
        Ok(count > 0)
    }
}
