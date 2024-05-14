// @generated automatically by Diesel CLI.

diesel::table! {
    channel_subscribers (user_id, channel_id) {
        user_id -> Int8,
        channel_id -> Int8,
        subscribed_at -> Timestamp,
    }
}

diesel::table! {
    channels (id) {
        id -> Int8,
        #[max_length = 255]
        channel_name -> Varchar,
        #[max_length = 255]
        channel_description -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_activity -> Timestamp,
        subscribers_count -> Int4,
    }
}

diesel::table! {
    messages (id) {
        id -> Int8,
        sender_id -> Int8,
        channel_id -> Int8,
        #[max_length = 2000]
        content -> Varchar,
        sent_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_relations (id) {
        id -> Int8,
        a -> Int8,
        b -> Int8,
        sender -> Int8,
        accepted -> Bool,
        created_at -> Timestamp,
        accepted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 254]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        display_name -> Nullable<Varchar>,
        #[max_length = 97]
        password_hash -> Bpchar,
        created_at -> Timestamp,
        last_online -> Timestamp,
    }
}

diesel::joinable!(channel_subscribers -> channels (channel_id));
diesel::joinable!(channel_subscribers -> users (user_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(
    channel_subscribers,
    channels,
    messages,
    user_relations,
    users,
);
