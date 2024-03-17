use std::sync::Arc;

use axum::{extract::{ws::WebSocket, State, WebSocketUpgrade}, response::Response};

use crate::server_state::ServerState;

pub async fn handler (ws: WebSocketUpgrade, State(state): State<Arc<ServerState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    state.add_client(socket).await;
}
