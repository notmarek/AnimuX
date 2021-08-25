use std::collections::HashMap;

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder};

use crate::models::user::{JsonUserAuth, Roles, User};
use crate::structs::{HCaptchaResponse, Response, State};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthThing {
    pub token: String,
    pub uri: String,
    pub ip: String,
}

pub async fn check_token(
    _req: HttpRequest,
    state: web::Data<State>,
    data: web::Form<AuthThing>,
) -> impl Responder {
    if let Ok(user) = User::from_token(data.token.clone(), state.secret.clone(), &state.database) {
        println!("{} accessed {}", user.username, data.uri);
        HttpResponse::new(StatusCode::OK)
    } else {
        println!(
            "{} tried using the token \"{}\" to access {}",
            data.ip, data.token, data.uri
        );
        HttpResponse::new(StatusCode::FORBIDDEN)
    }
}

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
            println!("Someone has failed the captcha.");
            return crate::coolshit::encrypted_json_response(
                Response {
                    status: String::from("error"),
                    data: "Can't you even do the captcha man?",
                },
                &state.response_secret,
            );
        }
    }
    if data.invite.is_none() || data.invite.as_ref().unwrap().len() < 8 {
        println!(
            "Someone has used an invalid invite - \"{}\"",
            data.invite.as_ref().unwrap()
        );
        return crate::coolshit::encrypted_json_response(
            Response {
                status: String::from("error"),
                data: "You need to specify a valid invite.",
            },
            &state.response_secret,
        );
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
            println!("New user has been registered - \"{}\"", &data.username);
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
            crate::coolshit::encrypted_json_response(
                Response {
                    status: String::from("success"),
                    data: u,
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

pub async fn login(data: web::Json<JsonUserAuth>, state: web::Data<State>) -> impl Responder {
    let user = User::login(
        data.username.clone(),
        data.password.clone(),
        state.secret.clone(),
        &state.database,
    );
    match user {
        Ok(u) => crate::coolshit::encrypted_json_response(
            Response {
                status: String::from("success"),
                data: u,
            },
            &state.response_secret,
        ),
        Err(e) => crate::coolshit::encrypted_json_response(
            Response {
                status: String::from("error"),
                data: e,
            },
            &state.response_secret,
        ),
    }
}

pub async fn all_users(state: web::Data<State>) -> impl Responder {
    let users: Vec<User> = User::get_all(&state.database);
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: users,
        },
        &state.response_secret,
    )
}
