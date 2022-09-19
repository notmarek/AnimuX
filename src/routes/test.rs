use crate::{structs::Response, utils::anilist_scraper::search_anime};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub q: String,
}
pub async fn test_search(q: web::Query<Search>) -> impl Responder {
    let data = search_anime(q.q.clone(), None).await;
    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data,
    })
}
