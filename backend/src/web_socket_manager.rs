use std::sync::Arc;

use axum::{extract::{ws::WebSocket, State, WebSocketUpgrade}, response::Response};
use axum_session::{Session, SessionPgPool};
use uuid::Uuid;

use crate::server_state::ServerState;

pub async fn handler (ws: WebSocketUpgrade, session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>) -> Response {
    let id = session.get::<Uuid>("user_id").unwrap();
    ws.on_upgrade(move |socket| handle_socket(socket, state, id))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>, id: Uuid) {
    state.add_client(socket, &id).await;
}
