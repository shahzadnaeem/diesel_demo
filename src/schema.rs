// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        value -> Varchar,
        display_value -> Varchar,
        display_order -> Int4,
        enum_id -> Int4,
        parent_id -> Nullable<Int4>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    posts,
);
