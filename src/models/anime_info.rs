use std::string;

use crate::schema::anime_info;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, Default)]
pub struct AnimeInfo {
    pub id: i32,
    pub real_name: String,
    pub anilist_id: Option<i32>,
    pub cover: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub episodes: Option<i32>,
    pub title_preffered: Option<String>,
    pub title_romanji: Option<String>,
    pub title_original: Option<String>,
    pub title_english: Option<String>,
    pub score: Option<i32>,
    pub is_adult: Option<bool>,
    pub source_material: Option<String>,
    pub not_found: bool,
}

#[derive(Insertable)]
#[table_name = "anime_info"]
pub struct NewAnimeInfoEntry {
    pub real_name: String,
    pub anilist_id: i32,
    pub cover: String,
    pub banner: String,
    pub description: String,
    pub episodes: i32,
    pub title_preffered: String,
    pub title_romanji: String,
    pub title_original: String,
    pub title_english: String,
    pub score: i32,
    pub is_adult: bool,
    pub source_material: String,
}

#[derive(Insertable)]
#[table_name = "anime_info"]
pub struct NotFoundAnimeInfoEntry {
    pub real_name: String,
    pub not_found: bool,
}

impl AnimeInfo {
    pub fn get(
        name: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Self, String> {
        use crate::schema::anime_info::dsl::*;
        let db = db.get().unwrap();
        match anime_info
            .filter(real_name.eq(&name))
            .first::<Self>(&db)
        {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Anime not found.")),
        }
    }

    pub fn new(
        name: String,
        al_id: i32,
        cover_img: String,
        banner_img: String,
        desc: String,
        eps: i32,
        preffered: String,
        english: String,
        original: String,
        romanji: String,
        avg_score: i32,
        adult: bool,
        src: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::anime_info::dsl::*;
        let db = db.get().unwrap();
        let entry = NewAnimeInfoEntry {
            real_name: name,
            anilist_id: al_id,
            cover: cover_img,
            banner: banner_img,
            description: desc,
            episodes: eps,
            title_preffered: preffered,
            title_english: english,
            title_original: original,
            title_romanji: romanji,
            score: avg_score,
            is_adult: adult,
            source_material: src,
        };
        match diesel::insert_into(anime_info)
            .values(entry)
            .get_result::<Self>(&db)
        {
            Ok(u) => u,
            _ => Self::default(),
        }
    }

    pub fn new_not_found(
        name: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::anime_info::dsl::*;
        let db = db.get().unwrap();
        let entry = NotFoundAnimeInfoEntry {
            real_name: name,
            not_found: true,
        };
        match diesel::insert_into(anime_info)
            .values(entry)
            .get_result::<Self>(&db)
        {
            Ok(u) => u,
            _ => Self::default(),
        }
    }
}
