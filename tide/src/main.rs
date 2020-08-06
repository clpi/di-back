//extern crate db;

pub use common::models::User;
use tide::log::LogMiddleware;
use tide::sessions::SessionMiddleware;
use tide::http::headers::HeaderValue;
use tide::utils::async_trait;
use tide::security::{Origin, CorsMiddleware};
use db::PgPool;

pub use db;
pub use tide::{
    http::Cookie,
    Response, StatusCode, Request
};

#[async_std::main]
async fn main() -> async_std::io::Result<()> {

    tide::log::start();

    let db_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = db::connect(&db_url).await.unwrap();

    let state = AppState { 
        data: "Data".to_string(), 
        pool: pool.clone() };

    db::clear(&pool).await.unwrap();
    db::up(&pool).await.unwrap();

    let mut app = tide::with_state(state);
    //app.with(SessionMiddleware::with_cookie_path("/").with_cookie_name;
    app.with(CorsMiddleware::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:5000")
        .allow_origin("http://localhost:5001"));

    app.with(LogMiddleware::new());

    app.with(SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        std::env::var("SECRET_KEY").expect("Must be 32 byte key").as_bytes(),
    ));

    app.with(tide::utils::Before(
        |mut request: tide::Request<AppState>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));
    

    app.at("/").get(|mut req: tide::Request<AppState>| async move {
        Ok("hello world!") 
    });

    app.at("/auth/login").post(|mut req: tide::Request<AppState>| async move {
        let user: User = req.body_json().await.unwrap();
        let mut resp = Response::new(StatusCode::Ok);
        let pool = req.state().pool.clone();
        match User::get_by_username(pool, user.username).await {
            Ok(dbuser) =>
                if dbuser.password == user.password {
                    resp.insert_header("Login", "true");
                    resp.set_body(format!("Signed in {}", dbuser.username));
                    resp.insert_cookie(Cookie::new("auth", "token"));            
                } else {
                    resp.insert_header("Login", "false");
                    resp.set_body("Incorrect credentials");
                }
            Err(_) =>  {
                resp.insert_header("Login", "false"); 
                resp.set_body("Incorrect credentials");
            } 
        }
        Ok(resp)
    });

    app.at("/auth/signup").post(|mut req: Request<AppState>| async move {
        let user: User = req.body_json().await.unwrap();
        let mut resp = Response::new(StatusCode::Ok);
        let pool = req.state().pool.clone();
        match user.insert(pool).await {
            Ok(_) => {
                let mut resp = Response::new(StatusCode::Ok);
                resp.insert_header("Register", "true"); //TODO actually implement real resp
                resp.set_body("Successfully signed up");
            },
            Err(_) =>  {
                let mut resp = Response::new(StatusCode::Unauthorized);
                resp.insert_header("Register", "false"); 
                resp.set_body("Could not sign up");
            },
        }
        Ok(resp)
    });

    app.at("/user").post(|mut req: Request<AppState>| async move {
        let user: User = req.body_json().await.unwrap();
        Ok(Response::new(StatusCode::Ok))
    }).get(|req: Request<AppState>| async move {
        match User::get_all(req.state().pool.clone()).await {
            Ok(users) => {
                let mut resp = Response::new(StatusCode::Ok);
                resp.set_body(serde_json::to_string(&users).unwrap());
                Ok(resp)
            },
            Err(_) => {
                let mut resp = Response::new(StatusCode::BadRequest);
                resp.set_body("Could not fetch users");
                Ok(resp)
            }
        }
    });

    app.at("/user/:username").get(|mut req: Request<AppState>| async move {
        let res = Response::new(200);
        let username: String = req.param("username").unwrap();
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
    let test = User::new("test@div.is".to_string(), "test".to_string(), "m".to_string());

    user.insert(pool.clone()).await.unwrap();
    chris.insert(pool.clone()).await.unwrap();
    test.insert(pool.clone()).await.unwrap();

    let get_chris = User::get_by_username(pool.clone(), "chris".to_string()).await.unwrap();
    let get_all = User::get_all(pool).await.unwrap();

    println!("User: {}", serde_json::to_string(&get_chris).unwrap());
    println!("Users: {}", serde_json::to_string(&get_all).unwrap());

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
    pool: PgPool,
}
