//extern crate db;

pub use common::models::User;
use tide::log::LogMiddleware;
use tide::sessions::SessionMiddleware;
use tide::http::headers::HeaderValue;
use tide::utils::async_trait;
use tide::security::{Origin, CorsMiddleware};

pub use db;
pub use tide::{
    http::Cookie,
    Response, StatusCode
};

#[async_std::main]
async fn main() -> async_std::io::Result<()> {

    let state = AppState { data: "Data".to_string() };

    let db_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = db::connect(&db_url).await.unwrap();

    db::clear(&pool).await.unwrap();
    db::up(&pool).await.unwrap();

    let mut app = tide::with_state(state);
    //app.with(SessionMiddleware::with_cookie_path("/").with_cookie_name;
    app.with(CorsMiddleware::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:5000")
        .allow_origin("http://localhost:5001"));

    app.with(LogMiddleware::new());

    app.at("/").get(|_| async move { Ok("hello world!") });

    app.at("/auth/signup").get(|_| async move {
        Ok(Response::new(StatusCode::Ok))
    });

    app.at("/auth/login").get(|_| async move {
        let cookie = Cookie::new("Auth", "Token");
        let mut resp = Response::new(StatusCode::Ok);
        resp.insert_cookie(cookie); 
        Ok(resp)
    });

    app.at("/user").post(|_| async move {
        let body = "";
        Ok(Response::new(StatusCode::Ok))
    });

    app.at("/user/:username").get(|req| async move {
        let res = Response::new(200);
        Ok(Response::new(200))
    });

    app.at("/index").get(|req: tide::Request<AppState>| async move { 
        Ok (req.resp()) 
    });

    app.at("/usertest").get(|mut req: tide::Request<AppState>| async move {
       let user: User = req.body_json().await?;
       println!("user is {}", user.username);
       let mut res = Response::new(200);
       res.set_body(tide::Body::from_json(&user)?);
       Ok(res)
    });

    let user = User::new("keewa@div.is".to_string(), "keewa".to_string(), "d".to_string());
    let chris = User::new("chris@div.is".to_string(), "chris".to_string(), "p".to_string());

    user.insert(&pool).await.unwrap();
    chris.insert(&pool).await.unwrap();

    let test = User::get_by_username(&pool, "chris").await.unwrap();
    let test2 = User::get_all(&pool).await.unwrap();

    println!("User: {}", serde_json::to_string(&test).unwrap());
    println!("Users: {}", serde_json::to_string(&test2).unwrap());

    app.listen("127.0.0.1:3002").await



}

pub trait RequestExt {
    fn resp(&self) -> String;
}

impl<AppState> RequestExt for tide::Request<AppState> {
    fn resp(&self) -> String {
        "response".to_string()
    }
}

#[derive(Clone)]
pub struct AppState {
    data: String,
}
