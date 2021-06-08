use crate::schema::invites;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct Invite {
    pub id: i32,
    pub invite: String,
    pub used: bool,
}

#[derive(Insertable)]
#[table_name = "invites"]
pub struct NewInvite {
    pub invite: String,
}

impl Invite {
    pub fn get(raw_invite: String, db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Result<Self, String> {
        use crate::schema::invites::dsl::*;

        let db = db.get().unwrap();
        match invites.filter(invite.eq(&raw_invite)).first::<Invite>(&db) {
            Ok(i) => Ok(i),
            Err(_) => Err(String::from("Invite not found.")),
        }
    }


    pub fn mark_as_used(self, db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Result<(), String> {
        use crate::schema::invites::dsl::*;

        let db = db.get().unwrap();
        match diesel::update(invites.find(self.id)).set(used.eq(true)).execute(&db) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
        
    }
}
