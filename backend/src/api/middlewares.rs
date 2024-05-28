use axum::{extract::Request, http::{Method, StatusCode, Uri}, middleware::Next, response::Response};
use axum_session::{Session, SessionPgPool};

use crate::structs::id::ChattyId;



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
