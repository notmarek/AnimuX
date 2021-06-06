#![feature(once_cell)]

mod googledrive;
mod helpers;
mod routes;
mod structs;

use routes::core::*;

use structs::*;

use std::{env, sync::Mutex};

use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpServer, Responder, Route, http, middleware::Logger, web};
use googledrive::{Drive, GoogleDrive};

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut state: State = State {
        app_name: String::from("Animu"),
        drive: None,
        mal_client_id: None,
        mal_secret: None,
    };

    let drive_enabled: String = env::var("ENABLE_GDRIVE").unwrap_or(String::new());
    if drive_enabled.to_lowercase() == "true" || drive_enabled.to_lowercase() == "yes" {
        let drive_api_key: String = env::var("GDRIVE_API_KEY").expect("GDRIVE_API_KEY not found.");
        let drive_secret_file: String =
            env::var("GDRIVE_APP_SECRET").expect("GDRIVE_APP_SECRET not found.");
        let drive: Drive = Drive::init(&drive_secret_file, &drive_api_key, "drive").await;
        state.drive = Some(Arc::new(drive));
    }

    let mal_enabled: String = env::var("ENABLE_MAL").unwrap_or(String::new());
    if mal_enabled.to_lowercase() == "true" || mal_enabled.to_lowercase() == "yes" {
        let mal_secret: String = env::var("MAL_SECRET").expect("MAL_SECRET not found.");
        let mal_client_id: String = env::var("MAL_CLIENT_ID").expect("MAL_CLIENT_ID not found.");
        state.mal_client_id = Some(mal_client_id);
        state.mal_secret = Some(mal_secret);
    }

    let base_path: String = env::var("BASE_PATH").unwrap_or("/".to_string());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_origin()
            .allow_any_header()
            .max_age(3600);
        let mut app = App::new();
        if drive_enabled.to_lowercase() == "true" || drive_enabled.to_lowercase() == "yes" {}
        if mal_enabled.to_lowercase() == "true" || mal_enabled.to_lowercase() == "yes" {}
        app = app.route(&format!("{}", &base_path), web::get().to(files)); // Default route
        app = app.route(&format!("{}{{tail:.*}}", &base_path), web::get().to(files)); // Default route
        app = app.data(state.clone());
        app.wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
