use std::sync::Arc;

use axum::{middleware, routing::{get, patch, post, MethodRouter}, Router};
use tower::ServiceBuilder;

use crate::server_state::ServerState;

use super::{chat_api, user_api};

pub fn create_api_router() -> Router<Arc<ServerState>> {

    Router::new()
        .route("/channel/:channel_id", get(chat_api::channel_info))

        .route("/messages/:channel_id/:ts", get(chat_api::load_messages))
        .route("/message", MethodRouter::new()
            .patch(chat_api::edit_message)
            .delete(chat_api::delete_message)
            .post(chat_api::send_message)
        )

        .route("/relation", MethodRouter::new() 
            .post(user_api::send_friend_request)
            .patch(user_api::edit_relation)
        )

        .route("/profile", patch(user_api::update_profile))
        .route("/init", get(user_api::initial_data_request))
        .route("/logout", post(user_api::logout))
        // User is Logged In.
        .layer(ServiceBuilder::new().layer(middleware::from_fn(super::middlewares::validate_auth)))
        // User is NOT Logged In.

        .route("/signin", post(user_api::signin))
        .route("/signup", post(user_api::signup))

        .layer(ServiceBuilder::new().layer(middleware::from_fn(super::middlewares::log)))

}
