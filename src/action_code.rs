mod game_room;
mod player;
use std::sync::Arc;

use axum::routing::get;
use axum::{Json, Router, Extension};
use axum::response::IntoResponse;
use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ServerAddress};
use crate::config::mongo_credential::action_code;
pub use game_room::*;

pub fn create_app() -> Router {
    let db = Arc::new(connect_mongodb());
    let rp = Arc::new(RoomMap::new());

    let app = Router::new()
        .route("/new_game_room", get(new_game_room));

    if cfg!(debug_assertions) {
        // Do some test for unstable API.
        app.route("/experiment", get(experiment))
    } else { app }
        .layer(Extension(db)).layer(Extension(rp))
}

pub fn connect_mongodb() -> Database {
    let mut client_option = ClientOptions::default();
    client_option.app_name = Some("ActionCode".into());
    client_option.hosts = vec![ServerAddress::Tcp {
        host: "localhost".to_string(),
        port: Some(27017),
    }];
    client_option.credential = Some(action_code().unwrap());

    Client::with_options(client_option).unwrap().database("action_code")
}

/// Create a new [`GameRoom`], return the connection [`player::Token`].
async fn new_game_room(
    Extension(rp): Extension<Arc<RoomMap>>,
) -> impl IntoResponse {
    let token = rp.insert_empty();
    tracing::debug!("Insert an empty game room - {:?}", token);

    Json(token)
}

/// Do some test for unstable API.
#[cfg(debug_assertions)]
async fn experiment(
    Extension(db): Extension<Arc<Database>>,
) -> impl IntoResponse
{
    // TODO save words into room
    Json(get_random_words(db, 25).await.unwrap())
}
