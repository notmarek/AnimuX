use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnilistTitle {
    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
    pub romaji: String,
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
    pub color: Option<String>,
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
    pub banner_image: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ALAnimeData {
    pub data: AnilistAnimeInfo,
}
pub async fn get_anime_info(id: u32, client: Option<Client>) -> ALAnimeData {
    let client = client.unwrap_or(Client::new());

    let query: &str = "query ($id: Int) {
        Media (id: $id, type: ANIME) {
            id
            title {
              userPreferred
              romaji
              english
              native
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

pub async fn search_anime(q: String, client: Option<Client>) -> ALAnimeData {
    let client = client.unwrap_or(Client::new());

    let query: &str = "query ($q: String) {
        Media(type: ANIME, search: $q) {
          id
          title {
            userPreferred
            romaji
            english
            native
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
    let data = json!({"query": query, "variables": {"q": q}});
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
