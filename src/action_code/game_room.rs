use nanoid::nanoid;
use serde::{Serialize, Deserialize};
use crate::config::NUM_ALPHABET;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomID(String);

impl RoomID {
    pub fn new() -> Self { Self::default() }
    pub fn get(&self) -> &str { &self.0 }
}

impl Default for RoomID {
    fn default() -> Self { Self(nanoid!(7, &NUM_ALPHABET)) }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_id: RoomID,
    word_list: Vec<String>,
}

impl Default for GameRoom {
    fn default() -> Self {
        Self { room_id: RoomID::new(), word_list: Vec::new(), }
    }
}

/// Get 25 words from the given mongo client.
pub fn get_words_from_mongo() -> Vec<String> {
    // TODO get data from mongo db.

    Vec::new()
}
