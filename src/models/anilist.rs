use crate::schema::anilist;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct AnilistEntry {
    pub id: i32,
    pub anime_name: String,
    pub anilist_id: Option<i32>,
    pub preview_image: Option<String>,
    pub not_found: bool,
}

#[derive(Insertable)]
#[table_name = "anilist"]
pub struct NewAnilistEntry {
    pub anime_name: String,
    pub anilist_id: i32,
    pub preview_image: String,
}

#[derive(Insertable)]
#[table_name = "anilist"]
pub struct NotFoundAnilistEntry {
    pub anime_name: String,
    pub not_found: bool,
}

impl AnilistEntry {
    pub fn get(
        name: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Self, String> {
        use crate::schema::anilist::dsl::*;
        let db = db.get().unwrap();
        match anilist
            .filter(anime_name.eq(&name))
            .first::<AnilistEntry>(&db)
        {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Invite not found.")),
        }
    }

    pub fn new(
        name: String,
        al_id: i32,
        preview: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::anilist::dsl::*;
        let db = db.get().unwrap();
        let entry = NewAnilistEntry {
            anime_name: name,
            anilist_id: al_id,
            preview_image: preview,
        };
        match diesel::insert_into(anilist)
            .values(entry)
            .get_result::<Self>(&db)
        {
            Ok(u) => u,
            _ => Self {
                id: 0,
                anime_name: String::new(),
                anilist_id: None,
                preview_image: None,
                not_found: false,
            },
        }
    }

    pub fn new_not_found(
        name: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::anilist::dsl::*;
        let db = db.get().unwrap();
        let entry = NotFoundAnilistEntry {
            anime_name: name,
            not_found: true,
        };
        match diesel::insert_into(anilist)
            .values(entry)
            .get_result::<Self>(&db)
        {
            Ok(u) => u,
            _ => Self {
                id: 0,
                anime_name: String::new(),
                anilist_id: None,
                preview_image: None,
                not_found: false,
            },
        }
    }
}
