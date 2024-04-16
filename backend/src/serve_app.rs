use axum::{
    body::Body, http::{
        header, HeaderMap, StatusCode
    }, 
    response::IntoResponse
};
use tokio_util::io::ReaderStream;


pub async fn serve_app() -> impl IntoResponse {
    let file = match tokio::fs::File::open("../frontend/dist/index.html").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
    headers.insert(header::CONTENT_DISPOSITION, "inline; filename=\"index.html\"".parse().unwrap());

    Ok((headers, body))
}
