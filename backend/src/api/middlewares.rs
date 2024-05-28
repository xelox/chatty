use axum::{extract::Request, http::{Method, StatusCode, Uri}, middleware::Next, response::{IntoResponse, Redirect, Response}};
use axum_session::{Session, SessionPgPool};

use crate::structs::id::ChattyId;



pub async fn api_protection(uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("API PROTECTION: {}", uri.to_string());
    // MESSAGING
    // [/] channel info             /channel/:channel_id        (get)
    // [/] load messages            /messages/:channel_id/:ts   (get)
    // [/] send message             /message                    (post)
    // [/] delete message           /message/:message_id        (delete)
    // [/] edit message             /message                    (patch)
    //
    // RELATIONS
    // [/] send friend req          /relation                   (post)
    // [/] edit friend rel          /relation                   (patch)
    //
    // USER
    // [/] log out                  /log_out                    (patch)
    // [/] sign in                  /signin                     (post)
    // [/] sign up                  /signout                    (post)
    // [/] initial data request     /init                       (get)
    // [/] update profile           /profile                    (patch)
    match uri {
        _ => {
            println!("Accessed a protection non implemented API call: {}", uri.to_string());
            return Err(StatusCode::NOT_IMPLEMENTED);
        }
    };

    // Ok(next.run(request).await)
}

pub async fn validate_auth(session: Session<SessionPgPool>, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    match session.get::<ChattyId>("user_id") {
        Some(_) => {
            Ok(next.run(request).await) 
        },
        _ => {
            println!("UNAUTHORIZED api access {} ", uri);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn log(method: Method, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("{}: {}", method.to_string(), uri.to_string());
    Ok(next.run(request).await)
}
