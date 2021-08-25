use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::helpers::parse_google_files;
use crate::structs::State;

use std::path::PathBuf;

use crate::googledrive::GoogleDrive;
use crate::helpers::file_sort;

pub async fn gdrive(req: HttpRequest, data: web::Data<State>) -> impl Responder {
    let path = PathBuf::from(
        req.path()
            .replace(&format!("{}GoogleDrive", data.base_path), "/"),
    );
    let new_path;
    let drive = data.drive.as_ref().unwrap();
    if path == PathBuf::from("/") {
        new_path = "root".to_string();
    } else {
        new_path = path.file_name().unwrap().to_str().unwrap().to_string();
    }
    let google_files = drive.get_files_in_folder(&new_path).await.files.unwrap();
    let mut files = parse_google_files(google_files, drive).await;
    files.sort_by(|a, b| file_sort(a, b));
    crate::coolshit::encrypted_json_response(files, &data.response_secret)
}
