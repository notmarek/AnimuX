#![feature(once_cell)]

mod googledrive;
mod helpers;
mod routes;
mod structs;

use routes::core::*;
use routes::gdrive::gdrive;

use routes::mal;
use structs::*;

use std::env;

use actix_web::{web, App, HttpServer};
use googledrive::{Drive, GoogleDrive};

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let mut state: State = State {
        app_name: String::from("Animu"),
        base_path: String::new(),
        drive: None,
        mal_client_id: None,
        mal_secret: None,
    };
    let address: String = env::var("ADDRESS").unwrap_or(String::from("127.0.0.1"));
    let port: String = env::var("PORT").unwrap_or(String::from("8080"));

    let drive_enabled: String = env::var("ENABLE_GDRIVE").unwrap_or(String::new());
    if drive_enabled.to_lowercase() == "true" || drive_enabled.to_lowercase() == "yes" {
        println!("MAL enabled.");
        let drive_api_key: String = env::var("GDRIVE_API_KEY").expect("GDRIVE_API_KEY not found.");
        let drive_secret_file: String =
            env::var("GDRIVE_APP_SECRET").expect("GDRIVE_APP_SECRET not found.");
        let drive: Drive = Drive::init(&drive_secret_file, &drive_api_key, "drive").await;
        state.drive = Some(Arc::new(drive));
    }

    let mal_enabled: String = env::var("ENABLE_MAL").unwrap_or(String::new());
    if mal_enabled.to_lowercase() == "true" || mal_enabled.to_lowercase() == "yes" {
        println!("Google Drive enabled.");
        let mal_secret: String = env::var("MAL_SECRET").expect("MAL_SECRET not found.");
        let mal_client_id: String = env::var("MAL_CLIENT_ID").expect("MAL_CLIENT_ID not found.");
        state.mal_client_id = Some(mal_client_id);
        state.mal_secret = Some(mal_secret);
    }

    let base_path: String = env::var("BASE_PATH").unwrap_or("/".to_string());
    state.base_path = base_path.clone();
    HttpServer::new(move || {
        let mut app = App::new();
        if drive_enabled.to_lowercase() == "true" || drive_enabled.to_lowercase() == "yes" {
            app = app.route(&format!("{}GoogleDrive", &base_path), web::get().to(gdrive));
            app = app.route(
                &format!("{}GoogleDrive/{{tail:.*}}", &base_path),
                web::get().to(gdrive),
            );
        }
        if mal_enabled.to_lowercase() == "true" || mal_enabled.to_lowercase() == "yes" {
            app = app.route(&format!("{}map", &base_path), web::get().to(mal::map));
            app = app.route(
                &format!("{}mal/link", &base_path),
                web::get().to(mal::malurl),
            );
            app = app.route(
                &format!("{}mal/anime", &base_path),
                web::post().to(mal::malanime),
            );
            app = app.route(
                &format!("{}mal/user", &base_path),
                web::post().to(mal::maluser),
            );
            app = app.route(
                &format!("{}mal/oauth2", &base_path),
                web::post().to(mal::malauth),
            );
            app = app.route(
                &format!("{}mal/list/update/anime", &base_path),
                web::post().to(mal::malupdateanimelist),
            );
        }

        app = app.route(&format!("{}", &base_path), web::get().to(files)); // Default route
        app = app.route(&format!("{}{{tail:.*}}", &base_path), web::get().to(files)); // Default route
        app = app.data(state.clone());
        app
    })
    .bind((address, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
