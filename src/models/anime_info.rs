use crate::schema::anime_info;
use crate::utils::anilist_scraper::AnilistMedia;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, Default, Clone, Identifiable)]
#[table_name = "anime_info"]
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
    pub updated: bool,
}

#[derive(AsChangeset)]
#[table_name = "anime_info"]
pub struct UpdatedAnilistId {
    pub anilist_id: i32,
    pub updated: bool,
    pub not_found: bool,
}

#[derive(Identifiable)]
#[table_name = "anime_info"]
pub struct FindById {
    pub id: i32,
}

#[derive(Insertable, AsChangeset)]
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
    pub not_found: bool,
    pub updated: bool,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "anime_info"]
pub struct NotFoundAnimeInfoEntry {
    pub real_name: String,
    pub not_found: bool,
    pub updated: bool,

}

impl AnimeInfo {
    pub fn get(
        name: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Self, String> {
        use crate::schema::anime_info::dsl::*;
        let db = db.get().unwrap();
        match anime_info.filter(real_name.eq(&name)).first::<Self>(&db) {
            Ok(e) => Ok(e),
            Err(_) => Err(String::from("Anime not found.")),
        }
    }

    pub fn get_all(pool: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Vec<Self> {
        use crate::schema::anime_info::dsl::*;
        let db = pool.get().unwrap();
        anime_info.get_results::<Self>(&db).unwrap()
    }

    pub fn update_alid(
        real_id: i32,
        al_id: i32,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) {
        let db = db.get().unwrap();
        diesel::update(&FindById { id: real_id })
            .set(UpdatedAnilistId {
                anilist_id: al_id,
                updated: true,
                not_found: false,
            })
            .execute(&db)
            .unwrap();
    }

    pub fn update(
        self,
        al_response: AnilistMedia,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        let db = db.get().unwrap();
        let entry = NewAnimeInfoEntry {
            real_name: self.real_name.clone(),
            anilist_id: al_response.id,
            cover: al_response.cover_image.extra_large,
            banner: al_response.banner_image.unwrap_or_default(),
            description: al_response.description,
            episodes: al_response.episodes,
            title_preffered: al_response.title.user_preferred,
            title_english: al_response.title.english,
            title_original: al_response.title.native,
            title_romanji: al_response.title.romaji,
            score: al_response.average_score,
            is_adult: al_response.is_adult,
            source_material: al_response.source,
            not_found: false,
            updated: false,
        };
        match diesel::update(&self).set(entry).get_result::<Self>(&db) {
            Ok(u) => u,
            _ => Self::default(),
        }
    }

    pub fn new(
        path: String,
        al_response: AnilistMedia,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::anime_info::dsl::*;
        let db = db.get().unwrap();
        let entry = NewAnimeInfoEntry {
            real_name: path,
            anilist_id: al_response.id,
            cover: al_response.cover_image.extra_large,
            banner: al_response.banner_image.unwrap_or_default(),
            description: al_response.description,
            episodes: al_response.episodes,
            title_preffered: al_response.title.user_preferred,
            title_english: al_response.title.english,
            title_original: al_response.title.native,
            title_romanji: al_response.title.romaji,
            score: al_response.average_score,
            is_adult: al_response.is_adult,
            source_material: al_response.source,
            not_found: false,
            updated: false,
        };
        match diesel::insert_into(anime_info)
            .values(entry)
            .get_result::<Self>(&db)
        {
            Ok(u) => u,
            _ => Self::default(),
        }
    }

    pub fn change_to_not_found(
        self,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        let db = db.get().unwrap();
        let entry = NotFoundAnimeInfoEntry {
            real_name: self.real_name.clone(),
            not_found: true,
            updated: false,

        };
        match diesel::update(&self).set(entry).get_result::<Self>(&db) {
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
            updated: false,
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
