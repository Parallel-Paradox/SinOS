mod action_code;
mod constant;

use std::net::SocketAddr;
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
    tracing::subscriber::set_global_default(subscriber).expect("Set default subscriber");

    // build application with router
    let app = create_app();

    // run app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {addr} ...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(index))
        .nest("/action_code", action_code::create_app())
}

async fn index() -> Html<&'static str> { Html(include_str!("../dist/index.html")) }
