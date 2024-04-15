// @generated automatically by Diesel CLI.

diesel::table! {
    users (unique_name) {
        unique_name -> Varchar,
        password_hash -> Varchar,
        display_name -> Nullable<Varchar>,
    }
}
