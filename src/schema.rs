// @generated automatically by Diesel CLI.

diesel::table! {
    elections (uuid) {
        uuid -> Text,
        name -> Text,
        owner_uuid -> Text,
    }
}

diesel::table! {
    items (uuid) {
        uuid -> Text,
        election_uuid -> Text,
        title -> Text,
        body -> Text,
        done -> Bool,
    }
}

diesel::table! {
    sessions (uuid) {
        uuid -> Text,
        user_uuid -> Text,
        created -> BigInt,
        expires -> BigInt,
        data -> Nullable<Text>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::table! {
    votes (election_uuid, user_uuid, item_uuid) {
        election_uuid -> Text,
        user_uuid -> Text,
        item_uuid -> Text,
        ordinal -> Integer,
    }
}

diesel::joinable!(elections -> users (owner_uuid));
diesel::joinable!(items -> elections (election_uuid));
diesel::joinable!(sessions -> users (user_uuid));
diesel::joinable!(votes -> elections (election_uuid));
diesel::joinable!(votes -> items (item_uuid));
diesel::joinable!(votes -> users (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    elections,
    items,
    sessions,
    users,
    votes,
);
