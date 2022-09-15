mod service;
mod constant;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::response::Html;
use axum::{Extension, Router};
use axum::routing::get;
use tracing::Level;

use crate::service::admin;
use crate::service::admin::{Admin, AdminCmd, AdminExt, AdminMsg};
use crate::constant::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    // build application with router
    let app = create_app();

    // run app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {addr} ...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn index() -> Html<&'static str> { Html(include_str!("../dist/index.html")) }

async fn echo(Extension(sender): AdminExt) {
    AdminMsg::Do(
        AdminCmd { command: admin::Command::Echo, rsp_sender: None }
    ).send(sender).await;
}

fn create_app() -> Router {
    // start admin service
    let shared_state =
        Arc::new(Admin::start(ADMIN_BUFFER_SIZE));

    Router::new()
        .route("/", get(index))
        .route("/echo", get(echo))
        .layer(Extension(shared_state))
}
