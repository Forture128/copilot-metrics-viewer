use crate::common::types::PaginationParams;
use crate::domain::teams::{TeamError, TeamMember, TeamMemberRepository, TeamResult};
use crate::infrastructure::db::DbPool;
use crate::schema::team_members;
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;

use super::BaseRepository;

/// Implementation of the TeamMemberRepository trait
pub struct TeamMemberRepositoryImpl {
    pool: DbPool,
}

impl TeamMemberRepositoryImpl {
    /// Creates a new instance of TeamMemberRepositoryImpl
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamMemberRepository for TeamMemberRepositoryImpl {
    /// Adds a user to a team
    async fn add_team_member(
        &self,
        organization_id: i32,
        team_id: i32,
        user_id: i32,
    ) -> TeamResult<TeamMember> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();
        let new_member = TeamMember {
            id: 0, // Will be set by the database
            organization_id,
            team_id,
            user_id,
            joined_at: now,
            created_at: now,
            updated_at: now,
        };

        let result = diesel::insert_into(team_members::table)
            .values(&new_member)
            .returning(team_members::all_columns)
            .get_result(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to add team member: {}", e)))?;

        Ok(result)
    }

    /// Removes a user from a team
    async fn remove_team_member(
        &self,
        organization_id: i32,
        team_id: i32,
        user_id: i32,
    ) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            team_members::table
                .filter(team_members::team_id.eq(team_id))
                .filter(team_members::organization_id.eq(organization_id))
                .filter(team_members::user_id.eq(user_id)),
        )
        .execute(conn)
        .await
        .map_err(|e| TeamError::DatabaseError(format!("Failed to remove team member: {}", e)))?;

        Ok(affected_rows > 0)
    }

    /// Gets all members of a team
    async fn get_team_members(
        &self,
        organization_id: i32,
        team_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<TeamMember>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Calculate offset based on page and per_page
        let offset = (params.page - 1) * params.per_page;

        // Get total count
        let total: i64 = team_members::table
            .filter(team_members::organization_id.eq(organization_id))
            .filter(team_members::team_id.eq(team_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to count team members: {}", e))
            })?;

        // Get paginated team members
        let members = team_members::table
            .filter(team_members::organization_id.eq(organization_id))
            .filter(team_members::team_id.eq(team_id))
            .order(team_members::joined_at.desc())
            .offset(offset)
            .limit(params.per_page)
            .load::<TeamMember>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list team members: {}", e)))?;

        Ok((members, total))
    }

    /// Gets all team IDs a user belongs to
    async fn get_user_team_ids(&self, organization_id: i32, user_id: i32) -> TeamResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let team_ids = team_members::table
            .filter(team_members::organization_id.eq(organization_id))
            .filter(team_members::user_id.eq(user_id))
            .select(team_members::team_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get teams for user: {}", e))
            })?;

        Ok(team_ids)
    }

    /// Checks if a user is a member of a team
    async fn is_team_member(
        &self,
        organization_id: i32,
        team_id: i32,
        user_id: i32,
    ) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = team_members::table
            .filter(team_members::organization_id.eq(organization_id))
            .filter(team_members::team_id.eq(team_id))
            .filter(team_members::user_id.eq(user_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to check if user is team member: {}", e))
            })?;

        Ok(count > 0)
    }

    /// Batch adds users to a team
    async fn batch_add_team_members(
        &self,
        organization_id: i32,
        team_id: i32,
        user_ids: Vec<i32>,
    ) -> TeamResult<Vec<TeamMember>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();
        let new_members: Vec<TeamMember> = user_ids
            .into_iter()
            .map(|user_id| TeamMember {
                id: 0, // Will be set by the database
                organization_id,
                team_id,
                user_id,
                joined_at: now,
                created_at: now,
                updated_at: now,
            })
            .collect();

        let result = diesel::insert_into(team_members::table)
            .values(&new_members)
            .returning(team_members::all_columns)
            .get_results(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to batch add team members: {}", e))
            })?;

        Ok(result)
    }

    /// Batch removes users from a team
    async fn batch_remove_team_members(
        &self,
        organization_id: i32,
        team_id: i32,
        user_ids: Vec<i32>,
    ) -> TeamResult<usize> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count = diesel::delete(
            team_members::table
                .filter(team_members::organization_id.eq(organization_id))
                .filter(team_members::team_id.eq(team_id))
                .filter(team_members::user_id.eq_any(user_ids)),
        )
        .execute(conn)
        .await
        .map_err(|e| {
            TeamError::DatabaseError(format!("Failed to batch remove team members: {}", e))
        })?;

        Ok(count)
    }

    /// Gets all user IDs for a team
    async fn get_team_user_ids(&self, organization_id: i32, team_id: i32) -> TeamResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let user_ids = team_members::table
            .filter(team_members::organization_id.eq(organization_id))
            .filter(team_members::team_id.eq(team_id))
            .select(team_members::user_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get user IDs for team: {}", e))
            })?;

        Ok(user_ids)
    }

    /// Counts members in a team
    async fn count_team_members(&self, organization_id: i32, team_id: i32) -> TeamResult<i64> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count = team_members::table
            .filter(team_members::organization_id.eq(organization_id))
            .filter(team_members::team_id.eq(team_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to count team members: {}", e))
            })?;

        Ok(count)
    }
}

#[async_trait]
impl BaseRepository for TeamMemberRepositoryImpl {
    type Entity = TeamMember;
    type SqlType = (
        diesel::sql_types::Integer,   // id
        diesel::sql_types::Integer,   // organization_id
        diesel::sql_types::Integer,   // team_id
        diesel::sql_types::Integer,   // user_id
        diesel::sql_types::Timestamp, // joined_at
        diesel::sql_types::Timestamp, // created_at
        diesel::sql_types::Timestamp, // updated_at
    );
    type Error = TeamError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> TeamResult<Option<TeamMember>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = team_members::table
            .filter(team_members::id.eq(id))
            .first::<TeamMember>(conn)
            .await
            .optional()
            .map_err(|e| TeamError::DatabaseError(format!("Failed to get team member: {}", e)))?;

        Ok(result)
    }

    async fn find_all(&self) -> TeamResult<Vec<TeamMember>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let members = team_members::table
            .load::<TeamMember>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list team members: {}", e)))?;

        Ok(members)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<TeamMember>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let total = team_members::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to count team members: {}", e))
            })?;

        let members = team_members::table
            .order(team_members::joined_at.desc())
            .offset(offset)
            .limit(params.per_page)
            .load::<TeamMember>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list team members: {}", e)))?;

        Ok((members, total))
    }
}
