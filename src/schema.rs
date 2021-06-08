table! {
    invites (id) {
        id -> Int4,
        invite -> Varchar,
        used -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        role -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    invites,
    users,
);
