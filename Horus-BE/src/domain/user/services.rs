use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::types::{Claims, PaginationParams};
use crate::domain::user::entities::UserRole;
use crate::domain::user::{
    AssignRoleRequest, AuthResponse, ChangePasswordRequest, CreateUserRequest, LoginRequest,
    UpdateUserRequest, User, UserError, UserRepository, UserResponse, UserResult,
    UserRoleRepository, UserWithRolesResponse,
};
use crate::infrastructure::{database::BaseRepository, redis::Redis};

/// User service for managing users and authentication
pub struct UserService<R, RR>
where
    R: UserRepository + BaseRepository<Entity = User>,
    RR: UserRoleRepository,
{
    repository: R,
    role_repository: RR,
    redis: Redis,
    jwt_secret: String,
    token_expiry: u64,
    cache: Arc<RwLock<HashMap<i32, User>>>,
}

impl<R, RR> UserService<R, RR>
where
    R: UserRepository + BaseRepository<Entity = User>,
    RR: UserRoleRepository,
{
    /// Creates a new instance of the user service
    pub fn new(
        repository: R,
        role_repository: RR,
        redis: Redis,
        jwt_secret: String,
        token_expiry: u64,
    ) -> Self {
        Self {
            repository,
            role_repository,
            redis,
            jwt_secret,
            token_expiry,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new user
    pub async fn create_user(&self, request: CreateUserRequest) -> UserResult<User> {
        // Check if username already exists
        if self.repository.username_exists(&request.username).await? {
            return Err(UserError::UserAlreadyExists);
        }

        // Check if email already exists
        if self.repository.email_exists(&request.email).await? {
            return Err(UserError::UserAlreadyExists);
        }

        // Hash the password
        let password_hash = self.hash_password(&request.password)?;

        // Create user with hashed password
        let mut dto = request;
        dto.password = password_hash;

        let user = self.repository.create_user(dto).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(user.id, user.clone());

        Ok(user)
    }

    /// Updates an existing user
    pub async fn update_user(&self, id: i32, request: UpdateUserRequest) -> UserResult<User> {
        // Check if user exists
        let existing = self.repository.get_user_by_id(id).await?;
        if existing.is_none() {
            return Err(UserError::UserNotFound(id));
        }

        // Check username uniqueness if it's being updated
        if let Some(ref username) = request.username {
            if self.repository.username_exists(username).await?
                && existing.as_ref().unwrap().username != *username
            {
                return Err(UserError::UserAlreadyExists);
            }
        }

        // Check email uniqueness if it's being updated
        if let Some(ref email) = request.email {
            if self.repository.email_exists(email).await?
                && existing.as_ref().unwrap().email != *email
            {
                return Err(UserError::UserAlreadyExists);
            }
        }

        let user = self.repository.update_user(id, request).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(user.id, user.clone());

        Ok(user)
    }

    /// Changes a user's password
    pub async fn change_password(
        &self,
        id: i32,
        request: ChangePasswordRequest,
    ) -> UserResult<bool> {
        // Get the user
        let user = match self.repository.get_user_by_id(id).await? {
            Some(user) => user,
            None => return Err(UserError::UserNotFound(id)),
        };

        // Verify current password
        if !self.verify_password(&request.current_password, &user.password)? {
            return Err(UserError::PasswordError(
                "Current password is incorrect".to_string(),
            ));
        }

        // Hash the new password
        let password_hash = self.hash_password(&request.new_password)?;

        // Update the password
        self.repository.update_password(id, &password_hash).await
    }

    /// Authenticates a user and returns a JWT token
    pub async fn login(&self, request: LoginRequest) -> UserResult<AuthResponse> {
        // Try to find user by username or email
        let user = match self
            .repository
            .get_user_by_username(&request.username_or_email)
            .await?
        {
            Some(user) => user,
            None => {
                match self
                    .repository
                    .get_user_by_email(&request.username_or_email)
                    .await?
                {
                    Some(user) => user,
                    None => {
                        return Err(UserError::AuthenticationError(
                            "Invalid credentials".to_string(),
                        ))
                    }
                }
            }
        };

        // Verify password
        if !self.verify_password(&request.password, &user.password)? {
            return Err(UserError::AuthenticationError(
                "Invalid credentials".to_string(),
            ));
        }

        // Generate JWT token
        let token = self.generate_token(user.id, user.organization_id)?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_expiry,
            user: self.to_user_response(&user),
        })
    }

    /// Gets a user by ID
    pub async fn get_user(&self, id: i32) -> UserResult<Option<User>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(user) = cache.get(&id) {
                return Ok(Some(user.clone()));
            }
        }

        // If not in cache, get from repository
        let user = self.repository.get_user_by_id(id).await?;

        // Update cache if user found
        if let Some(ref user) = user {
            let mut cache = self.cache.write().await;
            cache.insert(user.id, user.clone());
        }

        Ok(user)
    }

    /// Gets a user with their roles
    pub async fn get_user_with_roles(&self, id: i32) -> UserResult<UserWithRolesResponse> {
        // Get the user
        let user = match self.get_user(id).await? {
            Some(user) => user,
            None => return Err(UserError::UserNotFound(id)),
        };

        // Get the user's roles
        let role_ids = self
            .role_repository
            .get_user_role_ids(user.organization_id, user.id)
            .await?;

        Ok(UserWithRolesResponse {
            user: self.to_user_response(&user),
            role_ids,
        })
    }

    /// Lists users with pagination
    pub async fn list_users(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)> {
        self.repository.list_users(organization_id, params).await
    }

    /// Converts a User entity to a UserResponse DTO
    fn to_user_response(&self, user: &User) -> UserResponse {
        UserResponse {
            id: user.id,
            organization_id: user.organization_id,
            username: user.username.clone(),
            email: user.email.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    /// Hashes a password using Argon2
    fn hash_password(&self, password: &str) -> UserResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| UserError::PasswordError(format!("Failed to hash password: {}", e)))
    }

    /// Verifies a password against a hash
    fn verify_password(&self, password: &str, hash: &str) -> UserResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| UserError::PasswordError(format!("Invalid password hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generates a JWT token
    fn generate_token(&self, user_id: i32, organization_id: i32) -> UserResult<String> {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::seconds(self.token_expiry as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            org: organization_id,
            role: "user".to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| UserError::InternalError(format!("Failed to generate token: {}", e)))
    }

    /// Assigns a role to a user
    pub async fn assign_role(&self, request: AssignRoleRequest) -> UserResult<UserRole> {
        // Get the user to ensure it exists and get the organization_id
        let user = match self.get_user(request.user_id).await? {
            Some(user) => user,
            None => return Err(UserError::UserNotFound(request.user_id)),
        };

        // Assign the role and convert the result
        let repo_role = self
            .role_repository
            .assign_role(user.organization_id, request.user_id, request.role_id)
            .await?;

        Ok(repo_role)
    }

    /// Removes a role from a user
    pub async fn remove_role(&self, user_id: i32, role_id: i32) -> UserResult<bool> {
        // Get the user to ensure it exists and get the organization_id
        let user = match self.get_user(user_id).await? {
            Some(user) => user,
            None => return Err(UserError::UserNotFound(user_id)),
        };

        // Remove the role
        self.role_repository
            .remove_role(user.organization_id, user_id, role_id)
            .await
    }

    /// Gets all roles for a user
    pub async fn get_user_roles(&self, user_id: i32) -> UserResult<Vec<UserRole>> {
        // Get the user to ensure it exists and get the organization_id
        let user = match self.get_user(user_id).await? {
            Some(user) => user,
            None => return Err(UserError::UserNotFound(user_id)),
        };

        // Get the roles
        let repo_roles = self
            .role_repository
            .get_user_roles(user.organization_id, user_id)
            .await?;

        // Convert the repository roles to domain entity roles
        Ok(repo_roles)
    }
}
