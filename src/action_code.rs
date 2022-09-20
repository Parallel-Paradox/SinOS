use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::{Serialize, Deserialize};

mod room_id {
    use nanoid::nanoid;
    use serde::{Serialize, Deserialize};
    use crate::constant::NUM_ALPHABET;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoomID(String);

    impl RoomID {
        pub fn new() -> Self { RoomID::default() }
        pub fn get(&self) -> &str { &self.0 }
    }
    
    impl Default for RoomID {
        fn default() -> Self { Self(nanoid!(7, &NUM_ALPHABET)) }
    }
}
pub use room_id::*;

pub fn create_app() -> Router {
    Router::new()
        .route("/new_game_room", get(new_game_room))
}

#[derive(Debug, Serialize, Deserialize)]
struct GameRoom {
    pub room_id: RoomID,
}

async fn new_game_room() -> impl IntoResponse {
    let room_id = RoomID::new();
    let game_room = GameRoom { room_id };
    println!("{}", game_room.room_id.get());

    Json(game_room)
}
