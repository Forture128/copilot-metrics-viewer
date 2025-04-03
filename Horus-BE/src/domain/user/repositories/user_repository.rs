use crate::common::types::PaginationParams;
use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User, UserResult};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    /// Creates a new user
    async fn create_user(&self, dto: CreateUserRequest) -> UserResult<User>;

    /// Updates an existing user
    async fn update_user(&self, id: i32, dto: UpdateUserRequest) -> UserResult<User>;

    /// Deletes a user
    async fn delete_user(&self, id: i32) -> UserResult<bool>;

    /// Gets a user by their ID
    async fn get_user_by_id(&self, id: i32) -> UserResult<Option<User>>;

    /// Gets a user by their username
    async fn get_user_by_username(&self, username: &str) -> UserResult<Option<User>>;

    /// Gets a user by their email
    async fn get_user_by_email(&self, email: &str) -> UserResult<Option<User>>;

    /// Checks if a username already exists
    async fn username_exists(&self, username: &str) -> UserResult<bool>;

    /// Checks if an email already exists
    async fn email_exists(&self, email: &str) -> UserResult<bool>;

    /// Searches for users by username pattern
    async fn find_users_by_username(
        &self,
        organization_id: i32,
        username_pattern: &str,
        params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)>;

    /// Lists all users for an organization with pagination
    async fn list_users(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)>;

    /// Gets multiple users by their IDs
    async fn get_users_by_ids(&self, ids: Vec<i32>) -> UserResult<Vec<User>>;

    /// Updates a user's password
    async fn update_password(&self, id: i32, hashed_password: &str) -> UserResult<bool>;
}
