// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_on -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}
