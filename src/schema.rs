// @generated automatically by Diesel CLI.

diesel::table! {
    rocks (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        kind -> Varchar,
    }
}
