use crate::common::types::PaginationParams;
use crate::domain::teams::{TeamError, TeamRepo, TeamRepoRepository, TeamResult};
use crate::infrastructure::database::BaseRepository;
use crate::infrastructure::db::DbPool;
use crate::schema::team_repos;
use async_trait::async_trait;
use chrono::Utc;
use diesel::result::OptionalExtension;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

/// Implementation of the TeamRepoRepository trait
pub struct TeamRepoRepositoryImpl {
    pool: DbPool,
}

impl TeamRepoRepositoryImpl {
    /// Creates a new instance of TeamRepoRepositoryImpl
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepoRepository for TeamRepoRepositoryImpl {
    async fn assign_repo_to_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_id: i32,
    ) -> TeamResult<TeamRepo> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();
        let new_team_repo = TeamRepo {
            id: 0, // Will be set by the database
            organization_id,
            team_id,
            repo_id,
            created_at: now,
            updated_at: now,
        };

        let result = diesel::insert_into(team_repos::table)
            .values(&new_team_repo)
            .returning(team_repos::all_columns)
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to assign repo to team: {}", e))
            })?;

        Ok(result)
    }

    async fn remove_repo_from_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_id: i32,
    ) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            team_repos::table
                .filter(team_repos::organization_id.eq(organization_id))
                .filter(team_repos::team_id.eq(team_id))
                .filter(team_repos::repo_id.eq(repo_id)),
        )
        .execute(conn)
        .await
        .map_err(|e| TeamError::DatabaseError(format!("Failed to remove repo from team: {}", e)))?;

        Ok(affected_rows > 0)
    }

    async fn get_team_repos(
        &self,
        organization_id: i32,
        team_id: i32,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<TeamRepo>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        // Calculate offset based on page and per_page
        let offset = (params.page - 1) * params.per_page;

        // Get total count
        let total: i64 = team_repos::table
            .filter(team_repos::organization_id.eq(organization_id))
            .filter(team_repos::team_id.eq(team_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to count team repos: {}", e)))?;

        // Get paginated team repos
        let repos = team_repos::table
            .filter(team_repos::organization_id.eq(organization_id))
            .filter(team_repos::team_id.eq(team_id))
            .order(team_repos::id.desc())
            .offset(offset)
            .limit(params.per_page)
            .load::<TeamRepo>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list team repos: {}", e)))?;

        Ok((repos, total))
    }

    async fn get_repo_team_ids(&self, organization_id: i32, repo_id: i32) -> TeamResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let team_ids = team_repos::table
            .filter(team_repos::organization_id.eq(organization_id))
            .filter(team_repos::repo_id.eq(repo_id))
            .select(team_repos::team_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get teams for repo: {}", e))
            })?;

        Ok(team_ids)
    }

    async fn is_repo_assigned_to_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_id: i32,
    ) -> TeamResult<bool> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count: i64 = team_repos::table
            .filter(team_repos::organization_id.eq(organization_id))
            .filter(team_repos::team_id.eq(team_id))
            .filter(team_repos::repo_id.eq(repo_id))
            .count()
            .get_result(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to check if repo belongs to team: {}", e))
            })?;

        Ok(count > 0)
    }

    async fn batch_assign_repos_to_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_ids: Vec<i32>,
    ) -> TeamResult<Vec<TeamRepo>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let now = Utc::now().naive_utc();
        let new_team_repos: Vec<TeamRepo> = repo_ids
            .into_iter()
            .map(|repo_id| TeamRepo {
                id: 0, // Will be set by the database
                organization_id,
                team_id,
                repo_id,
                created_at: now,
                updated_at: now,
            })
            .collect();

        let result = diesel::insert_into(team_repos::table)
            .values(&new_team_repos)
            .returning(team_repos::all_columns)
            .get_results(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to batch assign repos to team: {}", e))
            })?;

        Ok(result)
    }

    async fn batch_remove_repos_from_team(
        &self,
        organization_id: i32,
        team_id: i32,
        repo_ids: Vec<i32>,
    ) -> TeamResult<usize> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let affected_rows = diesel::delete(
            team_repos::table
                .filter(team_repos::organization_id.eq(organization_id))
                .filter(team_repos::team_id.eq(team_id))
                .filter(team_repos::repo_id.eq_any(repo_ids)),
        )
        .execute(conn)
        .await
        .map_err(|e| {
            TeamError::DatabaseError(format!("Failed to remove repos from team: {}", e))
        })?;

        Ok(affected_rows)
    }

    async fn get_team_repo_ids(&self, organization_id: i32, team_id: i32) -> TeamResult<Vec<i32>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let repo_ids = team_repos::table
            .filter(team_repos::organization_id.eq(organization_id))
            .filter(team_repos::team_id.eq(team_id))
            .select(team_repos::repo_id)
            .load::<i32>(conn)
            .await
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get repo IDs for team: {}", e))
            })?;

        Ok(repo_ids)
    }

    async fn count_team_repos(&self, organization_id: i32, team_id: i32) -> TeamResult<i64> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let count = team_repos::table
            .filter(team_repos::organization_id.eq(organization_id))
            .filter(team_repos::team_id.eq(team_id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to count team repos: {}", e)))?;

        Ok(count)
    }
}

#[async_trait]
impl BaseRepository for TeamRepoRepositoryImpl {
    type Entity = TeamRepo;
    type SqlType = (
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Integer,
        diesel::sql_types::Timestamp,
        diesel::sql_types::Timestamp,
    );
    type Error = TeamError;

    fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    async fn find_by_id(&self, id: i32) -> TeamResult<Option<TeamRepo>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let result = team_repos::table
            .filter(team_repos::id.eq(id))
            .first::<TeamRepo>(conn)
            .await
            .optional()
            .map_err(|e| {
                TeamError::DatabaseError(format!("Failed to get team repo by ID: {}", e))
            })?;

        Ok(result)
    }

    async fn find_all(&self) -> TeamResult<Vec<TeamRepo>> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let repos = team_repos::table
            .load::<TeamRepo>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list team repos: {}", e)))?;

        Ok(repos)
    }

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> TeamResult<(Vec<TeamRepo>, i64)> {
        let conn = &mut self.pool.get().await.map_err(|e| {
            TeamError::DatabaseError(format!("Failed to get database connection: {}", e))
        })?;

        let offset = (params.page - 1) * params.per_page;

        let total = team_repos::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to count team repos: {}", e)))?;

        let repos = team_repos::table
            .order(team_repos::id.asc())
            .offset(offset)
            .limit(params.per_page)
            .load::<TeamRepo>(conn)
            .await
            .map_err(|e| TeamError::DatabaseError(format!("Failed to list team repos: {}", e)))?;

        Ok((repos, total))
    }
}
