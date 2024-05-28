use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_session::{SessionConfig, SessionLayer, SessionPgPool, SessionStore};
use std::{fs, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod api;
pub mod config;
pub mod database;
pub mod middlewares;
pub mod serve_app;
pub mod server_state;
pub mod structs;
pub mod web_socket_manager;
pub mod file_storage;

use crate::server_state::ServerState;

#[tokio::main]
async fn main() {
    setup();

    database::establish_connection();
    let session_config = SessionConfig::default()
        .with_table_name("session_table")
        .with_key(axum_session::Key::generate())
        .with_database_key(axum_session::Key::generate());
    let session_store = SessionStore::<SessionPgPool>::new(None, session_config)
        .await
        .unwrap();
    let server_state = Arc::new(ServerState::new());

    let app = Router::new()
        .route("/ws", get(web_socket_manager::handler))
        .route("/app/*any", get(serve_app::serve_app))

        .nest("/api/messages", api::router::create_api_router())

        .route("/app/auth", get(serve_app::serve_app))

        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/", ServeDir::new("../frontend/public"))
        .nest_service("/media", ServeDir::new("/home/alex/dev/chatty/media"))
        .layer(middleware::from_fn(middlewares::log))
        .layer(CorsLayer::permissive())
        .layer(SessionLayer::new(session_store))
        .with_state(server_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn setup() {
    fs::create_dir_all("/home/alex/dev/chatty/private/client_notifications")
        .expect("could not setup client_notifications");
}
