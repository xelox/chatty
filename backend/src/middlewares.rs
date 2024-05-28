use axum::{
    extract::Request, 
    http::{Method, StatusCode, Uri}, 
    middleware::Next, 
    response::{
        IntoResponse, Redirect, Response
    }
};
use axum_session::{Session, SessionPgPool};

use crate::structs::id::ChattyId;

pub async fn validate_auth(session: Session<SessionPgPool>, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    match session.get::<ChattyId>("user_id") {
        Some(_) => {
            Ok(next.run(request).await) 
        },
        _ => {
            println!("UNAUTHORIZED page access {} ", uri);
            Ok(Redirect::to("/app/auth").into_response())
        }
    }
}

pub async fn log(method: Method, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("{}: {}", method.to_string(), uri.to_string());
    Ok(next.run(request).await)
}
