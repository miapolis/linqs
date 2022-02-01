table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
    }
}

table! {
    link_items (id) {
        id -> Varchar,
        url -> Varchar,
        track_id -> Nullable<Varchar>,
    }
}

table! {
    link_uses (id) {
        id -> Int4,
        link_item_id -> Nullable<Varchar>,
        ip -> Nullable<Bytea>,
        user_agent -> Nullable<Text>,
        ts -> Nullable<Timestamp>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

joinable!(link_uses -> link_items (link_item_id));

allow_tables_to_appear_in_same_query!(
    auth_infos,
    link_items,
    link_uses,
    posts,
    users,
);
