use db::PgPool;

pub async fn create() -> tide::Result<Context> {
    let db_url = dotenv::var("DATABASE_URL").unwrap();
    let db = db::Db::new(db_url.as_str()).await.unwrap()
        .clear().await.unwrap()
        .up().await.unwrap();

    let state = Context { 
        data: "Data".to_string(), 
        pool: db.pool.clone() 
    };
    Ok(state)
}

#[derive(Clone)]
pub struct Context {
    pub data: String,
    pub pool: PgPool,
}
