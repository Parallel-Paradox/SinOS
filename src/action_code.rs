mod game_room;
use std::sync::Arc;

use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{WebSocket, Message};
use axum::routing::get;
use axum::{Json, Router, Extension, TypedHeader};
use axum::response::IntoResponse;
use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ServerAddress};
use nanoid::nanoid;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::config::mongo_credential::action_code;
pub use game_room::*;

pub fn create_app() -> Router {
    let db = Arc::new(connect_mongodb());
    let rp = Arc::new(RoomMap::new());

    let app = Router::new()
        .route("/connect", get(connect_ws));

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

async fn connect_ws(
    ws: WebSocketUpgrade, user_agent: Option<TypedHeader<headers::UserAgent>>,
    Extension(rp): Extension<Arc<RoomMap>>, Extension(db): Extension<Arc<Database>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::debug!("Try to connect with '{}'", user_agent.as_str());
    }

    ws.on_upgrade(|socket: WebSocket| async move {
        let session_id = nanoid!();     // alloc a session id for this connection.
        connection_loop(socket, session_id, rp, db).await;
    })
}

async fn connection_loop(mut socket: WebSocket, sid: String, rp: Arc<RoomMap>, db: Arc<Database>)
{
    // ----- Connection Status -----
    let mut _token: Option<Token> = None;

    while let Some(msg) = socket.recv().await {
        let cmd: Command;
        if let Ok(msg) = msg {
            match Command::try_from(msg) {
                Ok(cmd_) => { cmd = cmd_; },
                Err(_) => { continue; },
            }
        } else {
            tracing::warn!("Receive fail!");
            break;
        }

        match cmd {
            Command::Close => { break; },
            Command::NewRoom { owner_name } => {
                let token = match new_room(&mut socket, owner_name, rp.clone()).await {
                    Ok(token) => token,
                    Err(_) => { continue; }
                };
                _token = Some(token);
            },
            Command::EnterRoom { room_id, player_name, } => {
                let token = match enter_room(
                    &mut socket, room_id, player_name, rp.clone()).await
                {
                    Ok(token) => token,
                    Err(_) => { continue; }
                };
                _token = Some(token);
            },
            Command::StartGame => {
                // TODO Start Game
            },
        }
    }

    if let Some(token) = _token {
        rp.remove_player(token.room_id, token.player_id).unwrap();
    }

    tracing::debug!("Sid-{}- connection closed.", sid);
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Close,
    NewRoom { owner_name: String },
    EnterRoom { room_id: RoomID, player_name: String, },  // Auto leave when disconnected.

    // Only available for the owner of game room
    StartGame,
}

/// See [Structs and enums in JSON](https://serde.rs/json.html)
impl TryFrom<Message> for Command {
    type Error = ();

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        let cmd: Command;
        match msg {
            Message::Close(_) => { cmd = Command::Close; },
            Message::Text(src) => {
                let res = serde_json::from_str(&src);
                if res.is_err() {
                    tracing::error!("Parse fail! - {}", src);
                    return Err(());
                }
                cmd = res.unwrap();
            },
            _ => {
                tracing::error!("Unexpected message!");
                return Err(());
            },
        }

        Ok(cmd)
    }
}

async fn new_room(socket: &mut WebSocket, owner_name: String, rp: Arc<RoomMap>)
    -> Result<Token, ()>
{
    let owner = Player::new(RoomID::new(), owner_name);
    let token = owner.token.clone();
    let room = GameRoom::new(owner);

    // Make sure that game room has been inserted before notice the client. If fail to notice,
    // convert the operation.
    rp.insert(room);

    let msg = Message::Text(
        format!( "{}", json!(token.clone()) )
    );
    if socket.send(msg).await.is_err() {
        rp.remove(token.room_id);
        return Err(());
    }

    Ok(token)
}

async fn enter_room(socket: &mut WebSocket, room_id: RoomID, player_name: String, rp: Arc<RoomMap>)
    -> Result<Token, ()>
{
    let player = Player::new(room_id, player_name);
    let token = player.token.clone();

    if let Some(err) = rp.insert_player(room_id, player).err() {
        tracing::error!("New Player insert fail {:?}.", token.clone());
        let msg = Message::Text(
            format!("{} - Object not found.", err)
        );
        if let Err(err) = socket.send(msg).await {
            tracing::error!("Fail to send msg: {}", err);
        }
    }

    let msg = Message::Text(
        format!( "{}", json!(token.clone()) )
    );
    if socket.send(msg).await.is_err() {
        rp.remove_player(room_id, token.player_id).unwrap();
        return Err(());
    }

    Ok(token)
}

/// Do some test for unstable API.
#[cfg(debug_assertions)]
async fn experiment(
    Extension(db): Extension<Arc<Database>>,
) -> impl IntoResponse {
    // TODO save words into room
    Json(get_random_words(db, 25).await.unwrap())
}
