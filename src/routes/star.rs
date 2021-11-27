use crate::models::stars::Star;
use crate::models::user::User;
use crate::structs::{Response, State};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StarJson {
    path: String,
}

pub async fn star(
    req: HttpRequest,
    state: web::Data<State>,
    data: web::Json<StarJson>,
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
    Star::new(user.id, data.path.clone(), &state.database);
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: "star created",
        },
        &state.response_secret,
    )
}

pub async fn unstar(
    req: HttpRequest,
    state: web::Data<State>,
    data: web::Json<StarJson>,
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
    Star::remove(user.id, data.path.clone(), &state.database);
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: "star removed",
        },
        &state.response_secret,
    )
}

pub async fn stars(req: HttpRequest, state: web::Data<State>) -> impl Responder {
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
    crate::coolshit::encrypted_json_response(
        Response {
            status: String::from("success"),
            data: Star::get_by_uid(user.id, &state.database),
        },
        &state.response_secret,
    )
}
