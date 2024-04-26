use diesel::pg::PgConnection; 
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod users_table;
pub mod user_relations_table;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    PgConnection::establish(&db_url).expect("Error connecting to postgresql")
}
