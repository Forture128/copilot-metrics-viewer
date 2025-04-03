use crate::common::types::PaginationParams;
use crate::domain::teams::{
    CreateTeamRequest, Team, TeamError, TeamRepository, TeamResult, UpdateTeamRequest,
};
use crate::infrastructure::database::BaseRepository;
use crate::infrastructure::db::DbPool;
use crate::schema::{team_members, teams};
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

/// Implementation of the TeamRepository trait
pub struct TeamRepositoryImpl {
    pool: DbPool,
}

impl TeamRepositoryImpl {
    /// Creates a new instance of TeamRepositoryImpl
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepository for TeamRepositoryImpl {
    async fn create_team(&self, dto: CreateTeamRequest) -> TeamResult<Team> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();
        let new_team = Team {
            id: 0, // Will be set by the database
            organization_id: dto.organization_id,
            department_id: dto.department_id,
            name: dto.name.clone(),
            name_gh: dto.name_gh.clone(),
            created_at: now,
            updated_at: now,
        };

        let result = diesel::insert_into(teams::table)
            .values(&new_team)
            .returning(teams::all_columns)
            .get_result(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to create team: {}", e)))?;

        Ok(result)
    }

    async fn update_team(&self, id: i32, dto: UpdateTeamRequest) -> TeamResult<Team> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Verify team exists
        let exists = self.team_exists(id).await?;
        if !exists {
            return Err(TeamError::TeamNotFound(id));
        }

        // Get current team to update only changed fields
        let current_team = self
            .get_team_by_id(id)
            .await?
            .ok_or_else(|| TeamError::TeamNotFound(id))?;

        let mut team = current_team;
        team.updated_at = Utc::now().naive_utc();

        // Update fields if provided
        if let Some(name) = dto.name {
            team.name = name;
        }

        if let Some(name_gh) = dto.name_gh {
            team.name_gh = name_gh;
        }

        let result = diesel::update(teams::table.find(id))
            .set(&team)
            .returning(teams::all_columns)
            .get_result(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to update team: {}", e)))?;

        Ok(result)
    }

    async fn delete_team(&self, id: i32) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count = diesel::delete(teams::table.filter(teams::id.eq(id)))
            .execute(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to delete team: {}", e)))?;

        Ok(count > 0)
    }

    async fn get_team_by_id(&self, id: i32) -> TeamResult<Option<Team>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = teams::table
            .filter(teams::id.eq(id))
            .first::<Team>(conn)
            .await
            .optional()
            .map_err(|e| TeamError::DatabaseError(format!("Failed to get team by ID: {}", e)))?;

        Ok(result)
    }

    async fn team_exists(&self, id: i32) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = teams::table
            .filter(teams::id.eq(id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to check if team exists: {}", e))
            })?;

        Ok(count > 0)
    }

    async fn list_teams(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Calculate offset based on page and per_page
        let offset = (params.page - 1) * params.per_page;

        // Get total count
        let total: i64 = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get total teams count: {}", e))
            })?;

        // Get teams for the page
        let teams_list = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .order(teams::name.asc())
            .offset(offset)
            .limit(params.per_page)
            .load::<Team>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list teams: {}", e)))?;

        Ok((teams_list, total))
    }

    async fn list_teams_by_department(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Calculate offset based on page and per_page
        let offset = (params.page - 1) * params.per_page;

        // Get total count
        let total: i64 = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .filter(teams::department_id.eq(department_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get total teams count: {}", e))
            })?;

        // Get teams for the page
        let teams_list = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .filter(teams::department_id.eq(department_id))
            .order(teams::name.asc())
            .offset(offset)
            .limit(params.per_page)
            .load::<Team>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list teams: {}", e)))?;

        Ok((teams_list, total))
    }

    async fn team_name_exists(&self, organization_id: i32, name: &str) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .filter(teams::name.eq(name))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to check if team name exists: {}", e))
            })?;

        Ok(count > 0)
    }

    async fn team_gh_name_exists(&self, organization_id: i32, name_gh: &str) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .filter(teams::name_gh.eq(name_gh))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!(
                    "Failed to check if GitHub team name exists: {}",
                    e
                ))
            })?;

        Ok(count > 0)
    }

    async fn find_teams_by_name(
        &self,
        organization_id: i32,
        name_pattern: &str,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Team>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let like_pattern = format!("%{}%", name_pattern);

        // Calculate offset based on page and per_page
        let offset = (params.page - 1) * params.per_page;

        // Get total count
        let total: i64 = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .filter(teams::name.ilike(&like_pattern))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get total teams count: {}", e))
            })?;

        // Get teams for the page
        let teams_list = teams::table
            .filter(teams::organization_id.eq(organization_id))
            .filter(teams::name.ilike(&like_pattern))
            .order(teams::name.asc())
            .offset(offset)
            .limit(params.per_page)
            .load::<Team>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to find teams by name: {}", e))
            })?;

        Ok((teams_list, total))
    }

    async fn get_teams_by_ids(&self, ids: Vec<i32>) -> TeamResult<Vec<Team>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let teams_list = teams::table
            .filter(teams::id.eq_any(ids))
            .load::<Team>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to get teams by IDs: {}", e)))?;

        Ok(teams_list)
    }

    async fn get_teams_by_user_id(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> TeamResult<Vec<Team>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let teams_list = teams::table
            .inner_join(team_members::table)
            .filter(teams::organization_id.eq(organization_id))
            .filter(team_members::user_id.eq(user_id))
            .select(teams::all_columns)
            .load::<Team>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get teams for user: {}", e))
            })?;

        Ok(teams_list)
    }
}

#[async_trait]
impl BaseRepository for TeamRepositoryImpl {
    type Entity = Team;
    type Error = TeamError;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Text,
        diesel::sql_types::Text,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> TeamResult<Option<Self::Entity>> {
        let result = self
            .get_team_by_id(id)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to find team by ID: {}", e)))?;

        Ok(result)
    }

    async fn find_all(&self) -> TeamResult<Vec<Self::Entity>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = teams::table
            .load::<Team>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to find all teams: {}", e)))?;

        Ok(result)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<Self::Entity>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Calculate offset based on page and per_page
        let offset = (params.page - 1) * params.per_page;

        // Get total count
        let total: i64 = teams::table.count().get_result(conn).await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get total teams count: {}", e))
        })?;

        // Get teams for the page
        let result = teams::table
            .order(teams::name.asc())
            .offset(offset)
            .limit(params.per_page)
            .load::<Team>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get teams for the page: {}", e))
            })?;

        Ok((result, total))
    }
}
