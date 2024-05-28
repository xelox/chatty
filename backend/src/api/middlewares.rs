use std::{borrow::Borrow, fmt::Display, sync::Arc};

use axum::{body::{Body, to_bytes}, extract::{FromRequest, Request, State}, http::{Method, StatusCode, Uri}, middleware::Next, response::{IntoResponse, Response}, Json, RequestExt};
use axum_session::{Session, SessionPgPool};
use futures::StreamExt;
use tokio_util::bytes::Buf;

use crate::{database::{channel_subscribers_table::ChannelSubscribers, message_table::NewMessage}, server_state::ServerState, structs::id::ChattyId};


enum Protections {
    ViewChannel,
    SendMessage,
    DeleteMessage,
    EditMessage,
    SendFriendReq,
    EditFriendReq,
}

impl Display for Protections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Protections::ViewChannel => "View Channel",
            Protections::SendMessage => "Send Message",
            Protections::DeleteMessage => "Delete Message",
            Protections::EditMessage => "Edit Message",
            Protections::SendFriendReq => "Send Friend Req",
            Protections::EditFriendReq => "Edit Friend Req",
        };

        f.write_str(s)
    }
}

fn protection_pattern_issue(uri: Uri) -> Result<Response, StatusCode> {
    println!("Protection pattern not implemented: {}", uri.to_string());
    return Err(StatusCode::NOT_IMPLEMENTED);
}

fn protection_impl_issue(p: Protections) -> Result<Response, StatusCode> {
    println!("Protection implementation is missing: {}", p);
    return Err(StatusCode::NOT_IMPLEMENTED);
}

async fn can_view_channel(_request: &Request, _session: &Session<SessionPgPool>) -> Result<(), StatusCode> {
    Ok(())
}

async fn can_send_message(request: Request, session: &Session<SessionPgPool>) -> Result<Request, StatusCode> {
    let Some(allowed_channels) = session.get::<Vec<ChattyId>>("channels") else {
        println!("No channels set");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let mut next_request = Request::new(Body::empty());
    let next_headers = next_request.headers_mut();

    for (key, val) in request.headers() {
        next_headers.append(key, val.clone());
    }

    let Ok(payload) = request.extract::<Json<NewMessage>, _>().await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };


    if allowed_channels.binary_search(&payload.channel_id).is_err() {
        let query = ChannelSubscribers::sorded_subscribed_channels(&payload.sender_id);
        let Some(allwed_channels) = query else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        };
        session.set("channels", &allwed_channels);
        if allwed_channels.binary_search(&payload.channel_id).is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    next_request.extensions_mut().insert(payload.clone());

    Ok(next_request)
}

pub async fn api_protection(method: Method, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("API PROTECTION: {}", uri.to_string());
    
    let uri_parts: Vec<&str> = uri
        .path()
        .split("/")
        .skip(1)
        .collect();

    // First we determine what protection has to be applied.
    let protection = match uri_parts[0] {
        "channel" => Protections::ViewChannel,
        "messages" => Protections::ViewChannel,
        "message" => {
            match method {
                Method::POST => Protections::SendMessage,
                Method::DELETE => Protections::DeleteMessage,
                Method::PATCH => Protections::EditMessage,
                _ => { return protection_pattern_issue(uri); }
            }
        },
        "relation" => {
            match method {
                Method::POST => Protections::SendFriendReq,
                Method::PATCH => Protections::EditFriendReq,
                _ => { return protection_pattern_issue(uri); }
            }
        }
        _ => { return protection_pattern_issue(uri); }
    };


    // Then we perform the permission validation 
    let validation_result = match protection {
        // Protections::ViewChannel => can_view_channel(&request, &session).await,
        Protections::SendMessage => can_send_message(request, &session).await,
        _ => { return protection_impl_issue(protection); }
    };

    match validation_result {
        Ok(request) => Ok(next.run(request).await),
        Err(sc) => Err(sc)
    }

}

pub async fn validate_auth(session: Session<SessionPgPool>, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    match session.get::<ChattyId>("user_id") {
        Some(_) => {
            Ok(next.run(request).await) 
        },
        _ => {
            println!("UNAUTHORIZED api access {} ", uri);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn log(method: Method, uri: Uri, request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("{}: {}", method.to_string(), uri.to_string());
    Ok(next.run(request).await)
}
