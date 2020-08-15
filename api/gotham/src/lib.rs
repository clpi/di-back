mod handlers;

use macros::*;
use gotham::{
    state::State,
    handler::{Handler, NewHandler, IntoHandler, IntoHandlerFuture, HandlerFuture},
    router::{Router, builder::*},
};

#[test]
pub fn test() {
    println!("Hello, world!");
}

#[tokio::main]
pub async fn main() -> tokio::io::Result<()> {
    let address = "127.0.0.1:7070";
    gotham::start(address, routes::routes());

    Ok(())
}

pub mod routes {
    use super::*;

    pub fn routes() -> Router {
        build_simple_router(|route| {
            route.get_or_head("/")
                .to(index);
            route.scope("/user", |route| {
                route.get("/")
                    .to(index);
                route.post("/")
                    .to(index);
            });
        })
    }

    create_handler!(index);

    pub mod user {
        use super::*;

        create_handler!(index);

    }

    pub mod record {
        use super::*;

        create_handler!(index);

    }

    pub mod item {
        use super::*;

        create_handler!(index);
    }

}

pub trait ApiHandler {

}

pub trait Model {
    fn table() -> String;

    fn from_id() -> Self;

}

#[macro_export]
pub mod macros {

    #[macro_export]
    macro_rules! create_handler {
        ($($t:ident),*) => { $(
            pub fn $t(state: State) -> (State, &'static str) {
                (state, stringify!($t))
            }
        )+ }
    }

    #[macro_export]
    macro_rules! create_route {
        ($($x:expr),*) => { $(

        )+}
    }

    #[macro_export]
    macro_rules! api_def {
        ($($x:expr),*) => { $(

        )+}
    }

    #[macro_export]
    macro_rules! impl_model {
        ($($x:ty, $name:ident),*)=> { $(
            impl Model for $x {
                fn table() -> String { stringify!($name).to_string() }

                fn from_id() -> Self { Self::new() }
            }
        )+}
    }
}


#[cfg(test)]
pub mod test {

    use super::*;
    use gotham::test::{TestRequest, TestClient, TestServer};

    #[test]
    pub fn get_index() {
        let test_srv = TestServer::new(routes::routes()).unwrap();
        let res = test_srv.client()
            .get("http://localhost:7070")
            .perform()
            .unwrap();

        let body = res.read_body().unwrap();

    }

}
