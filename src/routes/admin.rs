use actix_web::{web, HttpResponse, Responder};

use std::fs;

use chrono::{DateTime, Utc};

use crate::structs::{Directory, File, ParsedFile, Response, State, StorageThing};
use crate::INDEX;

use crate::models::invites::Invite;

pub async fn create_invite(state: web::Data<State>) -> impl Responder {
    let inv = Invite::generate(&state.database);
    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: inv.invite,
    })
}

pub async fn get_all_invites(state: web::Data<State>) -> impl Responder {
    let invites: Vec<Invite> = Invite::get_all(&state.database);
    crate::coolshit::encrypted_json_response(Response {
        status: String::from("success"),
        data: invites,
    }, &state.response_secret)
}

pub fn index_folder(folder: String, root_folder: bool) -> Directory {
    let paths: fs::ReadDir = fs::read_dir(&folder).unwrap();
    let folder_name: String = fs::canonicalize(&folder)
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let dir_metadata = fs::metadata(&folder).unwrap();
    let modification_time: DateTime<Utc> = dir_metadata.modified().unwrap().into();

    let index: Vec<StorageThing> = paths
        .into_iter()
        .map(|path| {
            let metadata = path.as_ref().unwrap().metadata().unwrap();
            let modification_time: DateTime<Utc> = metadata.modified().unwrap().into();
            if !root_folder
                && !path
                    .as_ref()
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .contains("Animu")
            {
                StorageThing::Empty(String::new())
            } else if metadata.is_dir() {
                StorageThing::Directory(index_folder(
                    path.as_ref().unwrap().path().to_str().unwrap().to_string(),
                    false,
                ))
            } else {
                let file = File {
                    name: Some(path.as_ref().unwrap().file_name().into_string().unwrap()),
                    path: Some(path.as_ref().unwrap().path().to_str().unwrap().to_string()),
                    kind: Some("file".to_string()),
                    mtime: Some(modification_time.format("%a, %d %b %Y %T %Z").to_string()),
                    size: Some(metadata.len()),
                };
                StorageThing::File(ParsedFile::from_file(file))
            }
        })
        .collect();

    Directory {
        name: folder_name,
        files: index,

        mtime: Some(modification_time.format("%a, %d %b %Y %T %Z").to_string()),
    }
}

pub fn flatten_index(index: Directory) -> Directory {
    let mut files: Vec<StorageThing> = Vec::new();
    index.files.into_iter().for_each(|f| match f {
        StorageThing::Directory(mut dir) => {
            files.append(dir.files.as_mut());
        }
        StorageThing::File(file) => {
            files.push(StorageThing::File(file));
        }
        StorageThing::Empty(_) => {}
    });

    Directory {
        name: "Animu".to_string(),
        files,
        mtime: index.mtime,
    }
}

pub fn merge_folders(index: Directory, to_merge: &str) -> Directory {
    let mut files: Vec<StorageThing> = Vec::new();
    let mut merged_folder_files: Vec<StorageThing> = Vec::new();
    index.files.into_iter().for_each(|f| match f {
        StorageThing::Directory(mut dir) => {
            if dir.name.ends_with(to_merge) {
                merged_folder_files.append(dir.files.as_mut());
            } else {
                files.push(StorageThing::Directory(dir));
            }
        }
        StorageThing::File(file) => {
            files.push(StorageThing::File(file));
        }
        StorageThing::Empty(_) => {}
    });
    let merged_folder = Directory {
        name: to_merge.to_string(),
        files: merged_folder_files,
        mtime: None,
    };
    files.push(StorageThing::Directory(merged_folder));
    Directory {
        name: "Animu".to_string(),
        files,
        mtime: index.mtime,
    }
}

pub fn dynamic_merge(index: Directory) -> Directory {
    let mut dir_names: Vec<String> = Vec::new();
    let mut to_merge: Vec<String> = Vec::new();
    let mut new_index = index.clone();
    for f in index.files {
        if let StorageThing::Directory(dir) = f {
            if !dir_names.contains(&dir.name) {
                dir_names.push(dir.name);
            } else if !to_merge.contains(&dir.name) {
                to_merge.push(dir.name);
            }
        }
    }
    to_merge.into_iter().for_each(|dir_name| {
        new_index = merge_folders(new_index.clone(), &dir_name);
    });
    new_index
}

pub async fn index_files(state: web::Data<State>) -> impl Responder {
    unsafe {
        let mut i: Directory = index_folder(state.root_folder.clone(), true);
        i = flatten_index(flatten_index(i));
        INDEX = Some(dynamic_merge(i));
    }

    HttpResponse::Ok().json(Response {
        status: String::from("success"),
        data: "Reindexed all files.",
    })
}
