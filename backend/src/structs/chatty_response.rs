use axum::{body::Body, response::{IntoResponse, Response}};
use serde::Serialize;

pub enum ChattyResponse {
    Ok,
    Unauthorized,
    InternalError,
    Plain(String, Option<u16>),
    BadRequest(Option<String>),
}

impl IntoResponse for ChattyResponse
    where
{
    fn into_response(self) -> Response<Body> {
        match self {
            Self::Ok => {
                return Response::builder()
                    .status(200)
                    .header("Content-Type", "text/plain")
                    .body(Body::from("Ok"))
                    .unwrap();
            }

            Self::Unauthorized => {
                return Response::builder()
                    .status(401)
                    .header("Content-Type", "text/plain")
                    .body(Body::from("Unauthorized"))
                    .unwrap();
            }

            Self::Plain(str, status) => {
                let status = status.unwrap_or(200);
                return Response::builder()
                    .status(status)
                    .header("Content-Type", "text/plain")
                    .body(Body::from(str))
                    .unwrap();
            }

            Self::InternalError => {
                return Response::builder()
                    .status(500)
                    .header("Content-Type", "text/plain")
                    .body(Body::from("Internal Server Error"))
                    .unwrap();
            }

            Self::BadRequest(option) => {
                let body_text = option.unwrap_or(String::from("Bad Request"));
                return Response::builder()
                    .status(400)
                    .header("Content-Type", "text/plain")
                    .body(Body::from(body_text))
                    .unwrap();
            }
        }
    }
}


pub fn json_response<T: Serialize>(item: T) -> Response<Body> {
    let Ok(json) = serde_json::to_string(&item) else {
        return ChattyResponse::InternalError.into_response();
    };

    return Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap()
}
