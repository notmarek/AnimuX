use actix_web::{web, HttpResponse, Responder};

use crate::models::user::RegisterUser;
use crate::structs::{State};

pub async fn register(data: web::Json<RegisterUser>, state: web::Data<State>) -> impl Responder {
    
    println!("{:#?}", data);
    if state.hcaptcha_enabled && data.hcaptcha_userverify.is_none() {
        return HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"status\": \"error\", \"data\": \"No captcha code passed\"}")
    }
    return HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"status\": \"success\", \"data\": \"Good job\"}")
}