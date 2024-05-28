use std::sync::Arc;

use axum::{middleware, routing::{get, patch, post, MethodRouter}, Router};
use tower::ServiceBuilder;

use crate::server_state::ServerState;

use super::{chat_api, user_api};

pub fn create_api_router() -> Router<Arc<ServerState>> {
    // MESSAGING
    // [/] channel info             /channel/:channel_id        (get)
    // [/] load messages            /messages/:channel_id/:ts   (get)
    // [/] send message             /message                    (post)
    // [/] delete message           /message/:message_id        (delete)
    // [/] edit message             /message                    (patch)
    //
    // RELATIONS
    // [/] send friend req          /relation                   (post)
    // [/] edit friend rel          /relation                   (patch)
    //
    // USER
    // [/] log out                  /log_out                    (patch)
    // [/] sign in                  /signin                     (post)
    // [/] sign up                  /signout                    (post)
    // [/] initial data request     /init                       (get)
    // [/] update profile           /profile                    (patch)

    
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
        // Roues are permission checked.
        .layer(ServiceBuilder::new().layer(middleware::from_fn(super::middlewares::api_protection)))
        // Routes are NOT permission checked.

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
