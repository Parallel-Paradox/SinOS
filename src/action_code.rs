use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::{Router, TypedHeader};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::{Serialize, Deserialize};
use parking_lot::RwLock;
use crate::constant::NUM_ALPHABET;

mod room_id {
    use nanoid::nanoid;
    use serde::{Serialize, Deserialize};
    use crate::constant::NUM_ALPHABET;

    const ERR_WRONG_LENGTH: &str = "Access Room ID failed because of wrong length.";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoomID(String);

    impl RoomID {
        pub fn new(id: String) -> Result<Self, String> {
            match Self::check_length(&id) {
                Ok(_) => Ok(RoomID(id)),
                Err(err) => Err(err),
            }
        }

        pub fn get(&self) -> Result<&str, String> {
            match Self::check_length(&self.0) {
                Ok(_) => Ok(&self.0),
                Err(err) => Err(err),
            }
        }

        fn check_length(id: &str) -> Result<(), String> {
            if id.len() != 7 {
                tracing::error!("{}", ERR_WRONG_LENGTH);
                Err(ERR_WRONG_LENGTH.into())
            } else { Ok(()) }
        }
    }
    
    impl Default for RoomID {
        fn default() -> Self { Self::new(nanoid!(7, &NUM_ALPHABET)).unwrap() }
    }
}
pub use room_id::*;

pub fn create_app() -> Router {
    Router::new()
        .route("/new_game_room", get(new_game_room))
}

type GameRoomVec = RwLock<Vec<RwLock<GameRoom>>>;

fn new_async_vec<T>() -> RwLock<Vec<RwLock<T>>> { RwLock::new(Vec::new()) }

#[derive(Debug, Serialize, Deserialize)]
struct GameRoom {
    pub id: RoomID,
}

async fn new_game_room() -> impl IntoResponse {
    let room_id = RoomID::default();
    let game_room = GameRoom { id: room_id };
}
