use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::types::PaginationParams;
use crate::domain::roles::{
    AssignDepartmentRoleRequest, BatchCreateRolesRequest, CreateRoleRequest, DepartmentRole,
    DepartmentRoleRepository, Role, RoleError, RoleRepository, RoleResult, UpdateRoleRequest,
};
use crate::domain::user::UserRepository;
use crate::infrastructure::{database::BaseRepository, redis::Redis};

/// Service for managing roles and department-specific role assignments
pub struct RoleService<R, DR, UR>
where
    R: RoleRepository + BaseRepository<Entity = Role>,
    DR: DepartmentRoleRepository,
    UR: UserRepository,
{
    repository: R,
    department_role_repository: DR,
    user_repository: UR,
    redis: Redis,
    cache: Arc<RwLock<HashMap<i32, Role>>>,
}

impl<R, DR, UR> RoleService<R, DR, UR>
where
    R: RoleRepository + BaseRepository<Entity = Role>,
    DR: DepartmentRoleRepository,
    UR: UserRepository,
{
    /// Creates a new instance of the role service
    pub fn new(
        repository: R,
        department_role_repository: DR,
        user_repository: UR,
        redis: Redis,
    ) -> Self {
        Self {
            repository,
            department_role_repository,
            user_repository,
            redis,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new role
    pub async fn create_role(&self, request: CreateRoleRequest) -> RoleResult<Role> {
        // Check if role name already exists in the organization
        if self
            .repository
            .role_name_exists(request.organization_id, &request.name)
            .await?
        {
            return Err(RoleError::RoleAlreadyExists);
        }

        let role = self.repository.create_role(request).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(role.id, role.clone());

        Ok(role)
    }

    /// Updates an existing role
    pub async fn update_role(&self, id: i32, request: UpdateRoleRequest) -> RoleResult<Role> {
        // Check if role exists
        let existing = self.repository.get_role_by_id(id).await?;
        if existing.is_none() {
            return Err(RoleError::RoleNotFound(id));
        }
        let existing = existing.unwrap();

        // Check name uniqueness if it's being updated
        if let Some(ref name) = request.name {
            if self
                .repository
                .role_name_exists(existing.organization_id, name)
                .await?
                && existing.name != *name
            {
                return Err(RoleError::RoleAlreadyExists);
            }
        }

        let role = self.repository.update_role(id, request).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(role.id, role.clone());

        Ok(role)
    }

    /// Deletes a role
    pub async fn delete_role(&self, id: i32) -> RoleResult<bool> {
        // Check if role exists
        let existing = self.repository.get_role_by_id(id).await?;
        if existing.is_none() {
            return Err(RoleError::RoleNotFound(id));
        }

        let result = self.repository.delete_role(id).await?;

        // Update cache if successful
        if result {
            let mut cache = self.cache.write().await;
            cache.remove(&id);
        }

        Ok(result)
    }

    /// Gets a role by ID
    pub async fn get_role(&self, id: i32) -> RoleResult<Option<Role>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(role) = cache.get(&id) {
                return Ok(Some(role.clone()));
            }
        }

        // If not in cache, get from repository
        let role = self.repository.get_role_by_id(id).await?;

        // Update cache if role found
        if let Some(ref role) = role {
            let mut cache = self.cache.write().await;
            cache.insert(role.id, role.clone());
        }

        Ok(role)
    }

    /// Lists roles with pagination
    pub async fn list_roles(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<Role>, i64)> {
        self.repository.list_roles(organization_id, params).await
    }

    /// Batch creates roles
    pub async fn batch_create_roles(
        &self,
        request: BatchCreateRolesRequest,
    ) -> RoleResult<Vec<Role>> {
        let organization_id = request.organization_id;
        self.repository
            .batch_create_roles(organization_id, request.roles)
            .await
    }

    /// Assigns a department-specific role to a user
    pub async fn assign_department_role(
        &self,
        organization_id: i32,
        request: AssignDepartmentRoleRequest,
    ) -> RoleResult<DepartmentRole> {
        // Check if user exists
        let user_exists = self
            .user_repository
            .get_user_by_id(request.user_id)
            .await
            .map_err(|e| RoleError::InternalError(format!("Failed to check user: {}", e)))?
            .is_some();

        if !user_exists {
            return Err(RoleError::UserNotFound(request.user_id));
        }

        // Check if role exists
        let role_exists = self.repository.role_exists(request.role_id).await?;
        if !role_exists {
            return Err(RoleError::RoleNotFound(request.role_id));
        }

        // Check if already assigned
        if self
            .department_role_repository
            .user_has_department_role(
                organization_id,
                request.department_id,
                request.user_id,
                request.role_id,
            )
            .await?
        {
            return Err(RoleError::DepartmentRoleAlreadyAssigned(
                request.user_id,
                request.role_id,
                request.department_id,
            ));
        }

        // Assign the role
        self.department_role_repository
            .assign_department_role(organization_id, request)
            .await
    }

    /// Removes a department-specific role from a user
    pub async fn remove_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        user_id: i32,
        role_id: i32,
    ) -> RoleResult<bool> {
        // Check if assigned
        if !self
            .department_role_repository
            .user_has_department_role(organization_id, department_id, user_id, role_id)
            .await?
        {
            return Err(RoleError::DepartmentRoleNotAssigned(
                user_id,
                role_id,
                department_id,
            ));
        }

        // Remove the role
        self.department_role_repository
            .remove_department_role(organization_id, department_id, user_id, role_id)
            .await
    }

    /// Gets all department roles for a user
    pub async fn get_user_department_roles(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> RoleResult<Vec<DepartmentRole>> {
        // Check if user exists
        let user_exists = self
            .user_repository
            .get_user_by_id(user_id)
            .await
            .map_err(|e| RoleError::InternalError(format!("Failed to check user: {}", e)))?
            .is_some();

        if !user_exists {
            return Err(RoleError::UserNotFound(user_id));
        }

        self.department_role_repository
            .get_user_department_roles(organization_id, user_id)
            .await
    }

    /// Gets all users with a specific role in a department
    pub async fn get_users_with_department_role(
        &self,
        organization_id: i32,
        department_id: i32,
        role_id: i32,
        params: &PaginationParams,
    ) -> RoleResult<(Vec<i32>, i64)> {
        // Check if role exists
        let role_exists = self.repository.role_exists(role_id).await?;
        if !role_exists {
            return Err(RoleError::RoleNotFound(role_id));
        }

        self.department_role_repository
            .get_users_with_department_role(organization_id, department_id, role_id, params)
            .await
    }

    /// Batch assigns department roles
    pub async fn batch_assign_department_roles(
        &self,
        organization_id: i32,
        requests: Vec<AssignDepartmentRoleRequest>,
    ) -> RoleResult<Vec<DepartmentRole>> {
        self.department_role_repository
            .batch_assign_department_roles(organization_id, requests)
            .await
    }

    /// Gets all roles assigned to a user (across all contexts)
    pub async fn get_user_roles(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> RoleResult<Vec<Role>> {
        // Check if user exists
        let user_exists = self
            .user_repository
            .get_user_by_id(user_id)
            .await
            .map_err(|e| RoleError::InternalError(format!("Failed to check user: {}", e)))?
            .is_some();

        if !user_exists {
            return Err(RoleError::UserNotFound(user_id));
        }

        self.repository
            .get_roles_by_user_id(organization_id, user_id)
            .await
    }

    /// Converts a Role entity to a RoleResponse DTO
    fn to_role_response(&self, role: &Role) -> crate::domain::roles::RoleResponse {
        crate::domain::roles::RoleResponse {
            id: role.id,
            organization_id: role.organization_id,
            name: role.name.clone(),
            description: role.description.clone(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}
