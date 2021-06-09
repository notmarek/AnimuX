use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use crate::models::user::{JsonUserAuth, Roles, User};
use crate::structs::{HCaptchaResponse, Response, State};
use reqwest;

pub async fn register(data: web::Json<JsonUserAuth>, state: web::Data<State>) -> impl Responder {
    if state.hcaptcha_enabled {
        if data.hcaptcha_userverify.is_none() {
            return HttpResponse::Ok().content_type("application/json").body(
                Response {
                    status: String::from("error"),
                    data: "Captcha response couldn't be found.",
                }
                .json(),
            );
        }
        let form: HashMap<_, _> = std::array::IntoIter::new([
            ("response", data.hcaptcha_userverify.as_ref().unwrap()),
            ("secret", state.hcaptcha_secret.as_ref().unwrap()),
            ("sitekey", state.hcaptcha_sitekey.as_ref().unwrap()),
        ])
        .collect();
        let client = reqwest::Client::new();
        let resp: HCaptchaResponse = client
            .post("https://hcaptcha.com/siteverify")
            .form(&form)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        if !resp.success {
            return HttpResponse::Ok().json(Response {
                status: String::from("error"),
                data: "Can't you even do the captcha man?",
            });
        }
    }
    if data.invite.is_none() || data.invite.as_ref().unwrap().len() < 8 {
        return HttpResponse::Ok().json(Response {
            status: String::from("error"),
            data: "You need to specify a valid invite.",
        });
    }
    let user = User::register(
        data.username.clone(),
        data.password.clone(),
        data.invite.as_ref().unwrap().clone(),
        Roles::Member,
        state.secret.clone(),
        &state.database,
    );

    match user {
        Ok(u) => {
            if state.navidrome_enabled {
                state
                    .navidrome
                    .clone()
                    .unwrap()
                    .create_account(data.username.clone(), data.password.clone())
                    .await;
            }
            if state.mango_enabled {
                state
                    .mango
                    .clone()
                    .unwrap()
                    .create_account(data.username.clone(), data.password.clone())
                    .await;
            }
            return HttpResponse::Ok().json(Response {
                status: String::from("success"),
                data: u,
            });
        }
        Err(e) => {
            return HttpResponse::Ok().json(Response {
                status: String::from("error"),
                data: e,
            });
        }
    }
}

pub async fn login(data: web::Json<JsonUserAuth>, state: web::Data<State>) -> impl Responder {
    let user = User::login(
        data.username.clone(),
        data.password.clone(),
        state.secret.clone(),
        &state.database,
    );
    match user {
        Ok(u) => {
            return HttpResponse::Ok().json(Response {
                status: String::from("success"),
                data: u,
            });
        }
        Err(e) => {
            return HttpResponse::Ok().json(Response {
                status: String::from("error"),
                data: e,
            });
        }
    }
}

pub async fn all_users(state: web::Data<State>) -> impl Responder {
    let users: Vec<User> = User::get_all(&state.database);
    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: users,
    })
}
