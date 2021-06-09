use actix_web::{web, HttpResponse, Responder};
use async_std::io::Bytes;
use futures::FutureExt;
use futures::Stream;
use futures::StreamExt;

use std::collections::HashMap;

use crate::structs::MALAnime;
use crate::structs::MALAnimeUpdate;
use crate::structs::MALReply;
use crate::structs::MALUser;
use crate::structs::State;
use crate::structs::ANIME;

pub trait JsonStream {
    fn json_stream(self, r: reqwest::Response) -> HttpResponse;
}

impl JsonStream for actix_web::HttpResponseBuilder {
    fn json_stream(mut self, r: reqwest::Response) -> HttpResponse {
        self.content_type("application/json").streaming(
            r.bytes_stream()
                .map(|it| Ok::<_, actix_web::Error>(it.unwrap())),
        )
    }
}

// #[post("/mal/list/update/anime", format = "json", data = "<data>")]
pub async fn malupdateanimelist(data: web::Json<MALAnimeUpdate>) -> impl Responder {
    let mut form = HashMap::new();
    form.insert("status", data.status.clone());
    form.insert(
        "num_watched_episodes",
        data.num_watched_episodes.to_string(),
    );
    let client = reqwest::Client::new();
    let r = client
        .patch(&format!(
            "https://api.myanimelist.net/v2/anime/{}/my_list_status",
            data.anime_id
        ))
        .header("Authorization", format!("Bearer {}", data.token))
        .form(&form)
        .send()
        .await
        .unwrap();
    HttpResponse::Ok().json_stream(r)
}

// #[post("/mal/oauth2", format = "json", data = "<auth>")]
pub async fn malauth(auth: web::Json<MALReply>, config: web::Data<State>) -> impl Responder {
    let form: HashMap<&str, &str> = std::array::IntoIter::new([
        ("client_id", config.mal_client_id.as_ref().unwrap().as_str()),
        (
            "client_secret",
            config.mal_secret.as_ref().unwrap().as_str(),
        ),
        ("code", &auth.code),
        ("code_verifier", &auth.state),
        ("grant_type", "authorization_code"),
    ])
    .collect();
    let client = reqwest::Client::new();
    let r = client
        .post("https://myanimelist.net/v1/oauth2/token")
        .form(&form)
        .send()
        .await
        .unwrap();
    HttpResponse::Ok().json_stream(r)
}

// #[post("/mal/user", format = "json", data = "<data>")]
pub async fn maluser(data: web::Json<MALUser>) -> impl Responder {
    let client = reqwest::Client::new();
    let r = client
        .get(format!(
            "https://api.myanimelist.net/v2/users/{}",
            data.user
        ))
        .header("Authorization", format!("Bearer {}", data.token))
        .send()
        .await
        .unwrap();
    HttpResponse::Ok().json_stream(r)
}

// #[post("/mal/anime", format = "json", data = "<data>")]
pub async fn malanime(data: web::Json<MALAnime>) -> impl Responder {
    let client = reqwest::Client::new();
    let r = client
        .get(format!(
            "https://api.myanimelist.net/v2/anime/{}?fields=my_list_status,num_episodes",
            data.anime_id
        ))
        .header("Authorization", format!("Bearer {}", data.token))
        .send()
        .await
        .unwrap();
    HttpResponse::Ok().json_stream(r)
}

//#[get("/mal/link")]
pub async fn malurl() -> impl Responder {
    let code_verify = pkce::code_verifier(128);
    let code_challenge = pkce::code_challenge(&code_verify);
    HttpResponse::Found().with_header(("Location", format!("https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id=0e16733a4d9bbf1152fa9cb2ada84048&code_challenge={}&state={}", code_challenge, code_challenge)))
}

// //#[get("/map")]
pub async fn map() -> impl Responder {
    HttpResponse::Ok().json(&*ANIME)
}
