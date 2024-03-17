use axum::{
    extract::Request, 
    http::{HeaderMap, StatusCode, Uri}, 
    middleware::Next, 
    response::{
        IntoResponse, Redirect, Response
    }
};

pub async fn validate_auth(uri: Uri, headers: HeaderMap, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("LOG 1: {}", uri.to_string());
    match headers.get("auth_token") {
        Some(token) => {
            println!("{}: {}", uri.to_string(), token.to_str().unwrap());
            let response = next.run(request).await; 
            Ok(response)
        },
        _ => {
            if uri.to_string().starts_with("/api") {
                Err(StatusCode::UNAUTHORIZED)
            } else {
                Ok(Redirect::permanent("/app/auth").into_response())
            }
        }
    }
}

pub async fn log(uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("LOG 2: {}", uri.to_string());
    Ok(next.run(request).await)
}
