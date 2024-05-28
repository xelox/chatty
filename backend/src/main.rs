use axum::{
    middleware,
    routing::get,
    Router,
};
use axum_session::{SessionConfig, SessionLayer, SessionPgPool, SessionStore};
use std::{fs, sync::Arc};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

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

        // Restricted pages
        .route("/app/*any", get(serve_app::serve_app))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(middlewares::validate_auth)))

        // Api
        .nest("/api", api::router::create_api_router())

        // Un-restricted pages
        .route("/app/auth", get(serve_app::serve_app))

        // Static File Server
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/", ServeDir::new("../frontend/public"))
        .nest_service("/media", ServeDir::new("/home/alex/dev/chatty/media"))

        // LOG middleware
        // .layer(ServiceBuilder::new().layer(middleware::from_fn(middlewares::log)))

        // State & Sessions
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
