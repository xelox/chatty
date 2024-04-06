use axum::{
    body::Body, http::{
        header, HeaderMap, StatusCode
    }, 
    response::IntoResponse, Json,
};
use diesel::{RunQueryDsl, SelectableHelper};
use serde::Deserialize;
use tokio_util::io::ReaderStream;
use crate::database::{self, models::{self, User}, schema};
use axum_session::{Session, SessionPgPool};


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

#[derive(Deserialize)]
pub struct AuthForm {
    username: String,
    password: String,
}

pub async fn signup(session: Session<SessionPgPool>, Json(form): Json<AuthForm>) -> String {
    let conn = &mut database::establish_connection();
    let password_hash = password_auth::generate_hash(&form.password);
    
    let res: Result<User, _> = diesel::insert_into(schema::users::table)
        .values(&models::NewUser{
            nick: &form.username,
            password_hash: &password_hash,
        })
        .returning(models::User::as_returning())
        .get_result(conn);

    match res {
        Ok(user) => {
            session.set_store(true);
            session.set("username", user.nick);
            dbg!(&session.get::<String>("username"));
            "OK".to_string()
        }
        _ => {
            "FAILED".to_string()
        }
    }
}

pub async fn signin(session: Session<SessionPgPool>, Json(form): Json<AuthForm>) -> String {
    use schema::users::dsl::*;
    use diesel::prelude::*;
    let conn = &mut database::establish_connection();

    let user_search: Result<User, _> = users
        .find(form.username)
        .first(conn);

    match user_search {
        Ok(user) => {
            match password_auth::verify_password(form.password, &user.password_hash) {
                Ok(()) => {
                    session.set_store(true);
                    session.set("username", user.nick);
                    "OK".to_string()
                }
                _ => {
                    "INVALID PASSWORD".to_string()
                }
            }
        }
        _ => {
            "INVALID USER".to_string()
        }
    }
}
