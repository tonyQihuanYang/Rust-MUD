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

diesel::table! {
    players (id) {
        user_id -> Int4,
        id -> Int4,
        name -> Varchar,
        exp -> Int4,
    }
}

diesel::joinable!(players -> accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    players,
);
