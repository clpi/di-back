use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use sqlx::postgres::*;
use sqlx::FromRow;
use sqlx::prelude::*;
use sqlx::postgres::{PgPool, PgConnection};


pub struct Db {
    pool: PgPool, 
}

impl Db {
    pub async fn new(db_url: &str) -> sqlx::Result<Self> {
        let pool = PgPool::new(&db_url).await?;
        Ok ( Db { pool } )
    }

    //pub async fn listen(self) -> sqlx::Result<()> {
        //PgListener::from_pool(self)
            //.listen_all()?;
    //}

    pub async fn query(table: &str) -> () {  }
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
