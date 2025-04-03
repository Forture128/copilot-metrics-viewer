use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::Utc;

use crate::common::types::PaginationParams;
use crate::domain::department::repositories::DepartmentRepository;
use crate::domain::department::{
    AssignUserToDepartmentRequest, BatchAssignUsersToDepartmentRequest, CreateDepartmentRequest,
    Department, DepartmentError, DepartmentResponse, DepartmentResult, UpdateDepartmentRequest,
};
use crate::domain::roles::{
    AssignDepartmentRoleRequest, DepartmentRole, DepartmentRoleRepository, RoleRepository,
};
use crate::domain::user::UserRepository;
use crate::infrastructure::database::BaseRepository;
use crate::infrastructure::redis::Redis;

/// Service for managing departments
pub struct DepartmentService<R, UR, DR, RR>
where
    R: DepartmentRepository + BaseRepository<Entity = Department>,
    UR: UserRepository,
    DR: DepartmentRoleRepository,
    RR: RoleRepository,
{
    repository: R,
    user_repository: UR,
    department_role_repository: DR,
    role_repository: RR,
    redis: Redis,
    cache: Arc<RwLock<HashMap<i32, Department>>>,
}

impl<R, UR, DR, RR> DepartmentService<R, UR, DR, RR>
where
    R: DepartmentRepository + BaseRepository<Entity = Department>,
    UR: UserRepository,
    DR: DepartmentRoleRepository,
    RR: RoleRepository,
{
    /// Creates a new DepartmentService instance
    pub fn new(
        repository: R,
        user_repository: UR,
        department_role_repository: DR,
        role_repository: RR,
        redis: Redis,
    ) -> Self {
        Self {
            repository,
            user_repository,
            department_role_repository,
            role_repository,
            redis,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new department
    pub async fn create_department(
        &self,
        request: CreateDepartmentRequest,
    ) -> DepartmentResult<Department> {
        // Check if department name already exists
        if self
            .repository
            .department_name_exists(request.organization_id, &request.name)
            .await?
        {
            return Err(DepartmentError::DepartmentAlreadyExists(request.name));
        }

        // Create the department
        let department = self.repository.create_department(request).await?;

        // Update cache
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(department.id, department.clone());
        }

        Ok(department)
    }

    /// Updates an existing department
    pub async fn update_department(
        &self,
        id: i32,
        request: UpdateDepartmentRequest,
    ) -> DepartmentResult<Department> {
        // Check if department exists
        let current = self.get_department(id).await?;
        let current = current.ok_or_else(|| DepartmentError::DepartmentNotFound(id))?;

        // If name is being updated, check if it already exists
        if let Some(name) = &request.name {
            if name != &current.name
                && self
                    .repository
                    .department_name_exists(current.organization_id, name)
                    .await?
            {
                return Err(DepartmentError::DepartmentAlreadyExists(name.clone()));
            }
        }

        // Update the department
        let updated = self.repository.update_department(id, request).await?;

        // Update cache
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(updated.id, updated.clone());
        }

        Ok(updated)
    }

    /// Deletes a department
    pub async fn delete_department(&self, id: i32) -> DepartmentResult<bool> {
        // Check if department exists
        let department = self.get_department(id).await?;
        let department = department.ok_or_else(|| DepartmentError::DepartmentNotFound(id))?;

        // Delete the department
        let result = self.repository.delete_department(id).await?;

        // Remove from cache if deleted
        if result {
            if let Ok(mut cache) = self.cache.write() {
                cache.remove(&id);
            }
        }

        Ok(result)
    }

    /// Gets a department by ID
    pub async fn get_department(&self, id: i32) -> DepartmentResult<Option<Department>> {
        // Try to get from cache first
        if let Ok(cache) = self.cache.read() {
            if let Some(department) = cache.get(&id) {
                return Ok(Some(department.clone()));
            }
        }

        // Not in cache, get from database
        let department = self.repository.get_department_by_id(id).await?;

        // Update cache if found
        if let Some(department) = &department {
            if let Ok(mut cache) = self.cache.write() {
                cache.insert(department.id, department.clone());
            }
        }

        Ok(department)
    }

    /// Lists all departments for an organization
    pub async fn list_departments(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)> {
        self.repository
            .list_departments(organization_id, params)
            .await
    }

    /// Assigns a user to a department with a specific role
    pub async fn add_user_to_department(
        &self,
        department_id: i32,
        request: AssignUserToDepartmentRequest,
        default_role_id: i32, // Default role ID to assign
    ) -> DepartmentResult<DepartmentRole> {
        // Check if department exists
        let department = self.get_department(department_id).await?;
        let department =
            department.ok_or_else(|| DepartmentError::DepartmentNotFound(department_id))?;

        // Check if user exists
        let user_exists = self
            .user_repository
            .get_user_by_id(request.user_id)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to check if user exists: {}", e))
            })?;

        if user_exists.is_none() {
            return Err(DepartmentError::UserNotFound(request.user_id));
        }

        // Check if role exists
        let role_exists = self
            .role_repository
            .role_exists(default_role_id)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to check if role exists: {}", e))
            })?;

        if !role_exists {
            return Err(DepartmentError::InternalError(format!(
                "Role with ID {} not found",
                default_role_id
            )));
        }

        // Check if user already has the role in this department
        let has_role = self
            .department_role_repository
            .user_has_department_role(
                department.organization_id,
                department_id,
                request.user_id,
                default_role_id,
            )
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to check if user has role: {}", e))
            })?;

        if has_role {
            return Err(DepartmentError::DepartmentDataError(format!(
                "User {} already has role {} in department {}",
                request.user_id, default_role_id, department_id
            )));
        }

        // Assign the role to the user in the department
        let assign_request = AssignDepartmentRoleRequest {
            department_id,
            user_id: request.user_id,
            role_id: default_role_id,
        };

        self.department_role_repository
            .assign_department_role(department.organization_id, assign_request)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to assign role to user: {}", e))
            })
    }

    /// Removes a user from a department (removes all their roles in the department)
    pub async fn remove_user_from_department(
        &self,
        department_id: i32,
        user_id: i32,
    ) -> DepartmentResult<bool> {
        // Check if department exists
        let department = self.get_department(department_id).await?;
        let department =
            department.ok_or_else(|| DepartmentError::DepartmentNotFound(department_id))?;

        // Get user's roles in the department
        let user_roles = self
            .department_role_repository
            .get_user_department_role_ids(department.organization_id, user_id, department_id)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get user roles: {}", e))
            })?;

        if user_roles.is_empty() {
            // User has no roles in this department
            return Ok(false);
        }

        // Remove all roles for the user in this department
        let mut success = true;
        for role_id in user_roles {
            let result = self
                .department_role_repository
                .remove_department_role(department.organization_id, department_id, user_id, role_id)
                .await
                .map_err(|e| {
                    DepartmentError::DatabaseError(format!("Failed to remove role: {}", e))
                })?;

            success = success && result;
        }

        Ok(success)
    }

    /// Gets all users in a department with their roles
    pub async fn get_department_users(
        &self,
        department_id: i32,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<i32>, i64)> {
        // Check if department exists
        let department = self.get_department(department_id).await?;
        let department =
            department.ok_or_else(|| DepartmentError::DepartmentNotFound(department_id))?;

        // Get all users with any role in the department (we don't filter by role)
        self.department_role_repository
            .get_users_with_any_department_role(
                department.organization_id,
                department_id,
                vec![], // Empty vec means all roles
                params,
            )
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to get department users: {}", e))
            })
    }

    /// Batch adds users to a department with a default role
    pub async fn batch_add_users_to_department(
        &self,
        department_id: i32,
        request: BatchAssignUsersToDepartmentRequest,
        default_role_id: i32,
    ) -> DepartmentResult<Vec<DepartmentRole>> {
        // Check if department exists
        let department = self.get_department(department_id).await?;
        let department =
            department.ok_or_else(|| DepartmentError::DepartmentNotFound(department_id))?;

        // Create assign requests for each user
        let assign_requests = request
            .user_ids
            .into_iter()
            .map(|user_id| AssignDepartmentRoleRequest {
                department_id,
                user_id,
                role_id: default_role_id,
            })
            .collect();

        // Batch assign roles
        self.department_role_repository
            .batch_assign_department_roles(department.organization_id, assign_requests)
            .await
            .map_err(|e| {
                DepartmentError::DatabaseError(format!("Failed to batch assign roles: {}", e))
            })
    }

    /// Helper to convert Department to DepartmentResponse
    pub fn to_department_response(&self, department: &Department) -> DepartmentResponse {
        DepartmentResponse {
            id: department.id,
            organization_id: department.organization_id,
            name: department.name.clone(),
            created_at: department.created_at,
            updated_at: department.updated_at,
        }
    }

    /// Lists all departments for an organization
    pub async fn list_all_departments(
        &self,
        params: &PaginationParams,
    ) -> DepartmentResult<(Vec<Department>, i64)> {
        let departments = self.repository.list_all_departments(params).await?;
        Ok(departments)
    }
}
