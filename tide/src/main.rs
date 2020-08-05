//extern crate db;

pub use common::models::User;
pub use db;
pub use tide;

#[async_std::main]
async fn main() -> async_std::io::Result<()> {

    let app = tide::new();
    let state = AppState { data: "Data".to_string() };

    let db_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = db::connect(&db_url).await.unwrap();

    db::clear(&pool).await.unwrap();
    db::up(&pool).await.unwrap();

    app.listen("127.0.0.1:8080").await

}

pub struct AppState {
    data: String,
}
