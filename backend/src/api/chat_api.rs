use std::sync::Arc;

use axum::{extract::{Path, State}, response::{IntoResponse, Response}, Json};
use axum_macros::debug_handler;
use axum_session::{Session, SessionPgPool};

use crate::{
    database::{channel_subscribers_table::ChannelSubscribers, channel_table::ChannelTable, message_table::{ExistingMessage, Message, MessageOperations, NewMessage}}, 
    server_state::ServerState, 
    structs::{
        chatty_response::{chatty_json_response, ChattyResponse}, 
        id::ChattyId,
        ts::TimeStamp
    }
};

pub async fn send_message(session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>, Json(payload): Json<NewMessage>) -> ChattyResponse {
    let Some(allowed_channels) = session.get::<Vec<ChattyId>>("channels") else {
        println!("No channels set");
        return ChattyResponse::InternalError;
    };
    if allowed_channels.binary_search(&payload.channel_id).is_err() {
        let query = ChannelSubscribers::sorded_subscribed_channels(&payload.sender_id);
        let Some(allwed_channels) = query else {
            return ChattyResponse::InternalError;
        };
        session.set("channels", &allwed_channels);
        if allwed_channels.binary_search(&payload.channel_id).is_err() {
            return ChattyResponse::Unauthorized;
        }
    }

    let message = Message::store(&payload).await;
    match message {
        Some(message) => state.broadcast_message(message, MessageOperations::Send).await,
        None => ChattyResponse::InternalError
    }
}

pub async fn delete_message(State(state): State<Arc<ServerState>>, Json(input): Json<ExistingMessage>) -> ChattyResponse {
    let Some(message) = Message::delete(&input) else {
        return ChattyResponse::InternalError;
    };

    state.broadcast_message(message, MessageOperations::Delete).await
}

pub async fn edit_message() -> ChattyResponse {
    ChattyResponse::Ok
}

pub async fn load_messages(session: Session<SessionPgPool>, Path((channel_id, ts)): Path<(ChattyId, TimeStamp)>) -> Response {
    let Some(channels) = session.get::<Vec<ChattyId>>("channels") else {
        return ChattyResponse::InternalError.into_response();
    };

    if channels.binary_search(&channel_id).is_err() {
        return ChattyResponse::Unauthorized.into_response();
    };

    let result = Message::load_from_ts(&channel_id, &ts);

    match result {
        Some(messages) => chatty_json_response(messages),
        None => ChattyResponse::InternalError.into_response()
    }
}

pub async fn channel_info(Path(channel_id): Path<ChattyId>) -> Response {
    match ChannelTable::get_info(channel_id) {
        Some(channel) => chatty_json_response(channel),
        _ => ChattyResponse::InternalError.into_response()
    }
}

