use axum::Json;
use axum_session::{Session, SessionPgPool};
use serde::Deserialize;
use axum::response::IntoResponse;
use axum::extract::{Path, Multipart};
use crate::structs::socket_signal::Signal;
use crate::structs::notification::Notification;
use crate::database::user_relations_table::{UserRelationPair, RelationAndUser, UserRelation};
use crate::ServerState;
use std::sync::Arc;
use axum::extract::State;
use crate::database::users_table::ProfileDecoration;
use crate::structs::chatty_response::chatty_json_response;
use crate::structs::id::ChattyId;
use axum::response::Response;
use serde::Serialize;

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

pub async fn update_profile(session: Session<SessionPgPool>, mut form: Multipart) -> ChattyResponse {
    let Some(uid) = session.get::<ChattyId>("user_id") else {
        return ChattyResponse::InternalError;
    };
    
    let mut decorations = Vec::<ProfileDecoration>::new();
    while let Some(field) = form.next_field().await.unwrap() {
        let key = field.name().unwrap().to_string();
        // TODO: Use streams instead of bytes for performance benefits.
        match key.as_str() {
            "display_name" => {
                let str = field.text().await.unwrap();
                decorations.push(ProfileDecoration::DisplayName(str));
            },
            "pfp" => {
                let bytes = field.bytes().await.unwrap();
                decorations.push(ProfileDecoration::Pfp(bytes));          
            },
            "banner" => {
                let bytes = field.bytes().await.unwrap();
                decorations.push(ProfileDecoration::Banner(bytes));
            },
            "about_me" => {
                let str = field.text().await.unwrap();
                decorations.push(ProfileDecoration::AboutMe(str));
            },
            "custom_status" => {
                let str = field.text().await.unwrap();
                decorations.push(ProfileDecoration::Status(str));
            }
            _ => {}
        }
    }
    User::update_profile_decorations(decorations, uid)
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

#[derive(Deserialize)]
pub struct FriendshipInteraction {
    relation_id: ChattyId,
}

use crate::{database::{channel_subscribers_table::ChannelSubscribers, user_relations_table::EditFriendshipEnum, users_table::{AuthValidationResult, User}}, structs::{chatty_response::ChattyResponse, checked_string::CheckedString}};
pub async fn edit_relation(Path(action): Path<EditFriendshipEnum>, session: Session<SessionPgPool>, Json(form): Json<FriendshipInteraction>) -> ChattyResponse {
    let Some(request_maker) = session.get::<ChattyId>("user_id") else {
        return ChattyResponse::Unauthorized;
    };
    UserRelation::edit_relation(&request_maker, form.relation_id, action)
    // TODO: If the operation returns Ok, also send socket message to live "other" client.
}
