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
    let paths: fs::ReadDir = fs::read_dir(&format!("{}{}", data.root_folder, path)).unwrap();
    let files = paths
        .into_iter()
        .map(|path| {
            let metadata = path.as_ref().unwrap().metadata().unwrap();
            let modification_time: DateTime<Utc> = metadata.modified().unwrap().into();
            File {
                name: Some(path.as_ref().unwrap().file_name().into_string().unwrap()),
                kind: Some(String::from(match metadata.is_dir() {
                    true => "directory",
                    false => "file",
                })),
                mtime: Some(modification_time.format("%a, %d %b %Y %T %Z").to_string()),
                size: Some(metadata.len()),
            }
        })
        .collect();
    let mut parsed_files: Vec<ParsedFile> = parse_files(files);
    parsed_files.sort_by(|a, b| file_sort(a, b));
    HttpResponse::Ok().json(parsed_files)
}
