use crate::database::{self, models::{self, User}, schema};
use diesel::{RunQueryDsl, SelectableHelper};

use std::sync::Arc;
use axum::extract::ws::WebSocket;
use futures_locks::{Mutex, RwLock};

use super::{checked_string::CheckedString, socket_signal::{Signal, SignalList}};

#[derive(Clone)]
pub struct Client {
    socket: Option<Mutex<WebSocket>>,
    unique_name: Arc<String>,
    chats: RwLock<Vec<String>>, //TODO chats class
    status: RwLock<String>, //TODO status class
}

impl Client {
    pub fn init_existing(unique_name: String, socket: WebSocket) -> Client {
        return Client{
            unique_name: Arc::new(unique_name),
            socket: Some(Mutex::new(socket)),
            chats: RwLock::new(Vec::new()),
            status: RwLock::new(String::from("online")),
        }
    }

    pub fn create_new_acc(unique_name: &CheckedString, password: &String) -> bool {
        let conn = &mut database::establish_connection();
        let password_hash = password_auth::generate_hash(&password);

        let res: Result<User, _> = diesel::insert_into(schema::users::table)
            .values(&models::NewUser{
                unique_name: &unique_name.to_string(),
                display_name: None,
                password_hash: &password_hash,
            })
            .returning(models::User::as_returning())
            .get_result(conn);

        return res.is_ok();
    }

    pub fn validate_password(unique_name_: &CheckedString, password: &String) -> AuthValidationResult {
        use schema::users::dsl::*;
        use diesel::prelude::*;
        let conn = &mut database::establish_connection();

        let user_search: Result<User, _> = users
            .find(unique_name_)
            .first(conn);

        match user_search {
            Ok(user) => {
                match password_auth::verify_password(&password, &user.password_hash) {
                    Ok(()) => {
                        AuthValidationResult::Valid
                    }
                    _ => {
                        AuthValidationResult::IncorrectPassword
                    }
                }
            }
            _ => {
                AuthValidationResult::IncorrectUniqueName
            }
        }
    }

    pub async fn send_socket_order(&self, signals: Arc<[Signal]>) {
        if let Some(socket_mutex) = &self.socket {
            let mut socket = socket_mutex.lock().await;
            if let Some(message) = SignalList::new(signals).to_message() {
                let _ = socket.send(message).await;
            }
        }
    }
}


pub enum AuthValidationResult {
    Valid,
    IncorrectUniqueName,
    IncorrectPassword,
}
