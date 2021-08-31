use actix_web::{HttpResponse, http::header};
use serde::Serialize;
use libaes::Cipher;
use std::{convert::TryInto};


pub fn encrypted_json_response(value: impl Serialize, secret: &str) -> HttpResponse {
    let data = serde_json::to_string(&value).unwrap();
    let key: &[u8; 16] = &secret.as_bytes()[..16].try_into().unwrap();
    let cipher = Cipher::new_128(key);
    let encrypted = cipher.cbc_encrypt(b"0000000000000000", data.as_bytes());
    let enc_string = base64::encode(encrypted).replace("=", "");
    let shifted: Vec<u8> = enc_string.as_bytes().iter().map(|f| {
        f + 13
    }).collect();

    HttpResponse::Ok().append_header(header::ContentType::png()).body(shifted)
}