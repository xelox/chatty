use axum::{extract::Request, http::{StatusCode, Uri}, middleware::Next, response::{IntoResponse, Redirect, Response}};
use axum_session::{Session, SessionPgPool};

use crate::structs::id::ChattyId;



pub async fn api_protection(uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("API PROTECTION: {}", uri.to_string());
    Ok(next.run(request).await)
}

pub async fn validate_auth(session: Session<SessionPgPool>, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    match session.get::<ChattyId>("user_id") {
        Some(_) => {
            Ok(next.run(request).await) 
        },
        _ => {
            print!("UNAUTHORIZED usage {} ", uri);
            if uri.to_string().starts_with("/api") {
                println!("Returning Status Code");
                Err(StatusCode::UNAUTHORIZED)
            } else {
                println!("Redirecting");
                Ok(Redirect::to("/app/auth").into_response())
            }
        }
    }
}
