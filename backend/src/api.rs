use crate::database::channel_subscribers_table::ChannelSubscribers;
use crate::database::message_table::{Message, NewMessage};
use crate::database::users_table::{AuthValidationResult, User};
use crate::database::user_relations_table::{RelationAndUser, UserRelationPair, UserRelation};
use crate::server_state::ServerState;
use crate::structs::chatty_response::{chatty_json_response, ChattyResponse};
use crate::structs::checked_string::CheckedString;
use crate::structs::notification::Notification;
use crate::structs::socket_signal::Signal;
use crate::structs::ts::TimeStamp;
use axum::extract::{Json, Path, State};
use axum::response::{IntoResponse, Response};
use axum_session::{Session, SessionPgPool};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::structs::id::ChattyId;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendRequestForm {
    to: CheckedString,
}

pub async fn send_friend_request( session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>, Json(payload): Json<FriendRequestForm>,) -> ChattyResponse {
    let Some(sender_id) = session.get::<ChattyId>("user_id") else {
        return ChattyResponse::Unauthorized;
    };

    match User::query_user_by_username(&payload.to) {
        Some(target) => {
            let Some(req_id) = UserRelation::create(
                UserRelationPair::new(sender_id, target.id),
                &sender_id,
            ).await else {
                return ChattyResponse::InternalError;
            };

            let n = Notification::new_friend_req(&target.id, &sender_id, &req_id).await;

            if let Some(live_target) = state.get_client(&target.id).await {
                live_target
                    .send_socket_order(Arc::new([Signal::Notification(n)]))
                    .await;
            }
            ChattyResponse::Ok
        }
        _ => ChattyResponse::BadRequest(Some(String::from("User does not exist"))),
    }
}

pub async fn initial_data_request(session: Session<SessionPgPool>) -> Response {
    let Some(id) = session.get::<ChattyId>("user_id") else {
        return ChattyResponse::Unauthorized.into_response();
    };
    let Some(relations) = UserRelation::query_user_relations(&id) else {
        return ChattyResponse::InternalError.into_response();
    };
    let Some(user_info) = User::query_user(&id) else {
        return ChattyResponse::InternalError.into_response();
    };
    #[derive(Serialize, Debug)]
    struct CompleteResult {
        relations: Vec<RelationAndUser>,
        user_info: User,
    }
    let complete_result = CompleteResult {
        relations, user_info
    };
    return chatty_json_response(complete_result);
}

pub async fn load_messages(session: Session<SessionPgPool>, Path((channel_id, ts)): Path<(ChattyId, TimeStamp)>) -> Response {
    let Some(channels) = session.get::<Vec<ChattyId>>("channels") else {
        return ChattyResponse::InternalError.into_response();
    };

    if channels.binary_search(&channel_id).is_err() {
        return ChattyResponse::Unauthorized.into_response();
        // TODO: Normalized permission validation as a middleware. 
    };

    let result = Message::load_from_ts(&channel_id, &ts);

    match result {
        Some(messages) => chatty_json_response(messages),
        None => ChattyResponse::InternalError.into_response()
    }
}

#[derive(Deserialize)]
pub struct FriendshipInteraction {
    relation_id: ChattyId,
}


use crate::database::user_relations_table::EditFriendshipEnum;
pub async fn edit_relation(Path(action): Path<EditFriendshipEnum>, session: Session<SessionPgPool>, Json(form): Json<FriendshipInteraction>) -> ChattyResponse {
    let Some(request_maker) = session.get::<ChattyId>("user_id") else {
        return ChattyResponse::Unauthorized;
    };
    UserRelation::edit_relation(&request_maker, form.relation_id, action)
    // TODO: If the operation returns Ok, also send socket message to live "other" client.
}

#[derive(Deserialize)]
pub struct AuthForm {
    username: CheckedString,
    password: String,
}
pub async fn signup(session: Session<SessionPgPool>, Json(form): Json<AuthForm>) -> ChattyResponse {
    let res = User::create_user(&form.username, &form.password).await;
    if let Some(id) = res {
        session.set_store(true);
        session.set("user_id", id);
        ChattyResponse::Ok
    } else {
        ChattyResponse::BadRequest(Some(String::from("Username is taken")))
    }
}

pub async fn signin(session: Session<SessionPgPool>, Json(form): Json<AuthForm>) -> ChattyResponse {
    let result = User::validate_password(&form.username, &form.password);
    match result {
        AuthValidationResult::Valid(id) => {
            session.set_store(true);
            session.set("user_id", id);
            let query = ChannelSubscribers::sorded_subscribed_channels(&id);
            let Some(channels) = query else {
                return ChattyResponse::InternalError;
            };
            session.set("channels", channels);
            ChattyResponse::Ok
        }
        AuthValidationResult::IncorrectPassword => {
            ChattyResponse::BadRequest(Some(String::from("Incorrect password")))
        }
        AuthValidationResult::IncorrectUniqueName => {
            ChattyResponse::BadRequest(Some(String::from("Incorrect username")))
        }
    }
}

pub async fn logout(session: Session<SessionPgPool>) -> ChattyResponse {
    session.clear();
    ChattyResponse::Ok
}
