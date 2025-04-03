use crate::domain::auth::AuthResult;
use async_trait::async_trait;
use diesel::prelude::*;

/// Database user representation
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: i32,
    pub organization_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// AuthRepository trait defines operations for authentication
#[async_trait]
pub trait AuthRepository: Send + Sync {
    /// Find user by username or email
    async fn find_user_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> AuthResult<Option<DbUser>>;

    /// Check if user has super_admin role
    async fn check_user_has_super_admin_role(
        &self,
        user_id: i32,
        organization_id: i32,
    ) -> AuthResult<bool>;

    /// Verify password against hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String>;
}
