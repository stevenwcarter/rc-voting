
table! {
    users {
        id -> Integer,
        username -> Text,
    }
}

table! {
    items {
        id -> Integer,
        title -> Text,
        body -> Text,
        done -> Bool,
    }
}

table! {
    votes (user_id, item_id) {
        user_id -> Integer,
        item_id -> Integer,
        ordinal -> Integer,
    }
}

joinable!(votes -> items (item_id));
joinable!(votes -> users (user_id));
allow_tables_to_appear_in_same_query!(users, items, votes);
