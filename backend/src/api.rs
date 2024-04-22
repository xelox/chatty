use axum::extract::{Json, State};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::database::models::{Friendship, FriendshipTargets};
use crate::server_state::ServerState;
use crate::database::{self, models::User, schema};
use crate::structs::chatty_response::ChattyResponse;
use crate::structs::checked_string::CheckedString;
use crate::structs::notification::Notification;
use crate::structs::socket_signal::Signal;
use axum_session::{Session, SessionPgPool};
use crate::structs::client::{ Client, AuthValidationResult };
use axum_macros::debug_handler;

#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub struct MessageJSON {
    attachments: Vec<String>,
    author_id: String,
    channel_id: String,
    content: String,
}

pub async fn post_message(State(_state): State<Arc<ServerState>>, Json(_payload): Json<MessageJSON>) -> impl IntoResponse {
    ChattyResponse::Ok
}


#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub struct FriendRequestForm {
    to: CheckedString
}

#[debug_handler()]
pub async fn send_friend_request(session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>, Json(payload): Json<FriendRequestForm>) -> impl IntoResponse {
    let Some(sender) = session.get::<CheckedString>("client_unique_name") else {
        return "Who are you?".to_string();
    };

    use schema::users::dsl::*;
    use diesel::prelude::*;
    let conn = &mut database::establish_connection();

    let query: Result<User, _> = users
        .find(payload.to)
        .first(conn);

    match query {
        Ok(target) => {
            let Some(req_id) = Friendship::create(FriendshipTargets::new(&sender, &target.unique_name), &sender) else {
                return "COULD NOT FULFIL".to_string(); 
            };

            let n = Notification::new_friend_req(target.unique_name.clone(), &sender, &req_id);

            if let Some(live_target) = state.get_client(&target.unique_name).await {
                live_target.send_socket_order(Arc::new([
                    Signal::Notification(n)
                ])).await;
            }

            "OK".to_string()
        }
        _ => {
            "USER NOT FOUND".to_string()
        }
    }
}

// pub async fn initial_data_request(session: Session<SessionPgPool>) -> String {
//
// }

#[derive(Deserialize)]
pub struct AuthForm {
    unique_name: CheckedString,
    password: String,
}
pub async fn signup(session: Session<SessionPgPool>, Json(form): Json<AuthForm>) -> String {
    let valid = Client::create_new_acc(&form.unique_name, &form.password);
    if valid {
        session.set_store(true);
        session.set("client_unique_name", form.unique_name);
        "OK".to_string()
    } else {
        "FAILED".to_string()
    }
}

pub async fn signin(session: Session<SessionPgPool>, Json(form): Json<AuthForm>) -> String {
    let result = Client::validate_password(&form.unique_name, &form.password);
    match result {
        AuthValidationResult::Valid => {
            session.set_store(true);
            session.set("client_unique_name", form.unique_name);
            String::from("OK")
        },
        AuthValidationResult::IncorrectPassword => {
            String::from("INCORRECT PASSWORD")
        },
        AuthValidationResult::IncorrectUniqueName => {
            String::from("INCORRECT UNIQUE_NAME")
        }
    }
}
