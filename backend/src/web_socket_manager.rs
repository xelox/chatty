use std::sync::Arc;

use axum::{extract::{ws::WebSocket, State, WebSocketUpgrade}, response::Response};
use axum_session::{Session, SessionPgPool};

use crate::{server_state::ServerState, structs::id::ChattyId};

pub async fn handler (ws: WebSocketUpgrade, session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>) -> Response {
    let id = session.get::<ChattyId>("user_id").unwrap();
    ws.on_upgrade(move |socket| handle_socket(socket, state, id))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>, id: ChattyId) {
    state.add_client(socket, &id).await;
}
