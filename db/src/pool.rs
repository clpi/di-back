use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use sqlx::postgres::*;
use sqlx::FromRow;
use sqlx::prelude::*;



#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: i32,
}

impl User {
    pub fn new(email: &str, username: &str, password: &str) -> User {
        User {
            id: None,
            email: email.to_string(), 
            username: username.to_string(), 
            password: password.to_string(),
            created_at: Utc::now().timestamp() as i32
        }
    }
}

pub async fn connect(db_url: &str) -> sqlx::Result<PgPool> {
    let pool = PgPool::new(&db_url).await.unwrap();
    Ok(pool)
}

pub async fn clear(pool: &PgPool) -> sqlx::Result<()> {
    sqlx::query!("DROP TABLE IF EXISTS RecordItemLinks CASCADE;")
        .execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS Items CASCADE;")
        .execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS Records CASCADE;")
        .execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS Users CASCADE;")
        .execute(pool).await?;
    Ok(())
}

pub async fn up(pool: &PgPool) -> sqlx::Result<()> {

    sqlx::query_file_unchecked!("sql/tables/users.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/records.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/fields.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/groups.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/items.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/entrytypes.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/userrecordlinks.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/recorditemlinks.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/itemfieldlinks.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/fieldentrylinks.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/usergrouplinks.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/userinfo.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/entryentries.sql")
        .execute(pool).await.unwrap();

    sqlx::query_file_unchecked!("sql/tables/fieldentries.sql")
        .execute(pool).await.unwrap();

    Ok(())
}
