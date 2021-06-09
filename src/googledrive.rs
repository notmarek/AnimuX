extern crate google_drive3 as drive3;

extern crate hyper;

extern crate hyper_rustls;

use anitomy::{Anitomy, ElementCategory};

use async_trait::async_trait;

use drive3::DriveHub;

use yup_oauth2;
use yup_oauth2::{read_application_secret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

use crate::structs::{AnimeInfo, ParsedFile, ANIME};

pub struct Drive {
    pub hub: DriveHub,
    api_key: String,
}

#[async_trait]
pub trait GoogleDrive {
    async fn init(secret_file: &str, api_key: &str, id: &str) -> Self;
    async fn get_files_in_folder(&self, id: &str) -> drive3::api::FileList;
    async fn get_direct_link(&self, file: &drive3::api::File) -> String;
    async fn get_shared_drives(&self) -> drive3::api::DriveList;
    async fn parse_file(&self, file: google_drive3::api::File) -> ParsedFile;
}
#[async_trait]
impl GoogleDrive for Drive {
    async fn init(secret_file: &str, api_key: &str, id: &str) -> Self {
        let secret = read_application_secret(secret_file)
            .await
            .expect("failed to read gsuite credential file");
        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk(&format!("tokencache-{}.json", id))
                .build()
                .await
                .unwrap();
        let hub = DriveHub::new(
            hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
            auth,
        );
        Self {
            hub,
            api_key: api_key.to_string(),
        }
    }
    async fn get_files_in_folder(&self, id: &str) -> drive3::api::FileList {
        self.hub
            .files()
            .list()
            .supports_all_drives(true)
            .include_items_from_all_drives(true)
            .q(&format!("'{}' in parents and trashed=false", id))
            .include_team_drive_items(false)
            .spaces("drive")
            .param("fields", "files(id,name,modifiedTime,size,mimeType)")
            .doit()
            .await
            .unwrap()
            .1
    }
    async fn get_direct_link(&self, file: &drive3::api::File) -> String {
        format!(
            "https://www.googleapis.com/drive/v3/files/{}/?key={}&alt=media",
            file.id.as_ref().unwrap(),
            self.api_key
        )
    }
    async fn get_shared_drives(&self) -> drive3::api::DriveList {
        self.hub
            .drives()
            .list()
            .page_size(90)
            .doit()
            .await
            .unwrap()
            .1
    }
    async fn parse_file(&self, file: google_drive3::api::File) -> ParsedFile {
        let anime_info: Vec<AnimeInfo> = ANIME.clone();
        let parsed_file: ParsedFile;
        let url;
        let file_type = match file
            .mime_type
            .as_ref()
            .unwrap_or(&"file".to_string())
            .contains("folder")
        {
            true => {
                url = file.id.clone().unwrap();
                "directory".to_string()
            }

            _ => {
                url = self.get_direct_link(&file).await;
                "file".to_string()
            }
        };

        if file_type == "file"
            && !(file.name.as_ref().unwrap().contains(".mkv")
                || file.name.as_ref().unwrap().contains(".mp4"))
        {
            parsed_file = ParsedFile {
                name: Some(url),
                anime: file.name,
                group: Some(String::new()),
                episode: Some(String::new()),
                kind: Some(file_type),
                mtime: Some(
                    chrono::DateTime::parse_from_rfc3339(&file.modified_time.unwrap())
                        .unwrap()
                        .format("%a, %d %b %Y %T %Z")
                        .to_string(),
                ),
                size: Some(
                    file.size
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0_u64),
                ),
                mal_id: Some(0),
            };
        } else {
            let mut anitomy: Anitomy = Anitomy::new();
            match anitomy.parse(file.name.as_ref().unwrap()) {
                Ok(ref e) | Err(ref e) => {
                    let mal = &anime_info
                        .into_iter()
                        .filter(|i| {
                            i.name.as_ref().unwrap()
                                == e.get(ElementCategory::AnimeTitle).unwrap_or("")
                        })
                        .collect::<Vec<AnimeInfo>>();
                    parsed_file = ParsedFile {
                        name: Some(url),

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
                        kind: Some(file_type),
                        mtime: Some(
                            chrono::DateTime::parse_from_rfc3339(&file.modified_time.unwrap())
                                .unwrap()
                                .format("%a, %d %b %Y %T %Z")
                                .to_string()
                                .replace("+00:00", "UTC"),
                        ),
                        size: Some(
                            file.size
                                .unwrap_or(String::new())
                                .parse::<u64>()
                                .unwrap_or(0 as u64),
                        ),
                        mal_id: mal.get(0).map_or(Some(0), |mal| mal.mal),
                    }
                }
            }
        }
        parsed_file
    }
}
