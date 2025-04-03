use crate::{
    infrastructure::database::{Entity, Timestamps},
    schema::{departments_roles, roles},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents a role in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = roles)]
pub struct Role {
    /// Unique identifier for the role
    pub id: i32,
    /// ID of the organization this role belongs to
    pub organization_id: i32,
    /// Name of the role
    pub name: String,
    /// Description of the role
    pub description: Option<String>,
    /// Timestamp when the role was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the role was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for Role {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for Role {
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

/// Represents a department-specific role assignment
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = departments_roles)]
pub struct DepartmentRole {
    /// Unique identifier for the department role
    pub id: i32,
    /// ID of the organization
    pub organization_id: i32,
    /// ID of the department
    pub department_id: i32,
    /// ID of the user
    pub user_id: i32,
    /// ID of the role
    pub role_id: i32,
    /// Timestamp when the department role was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the department role was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for DepartmentRole {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for DepartmentRole {
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
