use axum::extract::WebSocketUpgrade;
use axum::{Router, TypedHeader};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::get;

pub fn register(app: Router) -> Router {
    app.route("/ws/connect", get(ws_connect))
}

async fn ws_connect(
    ws: WebSocketUpgrade, user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    // ----- on connect -----
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::debug!("'{}' connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            tracing::debug!("Client says: {:?}", msg);
            // ----- on close -----
            if let Message::Close(_) = msg {
                println!("-------");
                break;
            }
            // ----- on upgrade -----
            if socket
                .send(Message::Text(format!("{:?}", msg))).await
                .is_err()
            {
                tracing::warn!("Client disconnected with message '{:?}' dropped!", msg);
                return;
            }
        } else {
            tracing::debug!("Client disconnected.");
            return;
        }
    }
}
