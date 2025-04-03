use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use super::base::{ActiveStatus, BaseModel, SoftDelete, Timestamps};
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(embed)]
    pub base: BaseModel,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub async fn find_by_email(
        conn: &mut diesel_async::AsyncPgConnection,
        email: &str,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        users
            .filter(deleted_at.is_null())
            .filter(email.eq(email))
            .first(conn)
            .await
            .optional()
    }

    pub async fn create(
        conn: &mut diesel_async::AsyncPgConnection,
        new_user: &NewUser<'_>,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(new_user)
            .get_result(conn)
            .await
    }
}

// Implement base traits for User
impl Timestamps for User {
    fn created_at(&self) -> DateTime<Utc> {
        self.base.created_at()
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.base.updated_at()
    }
}

impl ActiveStatus for User {
    fn is_active(&self) -> bool {
        self.base.is_active()
    }

    fn is_deleted(&self) -> bool {
        self.base.is_deleted()
    }
}

#[async_trait::async_trait]
impl SoftDelete for User {
    async fn soft_delete(&mut self) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;
        use diesel_async::RunQueryDsl;

        let conn = &mut diesel_async::AsyncPgConnection::establish("").await?;
        diesel::update(users.find(self.base.id))
            .set((deleted_at.eq(Some(Utc::now())), is_active.eq(false)))
            .execute(conn)
            .await?;

        self.base.deleted_at = Some(Utc::now());
        self.base.is_active = false;
        Ok(())
    }

    async fn restore(&mut self) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;
        use diesel_async::RunQueryDsl;

        let conn = &mut diesel_async::AsyncPgConnection::establish("").await?;
        diesel::update(users.find(self.base.id))
            .set((
                deleted_at.eq::<Option<DateTime<Utc>>>(None),
                is_active.eq(true),
            ))
            .execute(conn)
            .await?;

        self.base.deleted_at = None;
        self.base.is_active = true;
        Ok(())
    }
}
