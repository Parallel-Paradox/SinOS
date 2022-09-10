mod service;

use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
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

    // build application with route
    let app = Router::new().route("/", get(index));

    tokio::spawn(async move {
        let sender = service::action_code::test_send().await;
        sleep(Duration::from_secs(10)).await;
        sender.unwrap().send(service::action_code::Command::HelloWorld).await.unwrap();
    });

    // run app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {} ...", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn index() -> Html<&'static str> { Html(include_str!("../dist/index.html")) }
