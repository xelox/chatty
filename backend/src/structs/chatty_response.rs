use axum::{body::Body, response::{IntoResponse, Response}};
use serde::Serialize;

pub enum ChattyResponse {
    Ok,
    Unauthorized,
    InternalError,
    Plain(String),
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

            Self::Plain(str) => {
                return Response::builder()
                    .status(200)
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
        }
    }
}

trait SerializeToJson {
    fn json_response(&self) -> Response<Body>;
}

impl<T: Serialize> SerializeToJson for T {
    fn json_response(&self) -> Response<Body> {
        let Ok(json) = serde_json::to_string(self) else {
            return ChattyResponse::InternalError.into_response();
        };
        
        return Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from(json))
            .unwrap()
    }
}
