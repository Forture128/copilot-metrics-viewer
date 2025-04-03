use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    common::types::PaginationParams,
    domain::department::{
        CreateDepartmentRequest, Department, DepartmentError, DepartmentRepository,
        DepartmentResult, UpdateDepartmentRequest,
    },
    infrastructure::database::BaseRepository,
    infrastructure::db::DbPool,
    schema::departments,
};

/// PostgreSQL implementation of the DepartmentRepository trait
pub struct DepartmentRepositoryImpl {
    pool: DbPool,
}

impl DepartmentRepositoryImpl {
    /// Creates a new DepartmentRepositoryImpl instance
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DepartmentRepository for DepartmentRepositoryImpl {
    async fn create_department(
        &self,
        dto: CreateDepartmentRequest,
    ) -> DepartmentResult<Department> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();
        let new_department = Department {
            id: 0, // Will be set by the database
            organization_id: dto.organization_id,
            name: dto.name,
            created_at: now,
            updated_at: now,
        };

        let result = diesel::insert_into(departments::table)
            .values(&new_department)
            .returning(departments::all_columns)
            .get_result::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to create department: {}", e))
            })?;

        Ok(result)
    }

    async fn update_department(
        &self,
        id: i32,
        dto: UpdateDepartmentRequest,
    ) -> DepartmentResult<Department> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Get the current department
        let current_department = departments::table
            .find(id)
            .first::<Department>(conn)
            .await
            .optional()
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to find department: {}", e))
            })?
            .ok_or_else(|| DepartmentError::DepartmentNotFound(id))?;

        // Create an updated department with changes
        let mut updated_department = current_department;

        if let Some(name) = dto.name {
            updated_department.name = name;
        }

        updated_department.updated_at = Utc::now().naive_utc();

        // Update in the database
        let result = diesel::update(departments::table.find(id))
            .set(&updated_department)
            .returning(departments::all_columns)
            .get_result::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to update department: {}", e))
            })?;

        Ok(result)
    }

    async fn delete_department(&self, id: i32) -> DepartmentResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(departments::table.filter(departments::id.eq(id)))
            .execute(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to delete department: {}", e))
            })?;

        Ok(affected_rows > 0)
    }

    async fn get_department_by_id(&self, id: i32) -> DepartmentResult<Option<Department>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = departments::table
            .find(id)
            .first::<Department>(conn)
            .await
            .optional()
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get department by ID: {}", e))
            })?;

        Ok(result)
    }

    async fn department_exists(&self, id: i32) -> DepartmentResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = departments::table
            .filter(departments::id.eq(id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to check if department exists: {}",
                    e
                ))
            })?;

        Ok(count > 0)
    }

    async fn list_departments(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get departments with pagination
        let departments = departments::table
            .filter(departments::organization_id.eq(organization_id))
            .order(departments::name.asc())
            .limit(params.per_page)
            .offset(offset)
            .load::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to list departments: {}", e))
            })?;

        // Get total count
        let total = departments::table
            .filter(departments::organization_id.eq(organization_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to get total department count: {}",
                    e
                ))
            })?;

        Ok((departments, total))
    }

    async fn list_all_departments(
        &self,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let departments = departments::table
            .order(departments::name.asc())
            .limit(params.per_page)
            .offset(offset)
            .load::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to list all departments: {}", e))
            })?;

        let total = departments::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to get total department count: {}",
                    e
                ))
            })?;

        Ok((departments, total))
    }

    async fn find_departments_by_name(
        &self,
        organization_id: i32,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;
        let pattern = format!("%{}%", name_pattern);

        // Get departments matching the pattern
        let departments = departments::table
            .filter(departments::organization_id.eq(organization_id))
            .filter(departments::name.ilike(pattern.clone()))
            .order(departments::name.asc())
            .limit(params.per_page)
            .offset(offset)
            .load::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to find departments by name: {}", e))
            })?;

        // Get total count of matching departments
        let total = departments::table
            .filter(departments::organization_id.eq(organization_id))
            .filter(departments::name.ilike(pattern))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to get total count of matching departments: {}",
                    e
                ))
            })?;

        Ok((departments, total))
    }

    async fn get_departments_by_ids(&self, ids: Vec<i32>) -> DepartmentResult<Vec<Department>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let departments = departments::table
            .filter(departments::id.eq_any(ids))
            .load::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get departments by IDs: {}", e))
            })?;

        Ok(departments)
    }

    async fn department_name_exists(
        &self,
        organization_id: i32,
        name: &str,
    ) -> DepartmentResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = departments::table
            .filter(departments::organization_id.eq(organization_id))
            .filter(departments::name.eq(name))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to check if department name exists: {}",
                    e
                ))
            })?;

        Ok(count > 0)
    }
}

#[async_trait]
impl BaseRepository for DepartmentRepositoryImpl {
    type Entity = Department;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Text,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );
    type Error = DepartmentError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> DepartmentResult<Option<Department>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        departments::table
            .find(id)
            .first::<Department>(conn)
            .await
            .optional()
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to find department by ID: {}", e))
            })
    }

    async fn find_all(&self) -> DepartmentResult<Vec<Department>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        departments::table
            .load::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to find all departments: {}", e))
            })
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            DepartmentError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        // Get departments with pagination
        let departments = departments::table
            .limit(params.per_page)
            .offset(offset)
            .load::<Department>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to find departments: {}", e))
            })?;

        // Get total count
        let total = departments::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!(
                    "Failed to get total department count: {}",
                    e
                ))
            })?;

        Ok((departments, total))
    }
}
