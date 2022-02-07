table! {
    use diesel::sql_types::*;
    use crate::db::link::models::TrackItemMapping;

    link_items (id) {
        id -> Text,
        user_id -> Int4,
        url -> Text,
        track_id -> Text,
        uses -> Integer,
        to_track -> Array<TrackItemMapping>,
    }
}

table! {
    use diesel::sql_types::*;

    link_uses (link_item_id) {
        id -> Integer,
        link_item_id -> Text,
        ip -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        ts -> Nullable<Timestamp>,
    }
}
