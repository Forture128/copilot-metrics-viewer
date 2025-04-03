use crate::{
    infrastructure::database::{Entity, Timestamps},
    schema::departments,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a department in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = departments)]
pub struct Department {
    /// Unique identifier for the department
    pub id: i32,
    /// ID of the organization this department belongs to
    pub organization_id: i32,
    /// Name of the department
    pub name: String,
    /// Timestamp when the department was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the department was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for Department {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for Department {
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
