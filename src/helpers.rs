use std::cmp::Ordering;

use crate::structs::*;

use crate::googledrive::{Drive, GoogleDrive};
pub fn parse_files(files: Vec<File>) -> Vec<ParsedFile> {
    let mut parsed_files: Vec<ParsedFile> = Vec::new();
    files
        .into_iter()
        .for_each(|file| parsed_files.push(ParsedFile::from_file(file)));
    parsed_files
}

pub async fn parse_google_files(
    files: Vec<google_drive3::api::File>,
    drive: &Drive,
) -> Vec<ParsedFile> {
    let mut parsed_files: Vec<ParsedFile> = Vec::new();
    for file in files {
        parsed_files.push(drive.parse_file(file).await)
    }
    parsed_files
}

pub fn file_sort(a: &ParsedFile, b: &ParsedFile) -> Ordering {
    if a.r#type.as_ref().unwrap() == &"file".to_string()
        && b.r#type.as_ref().unwrap() == &"file".to_string()
        && a.anime.as_ref().unwrap_or(&"~~~".to_string())
            == b.anime.as_ref().unwrap_or(&"~~~".to_string())
    {
        a.episode
            .as_ref()
            .unwrap_or(&"~~~".to_string())
            .to_lowercase()
            .cmp(
                &b.episode
                    .as_ref()
                    .unwrap_or(&"~~~".to_string())
                    .to_lowercase(),
            )
    } else {
        a.anime
            .as_ref()
            .unwrap_or(&"~~~".to_string())
            .to_lowercase()
            .cmp(
                &b.anime
                    .as_ref()
                    .unwrap_or(&"~~~".to_string())
                    .to_lowercase(),
            )
    }
}
