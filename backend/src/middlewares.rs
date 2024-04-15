use axum::{
    extract::Request, 
    http::{StatusCode, Uri}, 
    middleware::Next, 
    response::{
        IntoResponse, Redirect, Response
    }
};

use axum_session::{Session, SessionPgPool};

pub async fn validate_auth(session: Session<SessionPgPool>, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    match session.get::<String>("client_unique_name") {
        Some(client_unique_name) => {
            println!("User: {}", client_unique_name);
            let response = next.run(request).await; 
            Ok(response)
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

pub async fn log(uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("LOG: {}", uri.to_string());
    Ok(next.run(request).await)
}
