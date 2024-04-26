use crate::database;
use crate::database::users_table::{AuthValidationResult, User};
use crate::database::user_relations_table::{RelationAndUser, UserRelationPair, UserRelation};
use crate::server_state::ServerState;
use crate::structs::chatty_response::{chatty_json_response, ChattyResponse};
use crate::structs::checked_string::CheckedString;
use crate::structs::notification::Notification;
use crate::structs::socket_signal::Signal;
use axum::extract::{Json, Path, State};
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use axum_session::{Session, SessionPgPool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageJSON {
    attachments: Vec<String>,
    author_id: String,
    channel_id: String,
    content: String,
}

pub async fn post_message( State(_state): State<Arc<ServerState>>, Json(_payload): Json<MessageJSON>,) -> ChattyResponse {
    ChattyResponse::Ok
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendRequestForm {
    to: CheckedString,
}

#[debug_handler()]
pub async fn send_friend_request( session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>, Json(payload): Json<FriendRequestForm>,) -> ChattyResponse {
    let Some(sender) = session.get::<Uuid>("user_id") else {
        return ChattyResponse::Unauthorized;
    };

    let query = User::query_user_by_username(&payload.to);

    match query {
        Some(target) => {
            let Some(req_id) = UserRelation::create(
                UserRelationPair::new(&sender, &target.id),
                &sender,
            ) else {
                return ChattyResponse::InternalError;
            };

            let n = Notification::new_friend_req(&target.id, &sender, &req_id);

            if let Some(live_target) = state.get_client(&target.id).await {
                live_target
                    .send_socket_order(Arc::new([Signal::Notification(n)]))
                    .await;
            }
            // TODO: store the notification.
            ChattyResponse::Ok
        }
        _ => ChattyResponse::BadRequest(Some(String::from("User does not exist"))),
    }
}

pub async fn initial_data_request(session: Session<SessionPgPool>) -> Response {
    let Some(target) = session.get::<Uuid>("user_id") else {
        return ChattyResponse::Unauthorized.into_response();
    };
    let Some(relations) = UserRelation::query_user_relations(&target) else {
        return ChattyResponse::InternalError.into_response();
    };
    let Some(user_info) = User::query_user(&target) else {
        return ChattyResponse::InternalError.into_response();
    };
    #[derive(Serialize)]
    struct CompleteResult {
        relations: Vec<RelationAndUser>,
        user_info: User,
    }
    let complete_result = CompleteResult {
        relations, user_info
    };
    return chatty_json_response(complete_result);
}

#[derive(Deserialize)]
pub struct FriendshipInteraction {
    relation_id: Uuid,
}


use database::user_relations_table::EditFriendshipEnum;
pub async fn edit_relation(Path(action): Path<EditFriendshipEnum>, session: Session<SessionPgPool>, Json(form): Json<FriendshipInteraction>) -> ChattyResponse {
    let Some(request_maker) = session.get::<Uuid>("user_id") else {
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
    let res = User::create_user(&form.username, &form.password);
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
