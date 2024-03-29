use std::sync::Arc;
use axum::{
    routing::{get, post}, 
    Router,
    middleware
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tower::ServiceBuilder;

pub mod server_state;
pub mod send_msg_req_validator;
pub mod web_socket_manager;
pub mod middlewares;
pub mod serve_app;
pub mod database;

use crate::server_state::ServerState;
use crate::send_msg_req_validator::post_message;

#[tokio::main]
async fn main() {
    database::establish_connection();
    // build our application with a single route
    let server_state = Arc::new(ServerState::new());
    let app = Router::new()
        .route("/ws", get(web_socket_manager::handler))
        .route("/app/*any", get(serve_app::serve_app))
        .route("/api/post_message", post(post_message))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(middlewares::validate_auth))
        )
        .route("/app/auth", get(serve_app::serve_app))
        .route("/api/signin", post(serve_app::signin))
        .route("/api/signup", post(serve_app::signup))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .layer(middleware::from_fn(middlewares::log))
        .layer(CorsLayer::permissive())
        .with_state(server_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
