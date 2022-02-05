table! {
    link_items (id) {
        id -> Text,
        user_id -> Int4,
        url -> Text,
        track_id -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    link_uses (link_item_id) {
        id -> Integer,
        link_item_id -> Text,
        ip -> Text,
        user_agent -> Text,
        ts -> Timestamp,
    }
}
