use std::convert::TryInto;

use crate::models::invites::Invite;
use crate::schema::users;
use base64::{decode, encode};
use diesel::prelude::*;
use diesel::r2d2;
use libaes::Cipher;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonUserAuth {
    pub username: String,
    pub password: String,
    pub hcaptcha_userverify: Option<String>,
    pub invite: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: i32,
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum Roles {
    Member = 0,
    PowerUser = 1,
    Admin = 2,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedIn {
    pub token: String,
    pub message: String,
}
impl User {
    pub fn create_token(self, secret: String) -> String {
        let data = serde_json::to_string(&self).unwrap();
        let key: &[u8; 16] = &secret.as_bytes()[..16].try_into().unwrap();
        let cipher = Cipher::new_128(key);
        let encrypted = cipher.cbc_encrypt(b"0000000000000000", &data.as_bytes()[..]);
        encode(encrypted)
    }
    pub fn from_token(
        token: String,
        secret: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Self, String> {
        use crate::schema::users::dsl::*;
        let db = db.get().unwrap();
        let key: &[u8; 16] = &secret.as_bytes()[..16].try_into().unwrap();
        let cipher = Cipher::new_128(key);
        let decoded = &decode(token).unwrap_or(Vec::new());
        if decoded.len() == 0 {
            return Err(String::from("Encrypted string seems fucked."))
        }
        let decrypted: Vec<u8> = cipher.cbc_decrypt(b"0000000000000000", &decoded[..]);        
        let data = String::from_utf8(decrypted).unwrap_or(String::new());
        let data: User = serde_json::from_str(&data).unwrap_or(User {id: 0, username: String::new(), password: String::new(), role: 0});
        match users.filter(id.eq(&data.id)).first::<User>(&db) {
            Ok(u) => Ok(u),
            Err(e) => Err(format!("{}", e)),
        }
    }
    pub fn register(
        raw_username: String,
        raw_password: String,
        invite: String,
        raw_role: Roles,
        secret: String,
        pool: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<LoggedIn, String> {
        use crate::schema::users::dsl::*;
        let invite = match Invite::get(invite, &pool) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        if invite.used {
            return Err(String::from("Invite already used."));
        }
        let db = pool.get().unwrap();

        let mut hasher = Sha256::new();
        hasher.update(raw_password);
        let hashed_password = format!("{:x}", hasher.finalize());
        let user = NewUser {
            username: raw_username.clone(),
            password: hashed_password,
            role: raw_role as i32,
        };
        match users.filter(username.eq(&raw_username)).first::<User>(&db) {
            Ok(_) => Err(String::from("Username already taken.")),
            Err(_) => {
                match diesel::insert_into(users)
                    .values(user)
                    .get_result::<User>(&db)
                {
                    Ok(u) => {
                        match invite.mark_as_used(&pool) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        };
                        Ok(LoggedIn {
                            token: u.create_token(secret),
                            message: String::from("Account successfully created."),
                        })
                    }
                    Err(e) => Err(format!("{}", e)),
                }
            }
        }
    }
    pub fn login(
        raw_username: String,
        raw_password: String,
        secret: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<LoggedIn, String> {
        use crate::schema::users::dsl::*;

        let db = db.get().unwrap();

        let mut hasher = Sha256::new();
        hasher.update(raw_password);
        let hashed_password = format!("{:x}", hasher.finalize());
        match users
            .filter(username.eq(&raw_username))
            .filter(password.eq(hashed_password))
            .first::<User>(&db)
        {
            Ok(u) => Ok(LoggedIn {
                token: u.create_token(secret),
                message: String::from("Succesfully logged in."),
            }),
            Err(_) => Err(String::from("Username or password do not match.")),
        }
    }
}
