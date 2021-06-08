use actix_web::{web, HttpResponse, Responder};

use crate::models::user::{JsonUserAuth, NewUser, Roles, User};
use crate::structs::{Response, State};

pub async fn register(data: web::Json<JsonUserAuth>, state: web::Data<State>) -> impl Responder {
    println!("{:#?}", data);
    if state.hcaptcha_enabled && data.hcaptcha_userverify.is_none() {
        return HttpResponse::Ok().content_type("application/json").body(
            Response {
                status: String::from("error"),
                data: "Captcha response couldn't be found.",
            }
            .json(),
        );
    }
    let user = User::register(
        data.username.clone(),
        data.password.clone(),
        Roles::Member,
        &state.database,
    );


    match user {
        Ok(u) => {
            println!("{:#?}", u);
            return HttpResponse::Ok().content_type("application/json").body(
                Response {
                    status: String::from("success"),
                    data: u,
                }
                .json(),
            );
        },
        Err(e) => {
            return HttpResponse::Ok().content_type("application/json").body(
                Response {
                    status: String::from("error"),
                    data: e,
                }
                .json(),
            );
        }
    }
    // return Response { status: String::from("error"), data: "Captcha response couldn't be found." }.send()
    // return HttpResponse::Ok().content_type("application/json").body(
    //     Response {
    //         status: String::from("success"),
    //         data: user,
    //     }
    //     .json(),
    // );
}

pub async fn login(data: web::Json<JsonUserAuth>, state: web::Data<State>) -> impl Responder {
    println!("{:#?}", data);
    "haha"
}
