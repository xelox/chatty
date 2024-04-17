// @generated automatically by Diesel CLI.

diesel::table! {
    friendship (id) {
        id -> Uuid,
        a -> Varchar,
        b -> Varchar,
        sender -> Varchar,
        accepted -> Bool,
    }
}

diesel::table! {
    users (unique_name) {
        unique_name -> Varchar,
        password_hash -> Varchar,
        display_name -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    friendship,
    users,
);
