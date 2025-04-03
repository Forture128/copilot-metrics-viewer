use crate::{
    infrastructure::database::{Entity, Timestamps},
    schema::{team_members, team_repos, teams},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents a team in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = teams)]
pub struct Team {
    /// Unique identifier for the team
    pub id: i32,
    /// ID of the organization this team belongs to
    pub organization_id: i32,
    /// ID of the department this team belongs to
    pub department_id: i32,
    /// Name of the team
    pub name: String,
    /// GitHub team name
    pub name_gh: String,
    /// Timestamp when the team was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the team was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for Team {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for Team {
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

/// Represents a team member in the system
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = team_members)]
pub struct TeamMember {
    /// Unique identifier for the team member
    pub id: i32,
    /// ID of the organization this team member belongs to
    pub organization_id: i32,
    /// ID of the team this member belongs to
    pub team_id: i32,
    /// ID of the user that is a member of the team
    pub user_id: i32,
    /// When the user joined the team
    pub joined_at: NaiveDateTime,
    /// Timestamp when the team member was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the team member was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for TeamMember {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for TeamMember {
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
/// Represents a team-repository association
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = team_repos)]
pub struct TeamRepo {
    /// Unique identifier for the team-repo association
    pub id: i32,
    /// ID of the organization
    pub organization_id: i32,
    /// ID of the team
    pub team_id: i32,
    /// ID of the repository
    pub repo_id: i32,
    /// Timestamp when the team was created
    pub created_at: NaiveDateTime,
    /// Timestamp when the team was last updated
    pub updated_at: NaiveDateTime,
}

impl Entity for TeamRepo {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl Timestamps for TeamRepo {
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
