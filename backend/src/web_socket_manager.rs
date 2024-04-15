use std::sync::Arc;

use axum::{extract::{ws::WebSocket, State, WebSocketUpgrade}, response::Response};
use axum_session::{Session, SessionPgPool};

use crate::server_state::ServerState;

pub async fn handler (ws: WebSocketUpgrade, session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>) -> Response {
    let client_id = session.get("client_unique_name").unwrap();
    ws.on_upgrade(|socket| handle_socket(socket, state, client_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>, client_id: String) {
    state.add_client(socket, client_id).await;
}
