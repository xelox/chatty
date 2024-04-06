use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::server_state::ServerState;

#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub struct MessageJSON {
    attachments: Vec<String>,
    author_id: String,
    channel_id: String,
    content: String,
}

pub async fn post_message(State(state): State<Arc<ServerState>>, Json(payload): Json<MessageJSON>)  -> String {
    state.broadcast(serde_json::ser::to_string(&payload).unwrap()).await;
    "OK".to_string()
}
