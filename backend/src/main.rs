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
pub mod chat_api;

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
        .route("/api/update_profile", post(api::update_profile))
        .route("/api/channel_info/:channel_id", get(api::channel_info))

        .nest("/api/message", chat_api::router::create_chat_api_router(server_state.clone()))

        .route("/api/send_friend_request", post(api::send_friend_request))
        .route("/api/friendship/:action", post(api::edit_relation))
        .route("/api/logout", get(api::logout))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(middlewares::validate_auth)))
        .route("/app/auth", get(serve_app::serve_app))
        .route("/api/signin", post(api::signin))
        .route("/api/signup", post(api::signup))
        .route("/api/initial_data_request", get(api::initial_data_request))
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
