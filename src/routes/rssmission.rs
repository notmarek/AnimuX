use crate::models::rssmission::{Config, Feed, Matcher, Server};
use crate::structs::State;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

fn load_cfg(state: &State) -> Config {
    let config_file: String = fs::read_to_string(&state.rssmission_config.as_ref().unwrap())
        .expect("Something went wrong reading the configuration file");
    let mut cfg: Config = serde_json::from_str(&String::from(config_file)).unwrap();
    cfg.server = Some(Server {
        url: Some("classified".to_string()),
        username: Some("root".to_string()),
        password: Some("*****".to_string()),
    });
    cfg
}

fn commit_cfg(cfg: &Config, state: &State) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(state.rssmission_config.as_ref().unwrap())
        .unwrap();
    file.set_len(0).unwrap();
    file.write(&serde_json::to_string(&cfg).unwrap().as_bytes())
        .unwrap();
    file.flush().unwrap();
}

pub async fn current_cfg(state: web::Data<State>) -> impl Responder {
    let config = load_cfg(&state);
    HttpResponse::Ok().json(config)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCFGJson {
    pub feed_url: String,       // the feed to be updated/created
    pub regexp: Option<String>, // regex to be added
    pub path: Option<String>,   // location of the matcher
}

pub async fn add_matcher(
    state: web::Data<State>,
    update: web::Json<UpdateCFGJson>,
) -> impl Responder {
    let mut config = load_cfg(&state);
    let mut feeds: Vec<Feed> = vec![];
    let mut added = false;
    for mut feed in config.feeds.clone().unwrap() {
        if !added && feed.url.as_ref().unwrap() == &update.feed_url {
            let mut matchers = feed.matchers.clone().unwrap();
            matchers.push(Matcher {
                regexp: update.regexp.clone(),
                path: update.path.clone(),
            });
            feed.matchers = Some(matchers);
            added = true;
        }
        feeds.push(feed);
    }
    if !added {
        feeds.push(Feed {
            url: Some(update.feed_url.clone()),
            matchers: Some(vec![Matcher {
                regexp: update.regexp.clone(),
                path: update.path.clone(),
            }]),
        });
    }
    config.feeds = Some(feeds);
    commit_cfg(&config, &state);
    HttpResponse::Ok().json(config)
}

pub async fn remove_matcher(
    state: web::Data<State>,
    update: web::Json<UpdateCFGJson>,
) -> impl Responder {
    let mut config = load_cfg(&state);
    let mut feeds: Vec<Feed> = vec![];
    let mut removed = false;
    for mut feed in config.feeds.clone().unwrap() {
        if !removed && feed.url.as_ref().unwrap() == &update.feed_url {
            let mut matchers = feed.matchers.clone().unwrap();
            let index = matchers
                .iter()
                .position(|matcher| matcher.path == update.path || matcher.regexp == update.regexp)
                .unwrap_or(133769);
            if index != 133769 {
                matchers.remove(index);
                feed.matchers = Some(matchers);
            }
            removed = true;
        }
        feeds.push(feed);
    }
    config.feeds = Some(feeds);
    commit_cfg(&config, &state);
    HttpResponse::Ok().json(config)
}
