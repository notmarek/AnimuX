use std::sync::Arc;

// use crate::googledrive::Drive;
use crate::mango::Mango;
use crate::navidrome::Navidrome;
use crate::utils::anilist_scraper::search_anime;

use serde::{Deserialize, Serialize};

use std::fs;
use std::lazy::SyncLazy;

use anitomy::{Anitomy, ElementCategory};

use diesel::prelude::*;
use diesel::r2d2;

pub static ANIME: SyncLazy<Vec<AnimeInfo>> = SyncLazy::new(|| {
    let contents = fs::read_to_string("map.json");
    let mal_info: Vec<AnimeInfo> = serde_json::from_str(&contents.unwrap()).unwrap();
    mal_info
});

#[derive(Clone)]
pub struct State {
    pub app_name: String,
    pub base_path: String,
    // pub drive: Option<Arc<Drive>>,
    pub mal_secret: Option<String>,
    pub mal_client_id: Option<String>,
    pub hcaptcha_enabled: bool,
    pub hcaptcha_sitekey: Option<String>,
    pub hcaptcha_secret: Option<String>,
    pub secret: String,
    pub database: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    pub mango_enabled: bool,
    pub mango: Option<Mango>,
    pub navidrome_enabled: bool,
    pub navidrome: Option<Navidrome>,
    pub default_upload_path: Option<String>,
    pub root_folder: String,
    pub trans_username: Option<String>,
    pub trans_password: Option<String>,
    pub trans_address: Option<String>,
    pub rssmission_config: Option<String>,
    pub response_secret: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T: Serialize> {
    pub status: String,
    pub data: T,
}

impl<T: Serialize> Response<T> {
    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeInfo {
    pub name: Option<String>,
    pub mal: Option<u32>,
    pub episode_offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALReply {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALUser {
    pub user: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALAnime {
    pub anime_id: u32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALAnimeUpdate {
    pub anime_id: u32,
    pub token: String,
    pub status: String,
    pub num_watched_episodes: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub mtime: Option<String>,
    pub size: Option<u64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Directory {
    pub name: String,
    pub files: Vec<StorageThing>,
    pub mtime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StorageThing {
    File(ParsedFile),
    Directory(Directory),
    Empty(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HCaptchaResponse {
    pub success: bool,
    pub challange_ts: Option<String>,
    pub hostname: Option<String>,
    pub credit: Option<bool>,
    pub error_codes: Option<Vec<String>>,
    pub score: Option<f64>,
    pub score_reason: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedFile {
    pub name: Option<String>,
    pub anime: Option<String>,
    pub group: Option<String>,
    pub episode: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub mtime: Option<String>,
    pub size: Option<u64>,
    pub anilist_info: Option<crate::models::anime_info::AnimeInfo>,
}

impl ParsedFile {
    pub async fn from_file(file: File, db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Self {
        let anime_info: Vec<AnimeInfo> = ANIME.clone();
        let parsed_file: ParsedFile;

        if file.kind.as_ref().unwrap() == "file"
            && !(file.name.as_ref().unwrap().contains(".mkv")
                || file.name.as_ref().unwrap().contains(".mp4"))
        {
            parsed_file = ParsedFile {
                name: file.path,
                anime: file.name,
                group: Some(String::new()),
                episode: Some(String::new()),
                kind: file.kind,
                mtime: file.mtime,
                size: file.size,
                anilist_info: None,
            };
        } else {
            let mut anitomy: Anitomy = Anitomy::new();
            
            match anitomy.parse(file.name.as_ref().unwrap()) {
                Ok(ref e) | Err(ref e) => {
                    let info = match crate::models::anime_info::AnimeInfo::get(file.path.clone().unwrap(), db) {
                        Ok(i) => i,
                        Err(_) => {
                            match search_anime(e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string(), None).await {
                                Ok(e) => { crate::models::anime_info::AnimeInfo::new(file.path.clone().unwrap(), e.data.media, db) },
                                Err(_) => { crate::models::anime_info::AnimeInfo::new_not_found(file.path.clone().unwrap(), db)},
                            }
                        }
                    };
                    let mal = &anime_info
                        .into_iter()
                        .filter(|ye| {
                            ye.name.as_ref().unwrap()
                                == e.get(ElementCategory::AnimeTitle).unwrap_or("")
                        })
                        .collect::<Vec<AnimeInfo>>();
                    parsed_file = ParsedFile {
                        name: file.path,
                        anime: Some(e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string()),
                        group: Some(
                            e.get(ElementCategory::ReleaseGroup)
                                .unwrap_or("")
                                .to_string(),
                        ),
                        episode: Some(
                            e.get(ElementCategory::EpisodeNumber)
                                .unwrap_or("")
                                .to_string(),
                        ),
                        kind: file.kind,
                        mtime: file.mtime,
                        size: file.size,
                        anilist_info: Some(info),
                    }
                }
            }
        }
        parsed_file
    }
}
