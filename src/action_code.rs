mod game_room;
use std::sync::Arc;

use axum::routing::get;
use axum::{Json, Router, Extension};
use axum::response::IntoResponse;
pub use game_room::*;

pub fn create_app() -> Router {
    let db = Arc::new(connect_mongodb());

    Router::new()
        .route("/new_game_room", get(new_game_room))
        .route("/test", get(test))
        .layer(Extension(db))
}

pub fn connect_mongodb() -> mongodb::Database {
    use mongodb::Client;
    use mongodb::options::{ClientOptions, ServerAddress};
    use crate::config::mongo_credential::action_code;

    let mut client_option = ClientOptions::default();
    client_option.app_name = Some("ActionCode".into());
    client_option.hosts = vec![ServerAddress::Tcp {
        host: "localhost".to_string(),
        port: Some(27017),
    }];
    client_option.credential = Some(action_code().unwrap());

    Client::with_options(client_option).unwrap().database("action_code")
}

async fn new_game_room() -> impl IntoResponse {
    let game_room = GameRoom::default();
    println!("{:?}", game_room);

    Json(game_room)
}

async fn test(Extension(db): Extension<Arc<mongodb::Database>>) -> impl IntoResponse {
    Json(get_random_words(db, 25).await.unwrap())
}
