use crate::schema::invites;
use diesel::prelude::*;
use diesel::r2d2;
use rand::{distributions::Alphanumeric, Rng};

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
    pub fn get(
        raw_invite: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Self, String> {
        use crate::schema::invites::dsl::*;

        let db = db.get().unwrap();
        match invites.filter(invite.eq(&raw_invite)).first::<Invite>(&db) {
            Ok(i) => Ok(i),
            Err(_) => Err(String::from("Invite not found.")),
        }
    }

    pub fn generate(db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Self {
        use crate::schema::invites::dsl::*;
        let db = db.get().unwrap();
        let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
        let inv = NewInvite { invite: s };
        match diesel::insert_into(invites)
            .values(inv)
            .get_result::<Invite>(&db)
        {
            Ok(u) => u,
            _ => Invite {
                id: 0,
                invite: String::new(),
                used: false,
            },
        }
    }
    pub fn mark_as_used(
        self,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<(), String> {
        use crate::schema::invites::dsl::*;

        let db = db.get().unwrap();
        match diesel::update(invites.find(self.id))
            .set(used.eq(true))
            .execute(&db)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }
}
