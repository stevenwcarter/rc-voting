// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        done -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

diesel::table! {
    votes (user_id, item_id) {
        user_id -> Integer,
        item_id -> Integer,
        ordinal -> Integer,
    }
}

diesel::joinable!(votes -> items (item_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(items, users, votes,);
