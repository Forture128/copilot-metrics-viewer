use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

use crate::{
    common::types::PaginationParams,
    domain::department::{
        DepartmentError, DepartmentResult, DepartmentUser, DepartmentUserRepository,
    },
    infrastructure::database::BaseRepository,
    infrastructure::db::DbPool,
    schema::department_users,
};

/// PostgreSQL implementation of the DepartmentUserRepository trait
pub struct DepartmentUserRepositoryImpl {
    pool: DbPool,
}

impl DepartmentUserRepositoryImpl {
    /// Creates a new DepartmentUserRepositoryImpl instance
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DepartmentUserRepository for DepartmentUserRepositoryImpl {
    async fn add_user_to_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<DepartmentUser> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Check if the association already exists
        let exists = self
            .is_user_in_department(organization_id, department_id, user_id)
            .await?;

        if exists {
            return Err(DepartmentError::ValidationError(format!(
                "User {} is already in department {}",
                user_id, department_id
            )));
        }

        // Create the department user association
        let new_department_user = DepartmentUser {
            id: 0, // Will be set by the database
            organization_id,
            department_id,
            user_id,
            joined_at: Utc::now().naive_utc(),
        };

        let result = diesel::insert_into(department_users::table)
            .values(&new_department_user)
            .returning(department_users::all_columns)
            .get_result::<DepartmentUser>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to add user to department: {}", e))
            })?;

        Ok(result)
    }

    async fn remove_user_from_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            department_users::table
                .filter(department_users::organization_id.eq(organization_id))
                .filter(department_users::department_id.eq(department_id))
                .filter(department_users::user_id.eq(user_id)),
        )
        .execute(conn)
        .await
        .map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to remove user from department: {}", e))
        })?;

        Ok(affected_rows > 0)
    }

    async fn get_department_users(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<DepartmentUser>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get users in the department with pagination
        let users = department_users::table
            .filter(department_users::organization_id.eq(organization_id))
            .filter(department_users::department_id.eq(department_id))
            .limit(params.per_page)
            .offset(offset)
            .load::<DepartmentUser>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get department users: {}", e))
            })?;

        // Get total count
        let total = department_users::table
            .filter(department_users::organization_id.eq(organization_id))
            .filter(department_users::department_id.eq(department_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to get total department user count: {}",
                    e
                ))
            })?;

        Ok((users, total))
    }

    async fn get_user_department_ids(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> DepartmentResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let department_ids = department_users::table
            .filter(department_users::organization_id.eq(organization_id))
            .filter(department_users::user_id.eq(user_id))
            .select(department_users::department_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get user department IDs: {}", e))
            })?;

        Ok(department_ids)
    }

    async fn is_user_in_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = department_users::table
            .filter(department_users::organization_id.eq(organization_id))
            .filter(department_users::department_id.eq(department_id))
            .filter(department_users::user_id.eq(user_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to check if user is in department: {}",
                    e
                ))
            })?;

        Ok(count > 0)
    }

    async fn batch_add_users_to_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_ids: Vec<i32>,
    ) -> DepartmentResult<Vec<DepartmentUser>> {
        if user_ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Filter out users already in the department
        let mut added_users = Vec::new();
        let now = Utc::now().naive_utc();

        for user_id in user_ids {
            // Skip if already in department
            if self
                .is_user_in_department(organization_id, department_id, user_id)
                .await?
            {
                continue;
            }

            // Add the user to the department
            let new_department_user = DepartmentUser {
                id: 0, // Will be set by the database
                organization_id,
                department_id,
                user_id,
                joined_at: now,
            };

            let result = diesel::insert_into(department_users::table)
                .values(&new_department_user)
                .returning(department_users::all_columns)
                .get_result::<DepartmentUser>(conn)
                .await
                .map_err(|e| {
                    DepartmentError::DatabaseError(format!(
                        "Failed to add user to department in batch: {}",
                        e
                    ))
                })?;

            added_users.push(result);
        }

        Ok(added_users)
    }

    async fn batch_remove_users_from_department(
        &self,
        organization_id: i32,
        department_id: i32,
        user_ids: Vec<i32>,
    ) -> DepartmentResult<usize> {
        if user_ids.is_empty() {
            return Ok(0);
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            department_users::table
                .filter(department_users::organization_id.eq(organization_id))
                .filter(department_users::department_id.eq(department_id))
                .filter(department_users::user_id.eq_any(user_ids)),
        )
        .execute(conn)
        .await
        .map_err(|e| {
            DepartmentError::DatabaseError(format!(
                "Failed to batch remove users from department: {}",
                e
            ))
        })?;

        Ok(affected_rows as usize)
    }

    async fn get_department_user_ids(
        &self,
        organization_id: i32,
        department_id: i32,
    ) -> DepartmentResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let user_ids = department_users::table
            .filter(department_users::organization_id.eq(organization_id))
            .filter(department_users::department_id.eq(department_id))
            .select(department_users::user_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get department user IDs: {}", e))
            })?;

        Ok(user_ids)
    }

    async fn count_department_users(
        &self,
        organization_id: i32,
        department_id: i32,
    ) -> DepartmentResult<i64> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count = department_users::table
            .filter(department_users::organization_id.eq(organization_id))
            .filter(department_users::department_id.eq(department_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to count department users: {}", e))
            })?;

        Ok(count)
    }
}
