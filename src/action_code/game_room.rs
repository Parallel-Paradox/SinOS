use std::sync::Arc;

use futures::StreamExt;
use mongodb::{Database, bson::{doc, Document, from_bson}};
use nanoid::nanoid;
use serde::{Serialize, Deserialize};
use crate::config::{NUM_ALPHABET, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomID(String);

impl RoomID {
    pub fn get(&self) -> &str { &self.0 }
}

impl Default for RoomID {
    fn default() -> Self { Self(nanoid!(7, &NUM_ALPHABET)) }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_id: RoomID,
    word_list: Option<Vec<String>>,
}

impl Default for GameRoom {
    fn default() -> Self {
        Self { room_id: RoomID::default(), word_list: None, }
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
