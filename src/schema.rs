table! {
    anime_info (id) {
        id -> Int4,
        real_name -> Varchar,
        anilist_id -> Nullable<Int4>,
        cover -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        episodes -> Nullable<Int4>,
        title_preffered -> Nullable<Varchar>,
        title_romanji -> Nullable<Varchar>,
        title_original -> Nullable<Varchar>,
        title_english -> Nullable<Varchar>,
        score -> Nullable<Int4>,
        is_adult -> Nullable<Bool>,
        source_material -> Nullable<Varchar>,
        not_found -> Bool,
        updated -> Bool,
    }
}

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
    stars (id) {
        id -> Int4,
        user_id -> Int4,
        path -> Text,
    }
}

table! {
    storage (id) {
        id -> Int4,
        paths -> Array<Text>,
        name -> Varchar,
        exceptions -> Array<Text>,
    }
}

table! {
    torrent_queue (id) {
        id -> Int4,
        link -> Varchar,
        completed -> Bool,
        requested_by -> Int4,
        removed -> Bool,
        name -> Varchar,
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
    anime_info,
    invites,
    naughty,
    stars,
    storage,
    torrent_queue,
    users,
);
