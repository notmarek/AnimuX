use actix_web::{web, HttpRequest, Responder};

use crate::models::torrents::{get_torrent_name, NewTorrent, Torrent};
use crate::models::user::User;
use crate::structs::{Response, State};
use diesel::{prelude::*, r2d2};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct JsonRequestTorrent {
    link: String,
}
#[derive(Clone, Deserialize)]
pub struct JsonRequestApprove {
    id: i32,
    path: String,
}
#[derive(Clone, Deserialize)]
pub struct JsonRequestRemove {
    id: i32,
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
    match get_torrent_name(
        data.link.clone(),
        state.trans_username.clone().unwrap(),
        state.trans_password.clone().unwrap(),
        state.trans_address.clone().unwrap(),
    )
    .await
    {
        Ok(name) => {
            let new_torrent = NewTorrent {
                link: data.link.clone(),
                name,
                requested_by: user.id,
            };
            new_torrent.insert(&state.database);
            crate::coolshit::encrypted_json_response(
                Response {
                    status: String::from("success"),
                    data: "Torrent added to queue.",
                },
                &state.response_secret,
            )
        }
        Err(e) => crate::coolshit::encrypted_json_response(
            Response {
                status: String::from("error"),
                data: e,
            },
            &state.response_secret,
        ),
    }
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
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: "Torrent approved and added to transmission",
        },
        &state.response_secret,
    )
}

pub async fn delete_request(
    data: web::Json<JsonRequestRemove>,
    state: web::Data<State>,
) -> impl Responder {
    let torrent = Torrent::get(data.id, &state.database).unwrap();
    torrent.remove(&state.database);
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: "Request removed.",
        },
        &state.response_secret,
    )
}

#[derive(Serialize, Clone, Deserialize)]
pub struct TorrentButFancy {
    id: i32,
    name: String,
    link: String,
    requested_by: String,
    completed: bool,
}

impl TorrentButFancy {
    pub fn from_torrent(
        t: Torrent,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> TorrentButFancy {
        let username = User::get(t.requested_by, db).username;
        TorrentButFancy {
            id: t.id,
            name: t.name,
            link: t.link,
            requested_by: username,
            completed: t.completed,
        }
    }

    pub fn from_torrents(
        torrents: Vec<Torrent>,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Vec<TorrentButFancy> {
        torrents
            .into_iter()
            .map(|t| TorrentButFancy::from_torrent(t, db))
            .collect()
    }
}

pub async fn show_all_requests(state: web::Data<State>) -> impl Responder {
    let torrents = Torrent::get_all(&state.database);
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: TorrentButFancy::from_torrents(torrents, &state.database),
        },
        &state.response_secret,
    )
}
