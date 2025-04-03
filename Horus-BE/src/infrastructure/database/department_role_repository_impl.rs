use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    common::types::PaginationParams,
    domain::roles::{
        AssignDepartmentRoleRequest, DepartmentRole, DepartmentRoleRepository, RoleError,
        RoleResult,
    },
    infrastructure::database::BaseRepository,
    infrastructure::db::DbPool,
    schema::departments_roles,
};

/// PostgreSQL implementation of the DepartmentRoleRepository trait
pub struct DepartmentRoleRepositoryImpl {
    pool: DbPool,
}

impl DepartmentRoleRepositoryImpl {
    /// Creates a new DepartmentRoleRepositoryImpl instance
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseRepository for DepartmentRoleRepositoryImpl {
    type Entity = DepartmentRole;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );
    type Error = RoleError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> RoleResult<Option<DepartmentRole>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        departments_roles::table
            .filter(departments_roles::id.eq(id))
            .first::<DepartmentRole>(conn)
            .await
            .optional()
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get department role by ID: {}", e))
            })
    }

    async fn find_all(&self) -> RoleResult<Vec<DepartmentRole>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        departments_roles::table
            .load::<DepartmentRole>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to list all department roles: {}", e))
            })
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<DepartmentRole>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get the department roles
        let department_roles = departments_roles::table
            .limit(params.per_page)
            .offset(offset)
            .load::<DepartmentRole>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to list department roles: {}", e))
            })?;

        // Get the total count
        let total = departments_roles::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to get total department role count: {}",
                    e
                ))
            })?;

        Ok((department_roles, total))
    }
}

#[async_trait]
impl DepartmentRoleRepository for DepartmentRoleRepositoryImpl {
    async fn assign_department_role(
        &self,
        organization_id: i32,
        request: AssignDepartmentRoleRequest,
    ) -> RoleResult<DepartmentRole> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Check if the role is already assigned
        let exists = self
            .user_has_department_role(
                organization_id,
                request.department_id,
                request.user_id,
                request.role_id,
            )
            .await?;

        if exists {
            return Err(RoleError::DepartmentRoleAlreadyAssigned(
                request.user_id,
                request.role_id,
                request.department_id,
            ));
        }

        // Create the department role assignment
        let department_role = DepartmentRole {
            id: 0, // Will be set by the database
            organization_id,
            department_id: request.department_id,
            user_id: request.user_id,
            role_id: request.role_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let result = diesel::insert_into(departments_roles::table)
            .values(&department_role)
            .returning(departments_roles::all_columns)
            .get_result::<DepartmentRole>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to assign department role: {}", e))
            })?;

        Ok(result)
    }

    async fn remove_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> RoleResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            departments_roles::table
                .filter(departments_roles::organization_id.eq(organization_id))
                .filter(departments_roles::department_id.eq(department_id))
                .filter(departments_roles::user_id.eq(user_id))
                .filter(departments_roles::role_id.eq(role_id)),
        )
        .execute(conn)
        .await
        .map_err(|e| {
            RoleError::DatabaseError(format!("Failed to remove department role: {}", e))
        })?;

        Ok(affected_rows > 0)
    }

    async fn get_user_department_roles(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> RoleResult<Vec<DepartmentRole>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::user_id.eq(user_id))
            .load::<DepartmentRole>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get user department roles: {}", e))
            })?;

        Ok(result)
    }

    async fn get_users_with_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        role_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<i32>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get the user IDs
        let user_ids = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .filter(departments_roles::role_id.eq(role_id))
            .select(departments_roles::user_id)
            .distinct()
            .limit(params.per_page)
            .offset(offset)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get users with department role: {}", e))
            })?;

        // Get the total count
        let total = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .filter(departments_roles::role_id.eq(role_id))
            .select(departments_roles::user_id)
            .distinct()
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to get total count of users with department role: {}",
                    e
                ))
            })?;

        Ok((user_ids, total))
    }

    async fn user_has_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> RoleResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .filter(departments_roles::user_id.eq(user_id))
            .filter(departments_roles::role_id.eq(role_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to check if user has department role: {}",
                    e
                ))
            })?;

        Ok(count > 0)
    }

    async fn get_user_department_role_ids(
        &self,
        organization_id: i32,
        user_id: i32,
        department_id: i32,
    ) -> RoleResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::user_id.eq(user_id))
            .filter(departments_roles::department_id.eq(department_id))
            .select(departments_roles::role_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get user department role IDs: {}", e))
            })?;

        Ok(result)
    }

    async fn get_users_with_any_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        role_ids: Vec<i32>,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<i32>, i64)> {
        if role_ids.is_empty() {
            return Ok((Vec::new(), 0));
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get the user IDs
        let user_ids = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .filter(departments_roles::role_id.eq_any(role_ids.clone()))
            .select(departments_roles::user_id)
            .distinct()
            .limit(params.per_page)
            .offset(offset)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to get users with any department role: {}",
                    e
                ))
            })?;

        // Get the total count
        let total = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .filter(departments_roles::role_id.eq_any(role_ids))
            .select(departments_roles::user_id)
            .distinct()
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to get total count of users with any department role: {}",
                    e
                ))
            })?;

        Ok((user_ids, total))
    }

    async fn get_department_roles(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<DepartmentRole>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get department roles
        let department_roles = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .limit(params.per_page)
            .offset(offset)
            .load::<DepartmentRole>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!("Failed to get department roles: {}", e))
            })?;

        // Get the total count
        let total = departments_roles::table
            .filter(departments_roles::organization_id.eq(organization_id))
            .filter(departments_roles::department_id.eq(department_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                RoleError::DatabaseError(format!(
                    "Failed to get total count of department roles: {}",
                    e
                ))
            })?;

        Ok((department_roles, total))
    }

    async fn batch_assign_department_roles(
        &self,
        organization_id: i32,
        requests: Vec<AssignDepartmentRoleRequest>,
    ) -> RoleResult<Vec<DepartmentRole>> {
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        // Process each assignment individually
        let mut assigned_roles = Vec::with_capacity(requests.len());

        for request in requests {
            // Skip if already assigned
            if self
                .user_has_department_role(
                    organization_id,
                    request.department_id,
                    request.user_id,
                    request.role_id,
                )
                .await?
            {
                continue;
            }

            // Assign the role
            let department_role = self
                .assign_department_role(organization_id, request)
                .await?;

            assigned_roles.push(department_role);
        }

        Ok(assigned_roles)
    }

    async fn batch_remove_department_roles(
        &self,
        organization_id: i32,
        department_id: i32,
        user_ids: Vec<i32>,
        role_id: i32,
    ) -> RoleResult<usize> {
        if user_ids.is_empty() {
            return Ok(0);
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            RoleError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            departments_roles::table
                .filter(departments_roles::organization_id.eq(organization_id))
                .filter(departments_roles::department_id.eq(department_id))
                .filter(departments_roles::user_id.eq_any(user_ids))
                .filter(departments_roles::role_id.eq(role_id)),
        )
        .execute(conn)
        .await
        .map_err(|e| {
            RoleError::DatabaseError(format!("Failed to batch remove department roles: {}", e))
        })?;

        Ok(affected_rows as usize)
    }
}
