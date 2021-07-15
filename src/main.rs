#![feature(once_cell)]
#![feature(async_closure)]
#[macro_use]
extern crate diesel;

mod database;
mod googledrive;
mod helpers;
mod mango;
mod models;
mod navidrome;
mod routes;
mod schema;
mod structs;

use actix_web::HttpResponse;

use actix_web::http::HeaderName;
use actix_web::web::Data;
use http::HeaderValue;
use mango::Mango;
use navidrome::Navidrome;
use routes::admin::dynamic_merge;
use routes::admin::flatten_index;
use routes::admin::index_folder;
use routes::core::*;
use routes::gdrive::gdrive;
use routes::mal;

use structs::*;

use std::env;
use std::str::FromStr;

use actix_service::Service;
use actix_web::{web, App, HttpServer};
use googledrive::{Drive, GoogleDrive};

use std::sync::Arc;

use crate::models::user::User;
use crate::routes::admin::create_invite;
use crate::routes::admin::get_all_invites;
use crate::routes::admin::index_files;
use crate::routes::images::upload;
use crate::routes::requests::approve_request;
use crate::routes::requests::request_torrent;
use crate::routes::requests::show_all_requests;
use crate::routes::user::all_users;
use crate::routes::user::check_token;
use crate::routes::user::login;
use crate::routes::user::register;

static mut INDEX: Option<Directory> = None;

fn is_enabled(name: &str) -> bool {
    !name.is_empty() && (name.to_lowercase() == "true" || name.to_lowercase() == "yes")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db = database::establish_connection();
    let mut state: State = State {
        app_name: String::from("Animu"),
        base_path: String::new(),
        drive: None,
        mal_client_id: None,
        mal_secret: None,
        hcaptcha_enabled: false,
        hcaptcha_secret: None,
        hcaptcha_sitekey: None,
        secret: env::var("SECRET").unwrap_or_else(|_| String::from("weaksecret")),
        database: db,
        mango_enabled: false,
        mango: None,
        navidrome_enabled: false,
        navidrome: None,
        default_upload_path: None,
        root_folder: "/home/pi/Y/Animu/".to_string(),
        trans_address: None,
        trans_password: None,
        trans_username: None,
    };
    let address: String = env::var("ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port: String = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    state.root_folder = env::var("ROOT_FOLDER").unwrap_or_else(|_| "/home/pi/Y/Animu/".to_string());
    let hcaptcha_enabled: String = env::var("HCAPTCHA_ENABLED").unwrap_or_default();
    let drive_enabled: String = env::var("ENABLE_GDRIVE").unwrap_or_default();
    let mal_enabled: String = env::var("ENABLE_MAL").unwrap_or_default();
    let torrents_enabled: String = env::var("ENABLE_TORRENTS").unwrap_or_default();
    let navidrome_enabled: String = env::var("ENABLE_NAVIDROME").unwrap_or_default();
    let mango_enabled: String = env::var("ENABLE_MANGO").unwrap_or_default();
    let image_upload_enabled: String = env::var("ENABLE_UPLOADER").unwrap_or_default();

    if is_enabled(&torrents_enabled) {
        println!("Torrent requests enabled.");
        state.trans_username =
            Some(env::var("TRANSMISSION_USERNAME").expect("TRANSMISSION_USERNAME not found."));
        state.trans_username =
            Some(env::var("TRANSMISSION_PASSWORD").expect("TRANSMISSION_PASSWORD not found."));
        state.trans_username =
            Some(env::var("TRANSMISSION_ADDRESS").expect("TRANSMISSION_ADDRESS not found."));
    }

    if is_enabled(&navidrome_enabled) {
        println!("Navidrome enabled.");
        let navidrome_username: String =
            env::var("NAVIDROME_USERNAME").expect("NAVIDROME_USERNAME not found.");
        let navidrome_password: String =
            env::var("NAVIDROME_PASSWORD").expect("NAVIDROME_PASSWORD not found.");
        let navidrome_url: String = env::var("NAVIDROME_URL").expect("NAVIDROME_URL not found.");
        state.navidrome_enabled = true;
        state.navidrome = Some(
            Navidrome::new(navidrome_url, navidrome_username, navidrome_password)
                .await
                .unwrap(),
        );
        println!(
            "Navidrome logged in as '{}'.",
            state.navidrome.clone().unwrap().login.username
        );
    }

    if is_enabled(&image_upload_enabled) {
        println!("Image uploader enabled.");
        let uploader_path: String = env::var("UPLOADER_PATH").expect("UPLOADER_PATH not found.");
        state.default_upload_path = Some(uploader_path);
    }

    if is_enabled(&mango_enabled) {
        println!("Mango enabled.");
        let mango_username: String = env::var("MANGO_USERNAME").expect("MANGO_USERNAME not found.");
        let mango_password: String = env::var("MANGO_PASSWORD").expect("MANGO_PASSWORD not found.");
        let mango_url: String = env::var("MANGO_URL").expect("MANGO_URL not found.");
        state.mango_enabled = true;
        state.mango = Some(
            Mango::new(mango_url, mango_username, mango_password)
                .await
                .unwrap(),
        );
        println!("Mango logged in.");
    }

    if is_enabled(&hcaptcha_enabled) {
        println!("HCaptcha enabled.");
        state.hcaptcha_enabled = true;
        state.hcaptcha_sitekey =
            Some(env::var("HCAPTCHA_SITEKEY").expect("HCAPTCHA_SITEKEY not found."));
        state.hcaptcha_secret =
            Some(env::var("HCAPTCHA_SECRET").expect("HCAPTCHA_SECRET not found."));
    }

    if is_enabled(&drive_enabled) {
        println!("Google drive enabled.");
        let drive_api_key: String = env::var("GDRIVE_API_KEY").expect("GDRIVE_API_KEY not found.");
        let drive_secret_file: String =
            env::var("GDRIVE_APP_SECRET").expect("GDRIVE_APP_SECRET not found.");
        let drive: Drive = Drive::init(&drive_secret_file, &drive_api_key, "drive").await;
        state.drive = Some(Arc::new(drive));
    }

    if is_enabled(&mal_enabled) {
        println!("MAL enabled.");
        let mal_secret: String = env::var("MAL_SECRET").expect("MAL_SECRET not found.");
        let mal_client_id: String = env::var("MAL_CLIENT_ID").expect("MAL_CLIENT_ID not found.");
        state.mal_client_id = Some(mal_client_id);
        state.mal_secret = Some(mal_secret);
    }

    let base_path: String = env::var("BASE_PATH").unwrap_or_else(|_| "/".to_string());
    state.base_path = base_path.clone();
    unsafe {
        let mut i: Directory = index_folder(state.root_folder.clone(), true);
        i = flatten_index(flatten_index(i));
        INDEX = Some(dynamic_merge(i));
    }
    HttpServer::new(move || {
        let st = state.clone();
        let mut app = App::new().wrap_fn(move |req, srv| {
            let mut original = false;
            let mut response = None;
            let mut fut = None;
            if req.method() == http::Method::OPTIONS {
                original = false;
            } else if req.path().contains(&format!("{}user", st.base_path)) {
                original = true;
            } else if req.headers().contains_key("authorization") {
                if let Ok(user) = User::from_token(
                    String::from(
                        req.headers()
                            .get("authorization")
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    ),
                    st.secret.clone(),
                    &st.database,
                ) {
                    if req.path().contains(&format!("{}admin", st.base_path)) && user.role == 2 {
                        println!("{} tried accessing {}, but wasnt an admin.", user.username, req.path());
                        original = false;
                    } else {
                        println!("{} accessed {}", user.username, req.path());
                        original = true;
                    }
                }
            }
            if !original {
                response = Some(
                    req.into_response(
                        HttpResponse::Forbidden()
                            .content_type("application/json")
                            .body(
                                Response {
                                    status: String::from("error"),
                                    data: String::from("Access denied."),
                                }
                                .json(),
                            ),
                    ),
                );
            } else {
                fut = Some(srv.call(req));
            }
            async move {
                let mut r = match original {
                    true => fut.unwrap().await.unwrap(),
                    false => response.unwrap(),
                };
                let headers = r.headers_mut();
                headers.insert(
                    HeaderName::from_str("Access-Control-Allow-Origin").unwrap(),
                    HeaderValue::from_static("*"),
                );
                headers.insert(
                    HeaderName::from_str("Access-Control-Allow-Headers").unwrap(),
                    HeaderValue::from_static("Content-Type, Authorization"),
                );
                Ok(r)
            }
        });
        if is_enabled(&torrents_enabled) {
            app = app
                .route(
                    &format!("{}torrents/request", &base_path),
                    web::post().to(request_torrent),
                )
                .route(
                    &format!("{}torrents/show", &base_path),
                    web::get().to(show_all_requests),
                )
                .route(
                    &format!("{}admin/torrents/approve", &base_path),
                    web::post().to(approve_request),
                );
        }
        if is_enabled(&drive_enabled) {
            app = app
                .route(&format!("{}GoogleDrive", &base_path), web::get().to(gdrive))
                .route(
                    &format!("{}GoogleDrive/{{tail:.*}}", &base_path),
                    web::get().to(gdrive),
                );
        }
        if is_enabled(&mal_enabled) {
            app = app
                .route(&format!("{}map", &base_path), web::get().to(mal::map))
                .route(
                    &format!("{}mal/link", &base_path),
                    web::get().to(mal::malurl),
                )
                .route(
                    &format!("{}mal/anime", &base_path),
                    web::post().to(mal::malanime),
                )
                .route(
                    &format!("{}mal/user", &base_path),
                    web::post().to(mal::maluser),
                )
                .route(
                    &format!("{}mal/oauth2", &base_path),
                    web::post().to(mal::malauth),
                )
                .route(
                    &format!("{}mal/list/update/anime", &base_path),
                    web::post().to(mal::malupdateanimelist),
                );
        }
        if is_enabled(&image_upload_enabled) {
            app = app.route(
                &format!("{}images/upload", &base_path),
                web::post().to(upload),
            )
        }
        app = app
            .route(
                &format!("{}user/register", &base_path),
                web::post().to(register),
            )
            .route(
                &format!("{}user/check_token", &base_path),
                web::post().to(check_token),
            )
            .route(&format!("{}user/login", &base_path), web::post().to(login))
            .route(
                &format!("{}admin/all_users", &base_path),
                web::get().to(all_users),
            )
            .route(
                &format!("{}admin/reindex", &base_path),
                web::get().to(index_files),
            )
            .route(
                &format!("{}admin/create_invite", &base_path),
                web::post().to(create_invite),
            )
            .route(
                &format!("{}admin/invites", &base_path),
                web::get().to(get_all_invites),
            )
            .route(&base_path.to_string(), web::get().to(files)) // Default route
            .route(&format!("{}{{tail:.*}}", &base_path), web::get().to(files)) // Default route
            .app_data(Data::new(state.clone()));
        app
    })
    .bind((address, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
