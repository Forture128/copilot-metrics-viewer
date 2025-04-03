use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    common::types::PaginationParams,
    domain::roles::{
        CreateRoleRequest, Role, RoleError, RoleRepository, RoleResult, UpdateRoleRequest,
    },
    infrastructure::database::BaseRepository,
    infrastructure::db::DbPool,
    schema::{departments_roles, roles, user_roles},
};

/// PostgreSQL implementation of the RoleRepository trait
pub struct RoleRepositoryImpl {
    pool: DbPool,
}

impl RoleRepositoryImpl {
    /// Creates a new RoleRepositoryImpl instance
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for RoleRepositoryImpl {
    async fn create_role(&self, dto: CreateRoleRequest) -> RoleResult<Role> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();

        let new_role = Role {
            id: 0, // Will be set by the database
            organization_id: dto.organization_id,
            name: dto.name,
            description: dto.description,
            created_at: now,
            updated_at: now,
        };

        let result = diesel::insert_into(roles::table)
            .values(&new_role)
            .returning(roles::all_columns)
            .get_result::<Role>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to create role: {}", e)))?;

        Ok(result)
    }

    async fn update_role(&self, id: i32, dto: UpdateRoleRequest) -> RoleResult<Role> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Get the existing role first
        let mut role = self
            .get_role_by_id(id)
            .await?
            .ok_or_else(|| RoleError::RoleNotFound(id))?;

        // Update the fields that were provided
        if let Some(name) = dto.name {
            role.name = name;
        }

        if let Some(description) = dto.description {
            role.description = Some(description);
        }

        // Set the updated timestamp
        role.updated_at = Utc::now().naive_utc();

        // Perform the update
        let updated_role = diesel::update(roles::table.find(id))
            .set(&role)
            .get_result::<Role>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to update role: {}", e)))?;

        Ok(updated_role)
    }

    async fn delete_role(&self, id: i32) -> RoleResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(roles::table.find(id))
            .execute(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to delete role: {}", e)))?;

        Ok(affected_rows > 0)
    }

    async fn get_role_by_id(&self, id: i32) -> RoleResult<Option<Role>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = roles::table
            .find(id)
            .first::<Role>(conn)
            .await
            .optional()
            .map_err(|e| RoleError::DatabaseError(format!("Failed to get role by ID: {}", e)))?;

        Ok(result)
    }

    async fn role_exists(&self, id: i32) -> RoleResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to check if role exists: {}", e))
        })?;

        let count: i64 = roles::table
            .filter(roles::id.eq(id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to check if role exists: {}", e))
            })?;

        Ok(count > 0)
    }

    async fn list_roles(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<Role>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get the roles
        let roles = roles::table
            .filter(roles::organization_id.eq(organization_id))
            .order(roles::name.asc())
            .limit(params.per_page as i64)
            .offset(offset)
            .load::<Role>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to list roles: {}", e)))?;

        // Get the total count
        let total = roles::table
            .filter(roles::organization_id.eq(organization_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get total role count: {}", e))
            })?;

        Ok((roles, total))
    }

    async fn find_roles_by_name(
        &self,
        organization_id: i32,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<Role>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;
        let pattern = format!("%{}%", name_pattern);

        // Get the roles
        let roles = roles::table
            .filter(roles::organization_id.eq(organization_id))
            .filter(roles::name.ilike(pattern.clone()))
            .order(roles::name.asc())
            .limit(params.per_page)
            .offset(offset)
            .load::<Role>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to find roles by name: {}", e))
            })?;

        // Get the total count
        let total = roles::table
            .filter(roles::organization_id.eq(organization_id))
            .filter(roles::name.ilike(pattern))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to get total role count for search: {}",
                    e
                ))
            })?;

        Ok((roles, total))
    }

    async fn get_roles_by_ids(&self, ids: Vec<i32>) -> RoleResult<Vec<Role>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let roles = roles::table
            .filter(roles::id.eq_any(ids))
            .order(roles::name.asc())
            .load::<Role>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to get roles by IDs: {}", e)))?;

        Ok(roles)
    }

    async fn role_name_exists(&self, organization_id: i32, name: &str) -> RoleResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = roles::table
            .filter(roles::organization_id.eq(organization_id))
            .filter(roles::name.eq(name))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to check if role name exists: {}", e))
            })?;

        Ok(count > 0)
    }

    async fn batch_create_roles(
        &self,
        organization_id: i32,
        roles_to_create: Vec<CreateRoleRequest>,
    ) -> RoleResult<Vec<Role>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();

        // Create roles individually (transaction is handled by diesel-async)
        let mut created_roles = Vec::with_capacity(roles_to_create.len());

        for dto in roles_to_create {
            let new_role = Role {
                id: 0, // Will be set by the database
                organization_id,
                name: dto.name,
                description: dto.description,
                created_at: now,
                updated_at: now,
            };

            let role = diesel::insert_into(roles::table)
                .values(&new_role)
                .returning(roles::all_columns)
                .get_result::<Role>(conn)
                .await
                .map_err(|e| {
                    RoleError::DatabaseError(format!("Failed to batch create role: {}", e))
                })?;

            created_roles.push(role);
        }

        Ok(created_roles)
    }

    async fn get_roles_by_user_id(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> RoleResult<Vec<Role>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Get roles directly assigned to the user
        let direct_roles = roles::table
            .inner_join(
                user_roles::table.on(roles::id
                    .eq(user_roles::role_id)
                    .and(user_roles::user_id.eq(user_id))
                    .and(user_roles::organization_id.eq(organization_id))),
            )
            .select(roles::all_columns)
            .load::<Role>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get user direct roles: {}", e))
            })?;

        // Get roles assigned to the user through departments
        let department_roles = roles::table
            .inner_join(
                departments_roles::table.on(roles::id
                    .eq(departments_roles::role_id)
                    .and(departments_roles::user_id.eq(user_id))
                    .and(departments_roles::organization_id.eq(organization_id))),
            )
            .select(roles::all_columns)
            .load::<Role>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get user department roles: {}", e))
            })?;

        // Combine and deduplicate roles
        let mut all_roles = direct_roles;
        all_roles.extend(department_roles);
        all_roles.sort_by(|a, b| a.id.cmp(&b.id));
        all_roles.dedup_by(|a, b| a.id == b.id);

        Ok(all_roles)
    }
}

#[async_trait]
impl BaseRepository for RoleRepositoryImpl {
    type Entity = Role;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Text,
        diesel::sql_types::Nullable<diesel::sql_types::Text>,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );
    type Error = RoleError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Role>, RoleError> {
        self.get_role_by_id(id)
            .await
            .map_err(|e| RoleError::DatabaseError(e.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<Role>, RoleError> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let roles = roles::table
            .load::<Role>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to list roles: {}", e)))?;

        Ok(roles)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> Result<(Vec<Role>, i64), RoleError> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let total = roles::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to count roles: {}", e)))?;

        let roles = roles::table
            .order(roles::name.asc())
            .offset(offset)
            .limit(params.per_page)
            .load::<Role>(conn)
            .await
            .map_err(|e| RoleError::DatabaseError(format!("Failed to list roles: {}", e)))?;

        Ok((roles, total))
    }
}
