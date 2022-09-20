mod game_room;
pub use game_room::*;

use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use mongodb::Client;
use mongodb::options::{ClientOptions, Credential};
use crate::constant::MONGO_CREDENTIAL_ACTION_CODE;

pub fn create_app() -> Router {
    Router::new()
        .route("/new_game_room", get(new_game_room))
}

pub async fn create_mongo_client() -> Client {
    let mut client_option = ClientOptions::default();
    client_option.app_name = Some("ActionCode".into());
    client_option.credential = Some(MONGO_CREDENTIAL_ACTION_CODE.unwrap());

    Client::with_options(client_option).unwrap()
}

async fn new_game_room() -> impl IntoResponse {
    let game_room = GameRoom::default();
    println!("{:?}", game_room);

    Json(game_room)
}
