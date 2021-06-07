use actix_web::{web, HttpRequest, HttpResponse, Responder};

use std::fs;

use chrono::{DateTime, Utc};

use crate::helpers::{file_sort, parse_files};
use crate::structs::{File, ParsedFile, State};

pub async fn files(req: HttpRequest, data: web::Data<State>) -> impl Responder {
    let path = req
        .match_info()
        .get("tail")
        .unwrap()
        .parse::<String>()
        .unwrap()
        .replace(&data.base_path, "/");
    let mut files: Vec<File> = Vec::new();
    let paths: fs::ReadDir = fs::read_dir(&format!("/home/pi/Y/Animu/{}", path)).unwrap();
    paths.into_iter().for_each(|path| {
        let metadata = path.as_ref().unwrap().metadata().unwrap();
        let modification_time: DateTime<Utc> = metadata.modified().unwrap().into();
        let file: File = File {
            name: Some(path.as_ref().unwrap().file_name().into_string().unwrap()),
            r#type: Some(String::from(match metadata.is_dir() {
                true => "directory",
                false => "file",
            })),
            mtime: Some(modification_time.format("%a, %d %b %Y %T %Z").to_string()),
            size: Some(metadata.len()),
        };
        files.push(file);
    });
    let mut parsed_files: Vec<ParsedFile> = parse_files(files);
    parsed_files.sort_by(|a, b| file_sort(a, b));
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&parsed_files).unwrap())
}
