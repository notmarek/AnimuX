use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::models::torrents::{NewTorrent, Torrent};
use crate::models::user::User;
use crate::structs::{Response, State};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct JsonRequestTorrent {
    link: String,
}
#[derive(Clone, Deserialize)]
pub struct JsonRequestApprove {
    id: i32,
    path: String,
}

pub async fn request_torrent(
    req: HttpRequest,
    data: web::Json<JsonRequestTorrent>,
    state: web::Data<State>,
) -> impl Responder {
    let user = User::from_token(
        req.headers()
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        state.secret.clone(),
        &state.database,
    )
    .unwrap();
    let new_torrent = NewTorrent {
        link: data.link.clone(),
        requested_by: user.id,
    };
    new_torrent.insert(&state.database);
    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: "Torrent added to queue.",
    })
}

pub async fn approve_request(
    data: web::Json<JsonRequestApprove>,
    state: web::Data<State>,
) -> impl Responder {
    let torrent = Torrent::get(data.id, &state.database).unwrap();
    torrent
        .start(
            data.path.clone(),
            state.trans_username.clone().unwrap(),
            state.trans_password.clone().unwrap(),
            state.trans_address.clone().unwrap(),
            &state.database,
        )
        .await;
    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: "Torrent approved and added to transmission",
    })
}

pub async fn show_all_requests(state: web::Data<State>) -> impl Responder {
    let torrents = Torrent::get_all(&state.database);
    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: torrents,
    })
}
