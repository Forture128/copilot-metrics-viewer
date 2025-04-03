use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::prelude::*;

use crate::common::types::PaginationParams;
use crate::infrastructure::db::DbPool;

pub trait Entity {
    type Id;
    fn get_id(&self) -> Self::Id;
}

pub trait Timestamps {
    fn get_created_at(&self) -> NaiveDateTime;
    fn get_updated_at(&self) -> NaiveDateTime;
    fn set_updated_at(&mut self, time: NaiveDateTime);
}

#[async_trait]
#[allow(unused)]
pub trait BaseRepository {
    type Entity: Send + Entity + Queryable<Self::SqlType, Pg>;
    type SqlType;
    type Error;

    fn get_pool(&self) -> &DbPool;

    async fn find_by_id(
        &self,
        id: <Self::Entity as Entity>::Id,
    ) -> std::result::Result<Option<Self::Entity>, Self::Error>;

    async fn find_all(&self) -> std::result::Result<Vec<Self::Entity>, Self::Error>;

    async fn find_with_pagination(
        &self,
        params: &PaginationParams,
    ) -> std::result::Result<(Vec<Self::Entity>, i64), Self::Error>;
}
