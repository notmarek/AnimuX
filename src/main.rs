#![feature(once_cell)]
#![feature(async_closure)]
#[macro_use]
extern crate diesel;

mod database;
mod googledrive;
mod helpers;
mod models;
mod routes;
mod schema;
mod structs;

use actix_web::Route;
use actix_web::dev::ServiceResponse;
use actix_web::HttpResponse;

use actix_web::guard::Method;
use actix_web::http::HeaderName;
use http::HeaderValue;
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
use crate::routes::user::login;
use crate::routes::user::register;

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
        secret: env::var("SECRET").unwrap_or(String::from("weaksecret")),
        database: db,
    };
    let address: String = env::var("ADDRESS").unwrap_or(String::from("127.0.0.1"));
    let port: String = env::var("PORT").unwrap_or(String::from("8080"));
    let hcaptcha_enabled: String = env::var("HCAPTCHA_ENABLED").unwrap_or(String::new());
    let drive_enabled: String = env::var("ENABLE_GDRIVE").unwrap_or(String::new());
    let mal_enabled: String = env::var("ENABLE_MAL").unwrap_or(String::new());

    if hcaptcha_enabled.to_lowercase() == "true" || hcaptcha_enabled.to_lowercase() == "yes" {
        println!("HCaptcha enabled.");
        state.hcaptcha_enabled = true;
        state.hcaptcha_sitekey =
            Some(env::var("HCAPTCHA_SITEKEY").expect("HCAPTCHA_SITEKEY not found."));
        state.hcaptcha_secret =
            Some(env::var("HCAPTCHA_SECRET").expect("HCAPTCHA_SECRET not found."));
    }

    if drive_enabled.to_lowercase() == "true" || drive_enabled.to_lowercase() == "yes" {
        println!("MAL enabled.");
        let drive_api_key: String = env::var("GDRIVE_API_KEY").expect("GDRIVE_API_KEY not found.");
        let drive_secret_file: String =
            env::var("GDRIVE_APP_SECRET").expect("GDRIVE_APP_SECRET not found.");
        let drive: Drive = Drive::init(&drive_secret_file, &drive_api_key, "drive").await;
        state.drive = Some(Arc::new(drive));
    }

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
        let st = state.clone();
        let mut app = App::new().wrap_fn(move |req, srv| {
            let mut res = None;
            let mut fut = None;
            if req.method() == http::Method::OPTIONS {
                fut = Some(srv.call(req));
                // let r = ServiceResponse::new(req.into_parts().0, HttpResponse::Ok().finish());
                // res = Some(r);
            } else if !&req.path().contains(&format!("{}user", st.base_path))
                && (!&req.headers().contains_key("authorization")
                    || &req.headers().get("authorization").unwrap().len() < &5
                    || match User::from_token(
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
                        Ok(_) => false,
                        Err(_) => true,
                    })
            {
                let r = ServiceResponse::new(
                    req.into_parts().0,
                    HttpResponse::Forbidden()
                        .content_type("application/json")
                        .body(
                            Response {
                                status: String::from("error"),
                                data: String::from("Access denied."),
                            }
                            .json(),
                        ),
                );
                res = Some(r);
            } else {
                fut = Some(srv.call(req));
            }

            async {
                let mut r;
                if res.is_none() {
                    r = fut.unwrap().await.unwrap();
                } else {
                    r = res.unwrap();
                }
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
            app = app.route(&format!("{}admin/create_invite", &base_path), Route::new().method(http::Method::OPTIONS).to(create_invite));
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
        //
        app = app.route(
            &format!("{}user/register", &base_path),
            web::post().to(register),
        );
        app = app.route(&format!("{}user/login", &base_path), web::post().to(login));
        app = app.route(&format!("{}admin/create_invite", &base_path), web::post().to(create_invite));
        //
        app = app.route(&format!("{}", &base_path), web::get().to(files)); // Default route
        app = app.route(&format!("{}{{tail:.*}}", &base_path), web::get().to(files)); // Default route
        app = app.data(state.clone());
        app
    })
    .bind((address, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
