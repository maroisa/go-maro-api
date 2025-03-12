// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        created_at -> Date,
        #[max_length = 128]
        source -> Varchar,
        #[max_length = 32]
        alias -> Varchar,
    }
}
