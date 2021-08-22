use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::models::rssmission::{Config, Server};
use crate::models::torrents::{get_torrent_name, NewTorrent, Torrent};
use crate::models::user::User;
use crate::structs::{Response, State};
use diesel::{prelude::*, r2d2};
use serde::{Deserialize, Serialize};
use std::fs;

fn load_cfg(state: &State) -> Config {
    let config_file: String = fs::read_to_string(&state.rssmission_config.as_ref().unwrap())
        .expect("Something went wrong reading the configuration file");
    serde_json::from_str(&String::from(config_file)).unwrap()
}

pub async fn current_cfg(state: web::Data<State>) -> impl Responder {
    let mut config = load_cfg(&state);
    config.server = Some(Server { url: Some("classified".to_string()), username: Some("root".to_string()), password: Some("*****".to_string())});
    HttpResponse::Ok().json(config)
}

pub async fn update_cfg(state: web::Data<State>) -> impl Responder {
    
}