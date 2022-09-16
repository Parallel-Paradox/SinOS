use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::{Router, TypedHeader};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::get;
use nanoid::nanoid;
use parking_lot::RwLock;
use crate::constant::NUM_ALPHABET;

pub fn create_app() -> Router {
    Router::new()
        .route("/new_game_room", get(new_game_room))
}

type GameRoomVec = RwLock<Vec<RwLock<GameRoom>>>;

fn new_async_vec<T>() -> RwLock<Vec<RwLock<T>>> { RwLock::new(Vec::new()) }

#[derive(Debug)]
struct GameRoom {
    pub room_id: String,
    pub owner_id: String,
    pub player_id_arr: Vec<String>,
}

async fn new_game_room() -> impl IntoResponse {
    let room_id = nanoid!(7, &NUM_ALPHABET);
    let owner_id = nanoid!();
}
