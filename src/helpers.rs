use std::cmp::Ordering;

use crate::structs::*;

// use crate::googledrive::{Drive, GoogleDrive};
// pub fn parse_files(files: Vec<File>) -> Vec<ParsedFile> {
//     files
//         .into_iter()
//         .map(ParsedFile::from_file)
//         .collect()
// }

// pub async fn parse_google_files(
//     files: Vec<google_drive3::api::File>,
//     drive: &Drive,
// ) -> Vec<ParsedFile> {
//     let mut parsed_files: Vec<ParsedFile> = Vec::new();
//     for file in files {
//         parsed_files.push(drive.parse_file(file).await)
//     }
//     parsed_files
// }

pub fn storage_thing_sort(a: StorageThing, b: StorageThing) -> Ordering {
    match a {
        StorageThing::Directory(_) => Ordering::Less, 
        StorageThing::File(file_a) => {
            match b {
                StorageThing::Directory(_) => {
                    Ordering::Less
                },
                StorageThing::File(file_b) => file_sort(&file_a, &file_b),
                StorageThing::Empty(_) => {
                    Ordering::Less
                }
            }
        },
        StorageThing::Empty(_) => Ordering::Less
    }
}

pub fn file_sort(a: &ParsedFile, b: &ParsedFile) -> Ordering {
    if a.kind.as_ref().unwrap() == "file"
        && b.kind.as_ref().unwrap() == "file"
        && a.anime.as_ref().map_or("~~~", |it| it.as_str())
            == b.anime.as_ref().map_or("~~~", |it| it.as_str())
    {
        a.episode
            .as_ref()
            .map_or_else(|| "~~~".to_string(), |v| v.to_lowercase())
            .cmp(
                &b.episode
                    .as_ref()
                    .map_or_else(|| "~~~".to_string(), |v| v.to_lowercase()),
            )
    } else {
        a.anime
            .as_ref()
            .map_or_else(|| "~~~".to_string(), |v| v.to_lowercase())
            .cmp(
                &b.anime
                    .as_ref()
                    .map_or_else(|| "~~~".to_string(), |v| v.to_lowercase()),
            )
    }
}
