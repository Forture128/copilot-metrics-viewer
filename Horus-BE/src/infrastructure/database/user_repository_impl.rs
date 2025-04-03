use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    common::types::PaginationParams,
    domain::user::repositories::user_repository::UserRepository,
    domain::user::{CreateUserRequest, UpdateUserRequest, User, UserError, UserResult},
    infrastructure::database::base_repository::BaseRepository,
    infrastructure::db::DbPool,
    schema::users,
};

/// Implementation of the user repository
pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    /// Creates a new instance of the user repository
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseRepository for UserRepositoryImpl {
    type Entity = User;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Text,
        diesel::sql_types::Text,
        diesel::sql_types::Text,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );
    type Error = UserError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> UserResult<Option<Self::Entity>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = users::table
            .find(id)
            .first::<Self::Entity>(conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn find_all(&self) -> UserResult<Vec<Self::Entity>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let results = users::table
            .load::<Self::Entity>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> UserResult<(Vec<Self::Entity>, i64)> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let results = users::table
            .limit(params.per_page)
            .offset(offset)
            .load::<Self::Entity>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total = users::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((results, total))
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, dto: CreateUserRequest) -> UserResult<User> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Check if username or email already exists
        if self.username_exists(&dto.username).await? {
            return Err(UserError::ValidationError(format!(
                "Username '{}' is already taken",
                dto.username
            )));
        }

        if self.email_exists(&dto.email).await? {
            return Err(UserError::ValidationError(format!(
                "Email '{}' is already registered",
                dto.email
            )));
        }

        let now = Utc::now().naive_utc();
        let new_user = User {
            id: 0, // Auto-incremented by DB
            organization_id: dto.organization_id,
            username: dto.username,
            email: dto.email,
            password: dto.password, // Note: In a real app, this should be hashed before storing
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))
    }

    async fn update_user(&self, id: i32, dto: UpdateUserRequest) -> UserResult<User> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Find the user first
        let user = users::table
            .find(id)
            .first::<User>(conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?
            .ok_or_else(|| UserError::UserNotFound(id))?;

        // Check if the new username (if provided) is already taken by someone else
        if let Some(username) = &dto.username {
            if username != &user.username && self.username_exists(username).await? {
                return Err(UserError::ValidationError(format!(
                    "Username '{}' is already taken",
                    username
                )));
            }
        }

        // Check if the new email (if provided) is already taken by someone else
        if let Some(email) = &dto.email {
            if email != &user.email && self.email_exists(email).await? {
                return Err(UserError::ValidationError(format!(
                    "Email '{}' is already registered",
                    email
                )));
            }
        }

        let updated_user = diesel::update(users::table.find(id))
            .set((
                users::username.eq(dto.username.unwrap_or(user.username)),
                users::email.eq(dto.email.unwrap_or(user.email)),
                users::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<User>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(updated_user)
    }

    async fn delete_user(&self, id: i32) -> UserResult<bool> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let deleted_count = diesel::delete(users::table.find(id))
            .execute(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(deleted_count > 0)
    }

    async fn get_user_by_id(&self, id: i32) -> UserResult<Option<User>> {
        self.find_by_id(id).await
    }

    async fn get_user_by_username(&self, username: &str) -> UserResult<Option<User>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_user_by_email(&self, email: &str) -> UserResult<Option<User>> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = users::table
            .filter(users::email.eq(email))
            .first::<User>(conn)
            .await
            .optional()
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn username_exists(&self, username: &str) -> UserResult<bool> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = users::table
            .filter(users::username.eq(username))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(count > 0)
    }

    async fn email_exists(&self, email: &str) -> UserResult<bool> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = users::table
            .filter(users::email.eq(email))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(count > 0)
    }

    async fn find_users_by_username(
        &self,
        organization_id: i32,
        username_pattern: &str,
        params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;
        let like_pattern = format!("%{}%", username_pattern);

        let results = users::table
            .filter(users::organization_id.eq(organization_id))
            .filter(users::username.like(&like_pattern))
            .limit(params.per_page)
            .offset(offset)
            .load::<User>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total = users::table
            .filter(users::organization_id.eq(organization_id))
            .filter(users::username.like(&like_pattern))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((results, total))
    }

    async fn list_users(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let results = users::table
            .filter(users::organization_id.eq(organization_id))
            .limit(params.per_page)
            .offset(offset)
            .load::<User>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        let total = users::table
            .filter(users::organization_id.eq(organization_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok((results, total))
    }

    async fn get_users_by_ids(&self, ids: Vec<i32>) -> UserResult<Vec<User>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let results = users::table
            .filter(users::id.eq_any(ids))
            .load::<User>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn update_password(&self, id: i32, hashed_password: &str) -> UserResult<bool> {
        let conn = &mut self.get_pool().get().await.map_err(|e| {
            UserError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let updated_rows = diesel::update(users::table.find(id))
            .set((
                users::password.eq(hashed_password),
                users::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?;

        Ok(updated_rows > 0)
    }
}
