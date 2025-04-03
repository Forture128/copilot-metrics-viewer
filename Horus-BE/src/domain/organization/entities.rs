use crate::{
    infrastructure::database::{Entity, Timestamps},
    schema::{organization_configs, organizations},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
/// Represents an organization in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = organizations)]
pub struct Organization {
    /// Unique identifier for the organization
    pub id: i32,
    /// Name of the organization
    pub name: String,
    /// Timestamp when the organization was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the organization was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for Organization {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for Organization {
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

/// Represents a configuration key-value pair for an organization
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = organization_configs)]
pub struct OrganizationConfig {
    /// Unique identifier for the configuration
    pub id: i32,
    /// ID of the organization this config belongs to
    pub organization_id: i32,
    /// Configuration key
    pub config_key: String,
    /// Configuration value
    pub config_value: String,
    /// Timestamp when the configuration was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the configuration was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for OrganizationConfig {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for OrganizationConfig {
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
