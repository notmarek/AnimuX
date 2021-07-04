use actix_web::{web, HttpRequest, HttpResponse, Responder};
use async_std::fs::create_dir_all;
use rand::{distributions::Alphanumeric, Rng};

use mime_db::extension;
use std::io::Write;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

use crate::{
    models::user::User,
    structs::{Response, State},
};

pub async fn upload(
    mut payload: Multipart,
    req: HttpRequest,
    state: web::Data<State>,
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

    let mut field = payload.try_next().await.unwrap().unwrap();

    let extension = extension(
        field
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .unwrap_or("bin");

    let user_folder = format!(
        "{}/{}",
        state.default_upload_path.as_ref().unwrap(),
        &user.username
    );
    println!("{} uploaded a file.", &user.username);
    create_dir_all(&user_folder).await.unwrap();
    let filename: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let filepath = format!("{}/{}.{}", user_folder, &filename, &extension);

    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap();
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f = web::block(move || f.as_ref().unwrap().write_all(&data).map(|_| f))
            .await
            .unwrap()
            .unwrap();
    }

    return HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: format!(
            "https://i.notmarek.com/{}/{}.{}",
            &user.username, filename, extension
        ),
    });
}
