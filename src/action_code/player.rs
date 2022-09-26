//! To create a new game room, you need to create a [`Token`] first, then create a [`Player`], and
//! finally create a [`super::GameRoom`] with this player as its owner.

use nanoid::nanoid;
use serde::{Serialize, Deserialize};

use super::RoomID;

/// The client should have full access to the game resources with its token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub player_id: String,
    pub room_id: RoomID,
}

impl Token {
    pub fn new() -> Self { Self::default() }
}

impl Default for Token {
    fn default() -> Self {
        Self { player_id: nanoid!(), room_id: RoomID::new() }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub token: Token,
    pub name: String,
}

impl Player {
    pub fn new(token: Token) -> Self {
        Self { token, name: "Anonymous".to_owned() }
    }
}
