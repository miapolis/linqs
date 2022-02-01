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

joinable!(link_uses -> link_items (link_item_id));

allow_tables_to_appear_in_same_query!(link_items, link_uses,);
