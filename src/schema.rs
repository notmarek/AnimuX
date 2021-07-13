table! {
    invites (id) {
        id -> Int4,
        invite -> Varchar,
        used -> Bool,
    }
}

table! {
    naughty (id) {
        id -> Int4,
        ip -> Varchar,
        times -> Int4,
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
    naughty,
    users,
);
