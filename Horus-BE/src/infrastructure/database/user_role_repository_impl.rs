use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    common::types::PaginationParams,
    domain::user::{
        repositories::user_role_repository::UserRoleRepository, UserError, UserResult, UserRole,
    },
    infrastructure::{database::base_repository::BaseRepository, db::DbPool},
    schema::user_roles,
};

/// Implementation of the user role repository
pub struct UserRoleRepositoryImpl {
    pool: DbPool,
}

impl UserRoleRepositoryImpl {
    /// Creates a new instance of the user role repository
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseRepository for UserRoleRepositoryImpl {
    type Entity = UserRole;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );
    type Error = UserError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Self::Entity>, Self::Error> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = user_roles::table
            .find(id)
            .first::<Self::Entity>(conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn find_all(&self) -> Result<Vec<Self::Entity>, Self::Error> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let results = user_roles::table
            .load::<Self::Entity>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> Result<(Vec<Self::Entity>, i64), Self::Error> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let page = params.page;
        let per_page = params.per_page;
        let offset = (page - 1) * per_page;

        let results = user_roles::table
            .limit(per_page)
            .offset(offset)
            .load::<Self::Entity>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total = user_roles::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((results, total))
    }
}

#[async_trait]
impl UserRoleRepository for UserRoleRepositoryImpl {
    async fn assign_role(
        &self,
        organization_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> UserResult<UserRole> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Check if the role is already assigned
        let existing = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::user_id.eq(user_id))
            .filter(user_roles::role_id.eq(role_id))
            .first::<UserRole>(conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        if let Some(user_role) = existing {
            return Ok(user_role);
        }

        let now = Utc::now().naive_utc();
        let new_user_role = UserRole {
            id: 0, // Auto-incremented by DB
            organization_id,
            user_id,
            role_id,
            created_at: now,
            updated_at: now,
        };

        let inserted_role = diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .get_result::<UserRole>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(inserted_role)
    }

    async fn remove_role(
        &self,
        organization_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> UserResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let deleted_count = diesel::delete(
            user_roles::table
                .filter(user_roles::organization_id.eq(organization_id))
                .filter(user_roles::user_id.eq(user_id))
                .filter(user_roles::role_id.eq(role_id)),
        )
        .execute(conn)
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(deleted_count > 0)
    }

    async fn get_user_roles(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> UserResult<Vec<UserRole>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let user_roles = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::user_id.eq(user_id))
            .load::<UserRole>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(user_roles)
    }

    async fn get_users_with_role(
        &self,
        organization_id: i32,
        role_id: i32,
        params: &PaginationParams,
    ) -> UserResult<(Vec<i32>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let page = params.page;
        let per_page = params.per_page;
        let offset = (page - 1) * per_page;

        let user_ids: Vec<i32> = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::role_id.eq(role_id))
            .select(user_roles::user_id)
            .limit(per_page)
            .offset(offset)
            .load(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total: i64 = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::role_id.eq(role_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((user_ids, total))
    }

    async fn user_has_role(
        &self,
        organization_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> UserResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::user_id.eq(user_id))
            .filter(user_roles::role_id.eq(role_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(count > 0)
    }

    async fn get_user_role_ids(&self, organization_id: i32, user_id: i32) -> UserResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let role_ids: Vec<i32> = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::user_id.eq(user_id))
            .select(user_roles::role_id)
            .load(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(role_ids)
    }

    async fn batch_assign_roles(
        &self,
        organization_id: i32,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> UserResult<Vec<UserRole>> {
        if role_ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Check which roles are already assigned
        let existing_role_ids: Vec<i32> = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::user_id.eq(user_id))
            .filter(user_roles::role_id.eq_any(&role_ids))
            .select(user_roles::role_id)
            .load(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        // Filter out roles that are already assigned
        let new_role_ids: Vec<i32> = role_ids
            .into_iter()
            .filter(|id| !existing_role_ids.contains(id))
            .collect();

        if new_role_ids.is_empty() {
            // All roles are already assigned, get the existing ones
            let existing_roles = user_roles::table
                .filter(user_roles::organization_id.eq(organization_id))
                .filter(user_roles::user_id.eq(user_id))
                .filter(user_roles::role_id.eq_any(&existing_role_ids))
                .load::<UserRole>(conn)
                .await
                .map_err(|e| UserError::DatabaseError(e.to_string()))?;

            return Ok(existing_roles);
        }

        // Create the new user role assignments
        let now = Utc::now().naive_utc();
        let new_user_roles: Vec<_> = new_role_ids
            .into_iter()
            .map(|role_id| UserRole {
                id: 0,
                organization_id,
                user_id,
                role_id,
                created_at: now,
                updated_at: now,
            })
            .collect();

        let inserted_roles = diesel::insert_into(user_roles::table)
            .values(&new_user_roles)
            .get_results::<UserRole>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        // Fetch all assigned roles (both existing and newly assigned)
        let all_roles = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::user_id.eq(user_id))
            .filter(
                user_roles::role_id.eq_any(
                    existing_role_ids
                        .iter()
                        .chain(inserted_roles.iter().map(|r| &r.role_id))
                        .cloned()
                        .collect::<Vec<i32>>(),
                ),
            )
            .load::<UserRole>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(all_roles)
    }

    async fn batch_remove_roles(
        &self,
        organization_id: i32,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> UserResult<usize> {
        if role_ids.is_empty() {
            return Ok(0);
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let deleted_count = diesel::delete(
            user_roles::table
                .filter(user_roles::organization_id.eq(organization_id))
                .filter(user_roles::user_id.eq(user_id))
                .filter(user_roles::role_id.eq_any(role_ids)),
        )
        .execute(conn)
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(deleted_count)
    }

    async fn get_users_with_any_role(
        &self,
        organization_id: i32,
        role_ids: Vec<i32>,
        params: &PaginationParams,
    ) -> UserResult<(Vec<i32>, i64)> {
        if role_ids.is_empty() {
            return Ok((Vec::new(), 0));
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let user_ids: Vec<i32> = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::role_id.eq_any(&role_ids))
            .select(user_roles::user_id)
            .distinct()
            .limit(params.per_page)
            .offset(offset)
            .load(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total: i64 = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::role_id.eq_any(&role_ids))
            .select(user_roles::user_id)
            .distinct()
            .count()
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((user_ids, total))
    }

    async fn get_users_with_all_roles(
        &self,
        organization_id: i32,
        role_ids: Vec<i32>,
        params: &PaginationParams,
    ) -> UserResult<(Vec<i32>, i64)> {
        if role_ids.is_empty() {
            return Ok((Vec::new(), 0));
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let page = params.page;
        let per_page = params.per_page;
        let offset = (page - 1) * per_page;

        // Find users who have all the specified roles
        // We group by user_id and count role assignments, then filter where count equals the number of roles
        let role_count = role_ids.len() as i64;

        let user_ids: Vec<i32> = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::role_id.eq_any(&role_ids))
            .group_by(user_roles::user_id)
            .having(diesel::dsl::count(user_roles::role_id).eq(role_count))
            .select(user_roles::user_id)
            .limit(per_page)
            .offset(offset)
            .load(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total: i64 = user_roles::table
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(user_roles::role_id.eq_any(&role_ids))
            .group_by(user_roles::user_id)
            .having(diesel::dsl::count(user_roles::role_id).eq(role_count))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((user_ids, total))
    }
}
