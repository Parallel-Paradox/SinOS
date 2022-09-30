use std::{sync::Arc, collections::HashMap, fmt::Display, hash::Hash};

use futures::StreamExt;
use mongodb::{Database, bson::{doc, Document, from_bson}};
use nanoid::nanoid;
use parking_lot::{RwLock, Mutex};
use serde::{Serialize, Deserialize};
use crate::util::{NUM_ALPHABET, Result, ErrCode};

/// Hold [`RwLock`] of the whole map and each entry in this map. Access the write lock of the whole
/// map only when create or delete a game room.
#[derive(Debug)]
pub struct RoomMap(RwLock<HashMap<RoomID, Mutex<GameRoom>>>);

impl RoomMap {
    pub fn new() -> Self { Self::default() }
    
    pub fn insert(&self, room: GameRoom) {
        let mut map = self.0.write();
        let id = room.room_id;
        tracing::debug!("Insert a game room - id: {}, owned by player {}.", id, &room.owner_id);
        map.insert(id, Mutex::new(room));
    }

    pub fn remove(&self, id: RoomID) {
        let mut map = self.0.write();
        map.remove(&id);
        tracing::debug!("Remove a game room - id: {}.", id);
    }

    pub fn insert_player(&self, id: RoomID, player: Player) -> Result<()> {
        let map = self.0.read();
        let mut room = match map.get(&id) {
            Some(_room) => _room,
            None => { return Err(ErrCode::ObjectNotExist); },
        }.lock();

        let player_id = player.token.player_id.clone();
        tracing::debug!("Player {} enter room {}.", player_id, id);
        room.player_list.insert(player_id, player);

        Ok(())
    }

    pub fn remove_player(&self, id: RoomID, player_id: String) -> Result<()> {
        let map = self.0.read();
        let mut room = match map.get(&id) {
            Some(_room) => _room,
            None => { return Err(ErrCode::ObjectNotExist); },
        }.lock();

        tracing::debug!("Player {} leave room {}.", player_id, id);
        if room.player_list.remove(&player_id).is_none() {
            return Err(ErrCode::ObjectNotExist);
        };

        if room.player_list.len() == 0 {
            drop(room);
            drop(map);
            self.remove(id);
            return Ok(());
        }

        if player_id == room.owner_id {
            let (new_owner_id, _) = room.player_list.iter().next().unwrap();
            room.owner_id = new_owner_id.to_owned();
        }

        Ok(())
    }
}

impl Default for RoomMap {
    fn default() -> Self { Self(RwLock::new(HashMap::new())) }
}


/// Access [`RoomMap`] by this. Copy trait implied.
#[derive(Debug, Serialize, Deserialize, Eq, Hash, Copy, Clone)]
pub struct RoomID(usize);

impl RoomID {
    pub fn new() -> Self { Self::default() }
}

impl Default for RoomID {
    fn default() -> Self { Self(nanoid!(7, &NUM_ALPHABET).parse().unwrap()) }
}

impl Display for RoomID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:07}", self.0)
    }
}

impl PartialEq for RoomID {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}


/// Save the context of a Game room. A GameRoom must have a [`Player`] as its owner.  
/// To create a new game room, you need to create a [`Token`] first, then create a [`Player`], and
/// finally create a [`super::GameRoom`] with this player as its owner.
/// TODO - Player Management.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_id: RoomID,
    owner_id: String,
    player_list: HashMap<String, Player>,
    word_list: Option<Vec<String>>,
}

impl GameRoom {
    pub fn new(owner: Player) -> Self {
        let room_id = owner.token.room_id;
        let owner_id = owner.token.player_id.clone();
        let player_list = [(owner_id.clone(), owner)].iter().cloned().collect();

        Self { room_id, owner_id, player_list, word_list: None }
    }
}

/// The client should have full access to the game resources with its token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub player_id: String,
    pub room_id: RoomID,
}

impl Default for Token {
    fn default() -> Self {
        Self { player_id: nanoid!(), room_id: RoomID::new() }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub token: Token,
    pub name: String,
}

impl Player {
    pub fn new(room_id: RoomID, name: String) -> Self {
        Self {
            token: Token { player_id: nanoid!(), room_id },
            name,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self { token: Token::default(), name: "Anonymous".to_owned() }
    }
}


/// Get 25 random words from the given mongo database.
pub async fn get_random_words(db: Arc<Database>, size: i32) -> Result<Vec<String>> {
    let collections = db.collection::<Document>("words");

    let pipeline = vec![
        doc! { "$sample": { "size": size } },
    ];
    
    let mut words: Vec<String> = Vec::new();
    let mut cursor = collections.aggregate(pipeline, None).await?;
    while let Some(result) = cursor.next().await {
        let bson = result?.get("word").unwrap().clone();
        let word: String = from_bson(bson).unwrap();
        words.push(word);
    }
    Ok(words)
}
