use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use axum_session::{Session, SessionPgPool};

use crate::{database::{channel_subscribers_table::ChannelSubscribers, message_table::{Message, NewMessage}}, server_state::ServerState, structs::{chatty_response::ChattyResponse, id::ChattyId}};

pub async fn send_message(session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>, Json(mut payload): Json<NewMessage>) -> ChattyResponse {
    let Some(allowed_channels) = session.get::<Vec<ChattyId>>("channels") else {
        println!("No channels set");
        return ChattyResponse::InternalError;
    };
    if allowed_channels.binary_search(&payload.channel_id).is_err() {
        let query = ChannelSubscribers::sorded_subscribed_channels(&payload.sender_id);
        let Some(allwed_channels) = query else {
            return ChattyResponse::InternalError;
        };
        if allwed_channels.binary_search(&payload.channel_id).is_err() {
            return ChattyResponse::Unauthorized;
        }
        session.set("channels", allwed_channels);
    }

    let message = Message::store(&mut payload).await;
    match message {
        Some(message) => state.broadcast_message(message).await,
        None => ChattyResponse::InternalError
    }
}

pub async fn delete_message(Path(message_id): Path<ChattyId>) -> ChattyResponse {
    ChattyResponse::Ok
}
