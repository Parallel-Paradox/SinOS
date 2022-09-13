mod service;

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::RwLock;
use axum::extract::Path;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use tracing::Level;

#[tokio::main]
async fn main() {
    // initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    let app = test_shared_state();

    // run app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {addr} ...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn index() -> Html<&'static str> { Html(include_str!("../dist/index.html")) }

#[derive(Debug)]
struct Test { num: i32 }

async fn get_user(Path(user_id): Path<String>, state: Arc<RwLock<Test>>) {
    let mut x = state.write().unwrap();
    x.num +=1 ;
    println!("{user_id} {x:?}");
}

fn test_shared_state() -> Router {
    let shared_state = Arc::new(RwLock::new(Test { num: 5 }));

    // build application with route
    Router::new()
        .route("/", get(index))
        .route(
            "/users/:id",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |path| get_user(path, shared_state)
            })
        )
}
