// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        key -> Varchar,
        secret_key -> Varchar,
        target_url -> Varchar,
        is_active -> Bool,
        clicks -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
