use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use sha2::digest::generic_array::typenum::private::IsGreaterPrivate;

use crate::models::invites::{Invite, NewInvite};
use crate::models::user::{JsonUserAuth, Roles, User};
use crate::structs::{HCaptchaResponse, Response, State};

pub async fn create_invite(state: web::Data<State>) -> impl Responder {
    let inv = Invite::generate(&state.database);
    return HttpResponse::Ok().content_type("application/json").body(
        Response {
            status: String::from("success"),
            data: inv.invite,
        }
        .json(),
    );
}