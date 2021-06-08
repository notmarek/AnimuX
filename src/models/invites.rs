use serde::{Deserialize, Serialize};
use crate::schema::invites;
use diesel::prelude::*;

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