table! {
    link_items (id) {
        id -> Text,
        url -> Text,
        track_id -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    link_uses (link_item_id) {
        id -> Integer,
        link_item_id -> Text,
        ip -> Nullable<Binary>,
        user_agent -> Text,
        ts -> Timestamp,
    }
}
