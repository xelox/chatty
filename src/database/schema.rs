// @generated automatically by Diesel CLI.

diesel::table! {
    users (nick) {
        nick -> Varchar,
        password_hash -> Varchar,
        auth_token -> Nullable<Uuid>,
    }
}
