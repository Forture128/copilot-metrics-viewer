use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Base fields that are common across all models
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
pub struct BaseModel {
    pub id: i32,
    pub is_active: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Trait for soft delete functionality
#[async_trait::async_trait]
pub trait SoftDelete {
    async fn soft_delete(&mut self) -> Result<(), diesel::result::Error>;
    async fn restore(&mut self) -> Result<(), diesel::result::Error>;
}

/// Trait for timestamp functionality
pub trait Timestamps {
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
}

/// Trait for active status
pub trait ActiveStatus {
    fn is_active(&self) -> bool;
    fn is_deleted(&self) -> bool;
}

/// Implementation of common traits for BaseModel
impl Timestamps for BaseModel {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl ActiveStatus for BaseModel {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
