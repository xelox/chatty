use std::sync::Arc;
use axum::{
    routing::{get, post}, 
    Router,
    middleware
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tower::ServiceBuilder;
use axum_session::{SessionPgPool, SessionConfig, SessionStore, SessionLayer};

pub mod server_state;
pub mod api;
pub mod web_socket_manager;
pub mod middlewares;
pub mod serve_app;
pub mod database;
pub mod structs;

use crate::server_state::ServerState;

#[tokio::main]
async fn main() {
    database::establish_connection();
    let session_config = SessionConfig::default()
        .with_table_name("session_table")
        .with_key(axum_session::Key::generate())
        .with_database_key(axum_session::Key::generate());
    let session_store = SessionStore::<SessionPgPool>::new(None, session_config).await.unwrap();
    let server_state = Arc::new(ServerState::new());
    let app = Router::new()
        .route("/ws", get(web_socket_manager::handler))
        .route("/app/*any", get(serve_app::serve_app))
        .route("/api/post_message", post(api::post_message))
        .route("/api/send_friend_request", post(api::send_friend_request))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(middlewares::validate_auth))
        )
        .route("/app/auth", get(serve_app::serve_app))
        .route("/api/signin", post(api::signin))
        .route("/api/signup", post(api::signup))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/", ServeDir::new("../frontend/public"))
        .layer(middleware::from_fn(middlewares::log))
        .layer(CorsLayer::permissive())
        .layer(SessionLayer::new(session_store))
        .with_state(server_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
