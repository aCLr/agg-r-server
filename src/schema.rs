table! {
    records (id) {
        id -> Int4,
        title -> Nullable<Text>,
        guid -> Text,
        source_id -> Int4,
        content -> Text,
        date -> Timestamp,
        image -> Nullable<Text>,
    }
}

table! {
    records_user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        record_id -> Int4,
        starred -> Bool,
    }
}

table! {
    sources (id) {
        id -> Int4,
        name -> Text,
        origin -> Text,
        kind -> Text,
        image -> Nullable<Text>,
        last_scrape_time -> Timestamp,
    }
}

table! {
    sources_user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        source_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        last_read_date -> Timestamp,
        token -> Nullable<Text>,
    }
}

joinable!(records -> sources (source_id));
joinable!(records_user_settings -> records (record_id));
joinable!(records_user_settings -> users (user_id));
joinable!(sources_user_settings -> sources (source_id));
joinable!(sources_user_settings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    records,
    records_user_settings,
    sources,
    sources_user_settings,
    users,
);
