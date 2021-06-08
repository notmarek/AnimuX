use crate::schema::users;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonUserAuth {
    pub username: String,
    pub password: String,
    pub hcaptcha_userverify: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: i32,
}

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

impl User {
    pub fn register(
        raw_username: String,
        raw_password: String,
        raw_role: Roles,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<User, String> {
        use crate::schema::users::dsl::*;

        let db = db.get().unwrap();

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
                    Ok(u) => Ok(u),
                    Err(e) => Err(format!("{}", e)),
                }
            }
        }
    } 
    pub fn login(
        raw_username: String,
        raw_password: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<User, String> {
        use crate::schema::users::dsl::*;

        let db = db.get().unwrap();

        let mut hasher = Sha256::new();
        hasher.update(raw_password);
        let hashed_password = format!("{:x}", hasher.finalize());
        match users.filter(username.eq(&raw_username)).filter(password.eq(hashed_password)).first::<User>(&db) {
            Ok(u) => Ok(u),
            Err(_) => {
                Err(String::from("Username or password do not match."))
            }
        }
    }
}
