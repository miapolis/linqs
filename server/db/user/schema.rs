table! {
    users (id) {
        id -> Int4,
        username -> Text,
    }
}

table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(users, auth_infos,);
