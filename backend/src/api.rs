use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::database::models::{Friendship, FriendshipTargets};
use crate::server_state::ServerState;
use crate::database::{self, models::User, schema};
use crate::structs::notification::Notification;
use axum_session::{Session, SessionPgPool};
use crate::structs::client::{ Client, AuthValidationResult };

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
    "OK".to_string()
}


#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub struct FriendRequestForm {
    to: String
}
pub async fn send_friend_request(session: Session<SessionPgPool>, State(state): State<Arc<ServerState>>, Json(payload): Json<FriendRequestForm>) -> String {
    let sender = session.get::<String>("client_unique_name");
    if sender.is_none() {
        return "Who are you?".to_string();
    }
    let sender = sender.unwrap();

    use schema::users::dsl::*;
    use diesel::prelude::*;
    let conn = &mut database::establish_connection();
    

    let query: Result<User, _> = users
        .find(payload.to)
        .first(conn);

    match query {
        Ok(target) => {
            let message = format!("Friend request from {}", &sender);
            let req_id_res = Friendship::create(FriendshipTargets::new(&sender, &target.unique_name), &sender);
            if req_id_res.is_none() { return "COULD NOT FULFIL".to_string(); }
            let req_id = req_id_res.unwrap();
            Notification::new_friend_req(&target.unique_name, &sender, req_id);
            if let Some(connected_client) = state.get_client(&target.unique_name).await {
                connected_client.send_socket_message(&message).await;
            }
            let notification = Notification::new(target.unique_name, message);
            notification.store();
            "OK".to_string()
        }
        _ => {
            "USER NOT FOUND".to_string()
        }
    }
}

#[derive(Deserialize)]
pub struct AuthForm {
    unique_name: String,
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
