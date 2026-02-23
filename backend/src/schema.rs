// @generated automatically by Diesel CLI.

diesel::table! {
    group_contacts (low_group_id, high_group_id) {
        low_group_id -> Integer,
        high_group_id -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    groups (id) {
        id -> Integer,
        is_black -> Bool,
        max_liberties -> Integer,
    }
}

diesel::table! {
    stones (x, y) {
        x -> BigInt,
        y -> BigInt,
        group_id -> Integer,
    }
}

diesel::joinable!(stones -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(group_contacts, groups, stones,);
