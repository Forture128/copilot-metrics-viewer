use crate::domain::auth::repositories::auth_repository::{AuthRepository, DbUser};
use crate::domain::auth::{AuthError, AuthResult};
use crate::infrastructure::db::DbPool;
use crate::schema::{roles, user_roles, users};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

/// Implementation of AuthRepository
pub struct AuthRepositoryImpl {
    db_pool: DbPool,
}

impl AuthRepositoryImpl {
    /// Create a new instance of AuthRepositoryImpl
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn find_user_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> AuthResult<Option<DbUser>> {
        // Get a database connection from the pool
        let mut conn = self
            .db_pool
            .get()
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Try to find the user by username or email
        let user_result = users::table
            .filter(
                users::email
                    .eq(username_or_email)
                    .or(users::username.eq(username_or_email)),
            )
            .first::<DbUser>(&mut conn)
            .await;

        match user_result {
            Ok(user) => Ok(Some(user)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(AuthError::DatabaseError(e.to_string())),
        }
    }

    async fn check_user_has_super_admin_role(
        &self,
        user_id: i32,
        organization_id: i32,
    ) -> AuthResult<bool> {
        // Get a database connection from the pool
        let mut conn = self
            .db_pool
            .get()
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Check if user has super_admin role
        let result = user_roles::table
            .inner_join(roles::table)
            .filter(user_roles::user_id.eq(user_id))
            .filter(user_roles::organization_id.eq(organization_id))
            .filter(roles::name.eq("super_admin"))
            .select(roles::id)
            .first::<i32>(&mut conn)
            .await;

        Ok(result.is_ok())
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        // Parse the password hash
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(ph) => ph,
            Err(_) => return Err("Invalid password hash format".to_string()),
        };

        // Verify the password against the hash
        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
