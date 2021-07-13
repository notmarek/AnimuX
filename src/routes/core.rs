use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::helpers::file_sort;
use crate::index as global_index;
use crate::structs::{Directory, File, ParsedFile, State, StorageThing};

pub fn directory_index_to_files(index: Directory) -> Vec<ParsedFile> {
    let mut files: Vec<ParsedFile> = Vec::new();
    index.files.into_iter().for_each(|f| match f {
        StorageThing::Directory(dir) => {
            let file = File {
                name: Some(dir.name.clone()),
                path: Some(dir.name.clone()),
                kind: Some("directory".to_string()),
                mtime: dir.mtime,
                size: None,
            };
            files.push(ParsedFile::from_file(file));
        }
        StorageThing::File(file) => {
            files.push(file);
        }
        StorageThing::Empty(_) => {}
    });
    files
}

pub fn get_path_from_index(index: Directory, path: String, iteration: u8) -> Directory {
    let mut new_index = index.clone();
    index.files.into_iter().for_each(|f| match f {
        StorageThing::Directory(dir) => {
            let split_path: Vec<&str> = path.split("/").collect();
            if split_path[iteration as usize] == dir.name.clone() {
                if iteration < split_path.len() as u8 - 1 {
                    new_index = get_path_from_index(dir, path.clone(), iteration + 1);
                } else {
                    new_index = dir;
                }
            }
        }
        _ => {}
    });
    new_index
}

pub async fn files(req: HttpRequest, state: web::Data<State>) -> impl Responder {
    let path = req
        .match_info()
        .get("tail")
        .unwrap()
        .parse::<String>()
        .unwrap()
        .replace(&state.base_path, "/");
    unsafe {
        let index = get_path_from_index(global_index.clone().unwrap(), path, 0);
        let mut parsed_files = directory_index_to_files(index);
        parsed_files.sort_by(|a, b| file_sort(a, b));
        HttpResponse::Ok().json(parsed_files)
    }
}

// pub async fn files(req: HttpRequest, data: web::Data<State>) -> impl Responder {
//     let path = req
//         .match_info()
//         .get("tail")
//         .unwrap()
//         .parse::<String>()
//         .unwrap()
//         .replace(&data.base_path, "/");
//     let paths: fs::ReadDir = fs::read_dir(&format!("{}{}", data.root_folder, path)).unwrap();
//     let files = paths
//         .into_iter()
//         .map(|path| {
//             let metadata = path.as_ref().unwrap().metadata().unwrap();
//             let modification_time: DateTime<Utc> = metadata.modified().unwrap().into();
//             File {
//                 name: Some(path.as_ref().unwrap().file_name().into_string().unwrap()),
//                 path: Some(path.as_ref().unwrap().path().to_str().unwrap().to_string()),
//                 kind: Some(String::from(match metadata.is_dir() {
//                     true => "directory",
//                     false => "file",
//                 })),
//                 mtime: Some(modification_time.format("%a, %d %b %Y %T %Z").to_string()),
//                 size: Some(metadata.len()),
//             }
//         })
//         .collect();
//     let mut parsed_files: Vec<ParsedFile> = parse_files(files);
//     parsed_files.sort_by(|a, b| file_sort(a, b));
//     HttpResponse::Ok().json(parsed_files)
// }
