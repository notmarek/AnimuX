use actix_web::{web, HttpRequest, Responder};

use crate::helpers::file_sort;
use crate::structs::{Directory, File, ParsedFile, State, StorageThing};
use crate::INDEX;

use serde::Deserialize;

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
    index.files.into_iter().for_each(|f| {
        if let StorageThing::Directory(dir) = f {
            let split_path: Vec<&str> = path.split('/').collect();
            if split_path[iteration as usize] == dir.name {
                if iteration < split_path.len() as u8 - 1 {
                    new_index = get_path_from_index(dir, path.clone(), iteration + 1);
                } else {
                    new_index = dir;
                }
            }
        }
    });
    new_index
}

pub fn search_dir(
    full_dir: Directory,
    new_dir: Directory,
    parent: String,
    query: String,
) -> Directory {
    let mut new_index = new_dir;
    full_dir.files.into_iter().for_each(|f| match f {
        StorageThing::Directory(dir) => {
            new_index = search_dir(
                dir.clone(),
                new_index.clone(),
                format!("{}/{}", parent, dir.name),
                query.clone(),
            );
        }
        StorageThing::File(file) => {
            if file
                .anime
                .as_ref()
                .unwrap()
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                new_index.files.push(StorageThing::File(file));
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
        let index = get_path_from_index(INDEX.clone().unwrap(), path, 0);
        let mut parsed_files = directory_index_to_files(index);
        parsed_files.sort_by(|a, b| file_sort(a, b));
        crate::coolshit::encrypted_json_response(parsed_files, &state.response_secret)
    }
}
#[derive(Deserialize)]
pub struct Search {
    #[serde(rename = "q")]
    pub query: String,
}

pub async fn filter_files(data: web::Query<Search>, state: web::Data<State>) -> impl Responder {
    unsafe {
        let index = search_dir(
            INDEX.clone().unwrap(),
            Directory {
                name: "Search".to_string(),
                files: vec![],
                mtime: Some(String::new()),
            },
            String::new(),
            data.query.clone(),
        );
        let mut parsed_files = directory_index_to_files(index);
        parsed_files.sort_by(|a, b| file_sort(a, b));
        crate::coolshit::encrypted_json_response(parsed_files, &state.response_secret)
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
//     crate::coolshit::encrypted_json_response(parsed_files)
// }
