// @generated automatically by Diesel CLI.

#[cfg(feature = "ssr")]
diesel::table! {
    items (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        done -> Bool,
    }
}

#[cfg(feature = "ssr")]
diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

#[cfg(feature = "ssr")]
diesel::table! {
    votes (user_id, item_id) {
        user_id -> Integer,
        item_id -> Integer,
        ordinal -> Integer,
    }
}

#[cfg(feature = "ssr")]
diesel::joinable!(votes -> items (item_id));
#[cfg(feature = "ssr")]
diesel::joinable!(votes -> users (user_id));

#[cfg(feature = "ssr")]
diesel::allow_tables_to_appear_in_same_query!(items, users, votes,);
