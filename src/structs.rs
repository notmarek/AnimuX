use std::sync::Arc;

use crate::googledrive::Drive;

use actix_web::{HttpResponse, Responder};
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
    pub drive: Option<Arc<Drive>>,
    pub mal_secret: Option<String>,
    pub mal_client_id: Option<String>,
    pub hcaptcha_enabled: bool,
    pub hcaptcha_sitekey: Option<String>,
    pub hcaptcha_secret: Option<String>,
    pub secret: String,
    pub database: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
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
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub mtime: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedFile {
    pub name: Option<String>,
    pub anime: Option<String>,
    pub group: Option<String>,
    pub episode: Option<String>,
    pub r#type: Option<String>,
    pub mtime: Option<String>,
    pub size: Option<u64>,
    pub mal_id: Option<u32>,
}

impl ParsedFile {
    pub fn from_file(file: File) -> Self {
        let anime_info: Vec<AnimeInfo> = ANIME.get(..).unwrap().to_vec();
        let parsed_file: ParsedFile;

        if file.r#type.as_ref().unwrap() == "file"
            && !(file.name.as_ref().unwrap().contains(".mkv")
                || file.name.as_ref().unwrap().contains(".mp4"))
        {
            parsed_file = ParsedFile {
                name: file.name.clone(),
                anime: file.name,
                group: Some(String::new()),
                episode: Some(String::new()),
                r#type: file.r#type,
                mtime: file.mtime,
                size: file.size,
                mal_id: Some(0),
            };
        } else {
            let mut anitomy: Anitomy = Anitomy::new();
            match anitomy.parse(file.name.as_ref().unwrap()) {
                Ok(ref e) | Err(ref e) => {
                    let mal = &anime_info
                        .into_iter()
                        .filter(|ye| {
                            ye.name.as_ref().unwrap()
                                == &e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string()
                        })
                        .collect::<Vec<AnimeInfo>>();
                    parsed_file = ParsedFile {
                        name: file.name,
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
                        r#type: file.r#type,
                        mtime: file.mtime,
                        size: file.size,
                        mal_id: {
                            if mal.len() < 1 {
                                Some(0)
                            } else {
                                mal[0].mal
                            }
                        },
                    }
                }
            }
        }
        parsed_file
    }
}
