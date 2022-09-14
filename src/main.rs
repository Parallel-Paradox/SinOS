mod service;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::Path;
use axum::response::Html;
use axum::{Extension, Router};
use axum::routing::get;
use tokio::sync::mpsc;
use tracing::Level;

use crate::service::admin::{Admin, AdminExt, AdminMsg};
use crate::service::ServiceMsg::CloseService;

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

async fn get_user(Path(user_id): Path<String>, Extension(sender): AdminExt) {
    println!("{user_id}");
    sender.clone().send(CloseService).await.unwrap();
}

fn test_shared_state() -> Router {
    let shared_state = Arc::new(Admin::start(32));

    // build application with route
    Router::new()
        .route("/", get(index))
        .route("/users/:id", get(get_user))
        .layer(Extension(shared_state))
}
