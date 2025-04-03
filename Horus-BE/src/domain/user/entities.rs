use crate::{
    infrastructure::database::{Entity, Timestamps},
    schema::{user_roles, users},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a user in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    /// Unique identifier for the user
    pub id: i32,
    /// ID of the organization this user belongs to
    pub organization_id: i32,
    /// Username for the user
    pub username: String,
    /// Email address of the user
    pub email: String,
    /// Hashed password of the user
    pub password: String,
    /// Timestamp when the user was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the user was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for User {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for User {
    fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn get_updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    fn set_updated_at(&mut self, time: NaiveDateTime) {
        self.updated_at = time;
    }
}

/// Represents a user role in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = user_roles)]
pub struct UserRole {
    pub id: i32,
    pub organization_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Entity for UserRole {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for UserRole {
    fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn get_updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    fn set_updated_at(&mut self, time: NaiveDateTime) {
        self.updated_at = time;
    }
}
