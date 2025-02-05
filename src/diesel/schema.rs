// @generated automatically by Diesel CLI.

diesel::table! {
    links (link_id) {
        link_id -> Int4,
        created_at -> Date,
        #[max_length = 32]
        source -> Varchar,
        #[max_length = 32]
        alias -> Varchar,
    }
}
