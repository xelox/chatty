use axum::{body::Body, extract::Request};

use crate::structs::chatty_response::ChattyResponse;

pub fn send_message(req: &Request<Body>) -> Result<(), ChattyResponse> {
    Ok(())
}
