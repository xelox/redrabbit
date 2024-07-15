// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Integer,
        name -> Text,
        startdue -> Nullable<Integer>,
        deadline -> Nullable<Integer>,
        notes -> Nullable<Text>,
        done -> Bool,
        started -> Bool,
        parent_id -> Nullable<Integer>,
        is_open -> Bool,
    }
}
