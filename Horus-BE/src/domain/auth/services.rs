use crate::common::types::Claims;
use crate::domain::auth::repositories::auth_repository::AuthRepository;
use crate::domain::auth::{AuthError, AuthResponse, AuthResult, LoginRequest};
use crate::infrastructure::database::AuthRepositoryImpl;
use crate::infrastructure::db::DbPool;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

/// Auth service for handling authentication and authorization
pub struct AuthService<R: AuthRepository> {
    repository: R,
    jwt_secret: String,
    token_expiry: i64,
}

impl<R: AuthRepository> AuthService<R> {
    /// Create a new auth service instance
    pub fn new(repository: R, jwt_secret: String, token_expiry: i64) -> Self {
        Self {
            repository,
            jwt_secret,
            token_expiry,
        }
    }

    /// Login a user and generate an authentication token
    pub async fn login(&self, login_req: LoginRequest) -> AuthResult<AuthResponse> {
        // Find user by username or email
        let user = match self
            .repository
            .find_user_by_username_or_email(&login_req.username_or_email)
            .await?
        {
            Some(user) => user,
            None => return Err(AuthError::InvalidCredentials),
        };

        // Verify password
        let is_valid_password = self
            .repository
            .verify_password(&login_req.password, &user.password)
            .map_err(|_| AuthError::InvalidCredentials)?;

        if !is_valid_password {
            return Err(AuthError::InvalidCredentials);
        }

        // Check if user has super_admin role
        let is_super_admin = self
            .repository
            .check_user_has_super_admin_role(user.id, user.organization_id)
            .await?;

        // Determine role for token
        let role = if is_super_admin {
            "super_admin"
        } else {
            "user"
        };

        // Create JWT claims
        let claims = Claims::new(
            user.id,
            user.organization_id,
            role,
            self.token_expiry as u64,
        );

        // Generate token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::TokenError(e.to_string()))?;

        // Return token response
        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_expiry,
            user_id: user.id,
            organization_id: user.organization_id,
            is_super_admin,
        })
    }
}

/// Auth service factory
#[derive(Clone)]
pub struct AuthServiceFactory {
    db_pool: Arc<DbPool>,
    jwt_secret: String,
    token_expiry: i64,
}

impl AuthServiceFactory {
    /// Create a new auth service factory
    pub fn new(db_pool: DbPool, jwt_secret: String, token_expiry: i64) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
            jwt_secret,
            token_expiry,
        }
    }

    /// Create a new auth service instance
    pub fn create_service(&self) -> AuthService<AuthRepositoryImpl> {
        let repository = AuthRepositoryImpl::new(self.db_pool.as_ref().clone());
        AuthService::new(repository, self.jwt_secret.clone(), self.token_expiry)
    }
}
