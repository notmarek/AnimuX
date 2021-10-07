use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnilistTitle {
    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
    pub romanji: String,
    pub english: String,
    pub native: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnilistEpisode {
    pub thumbnail: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnilistCoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnilistMedia {
    pub id: i32,
    pub title: AnilistTitle,
    pub description: String,
    pub episodes: i32,
    #[serde(rename = "streamingEpisodes")]
    pub streaming_episodes: Vec<AnilistEpisode>,
    pub source: String,
    #[serde(rename = "bannerImage")]
    pub banner_image: String,
    #[serde(rename = "coverImage")]
    pub cover_image: AnilistCoverImage,
    #[serde(rename = "averageScore")]
    pub average_score: i32,
    #[serde(rename = "isAdult")]
    pub is_adult: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AnilistAnimeInfo {
    #[serde(rename = "Media")]
    pub media: AnilistMedia,
}

pub async fn get_anime_info(id: u32, client: Option<Client>) -> AnilistAnimeInfo {
    let client = client.unwrap_or(Client::new());

    let query: &str = "query ($id: Int) {
        Media (id: $id, type: ANIME) {
            id
            title {
              userPreferred
            }
            description
            episodes
            streamingEpisodes {
              thumbnail
              title
            }
            source
            bannerImage
            coverImage {
                extraLarge
                color
            }
            averageScore
            isAdult		
        }
    }";
    let data = json!({"query": query, "variables": {"id": id}});
    client
        .post("https://graphql.anilist.co/")
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
