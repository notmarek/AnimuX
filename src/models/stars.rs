use crate::schema::stars;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, Default, Clone, Identifiable)]
#[table_name = "stars"]
pub struct Star {
    pub id: i32,
    pub user_id: i32,
    pub path: String,
}

#[derive(Insertable)]
#[table_name = "stars"]
pub struct NewStar {
    pub user_id: i32,
    pub path: String,
}

impl Star {
    pub fn get_by_uid(
        uid: i32,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Vec<Self>, Vec<Self>> {
        use crate::schema::stars::dsl::*;

        let db = db.get().unwrap();
        match stars.filter(user_id.eq(&uid)).get_results::<Self>(&db) {
            Ok(i) => Ok(i),
            Err(_) => Err(vec![]),
        }
    }

    pub fn get(
        uid: i32,
        p: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::stars::dsl::*;

        let db = db.get().unwrap();
        match stars
            .filter(user_id.eq(&uid))
            .filter(path.eq(&p))
            .get_result::<Self>(&db)
        {
            Ok(i) => i,
            Err(_) => Self {
                id: 0,
                user_id: 0,
                path: String::new(),
            },
        }
    }

    pub fn remove(uid: i32, p: String, db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) {
        use crate::schema::stars::dsl::*;
        let db = db.get().unwrap();
        diesel::delete(stars.filter(user_id.eq(&uid)).filter(path.eq(&p)))
            .execute(&db)
            .unwrap();
    }

    pub fn new(
        uid: i32,
        p: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::stars::dsl::*;
        let exists = Self::get(uid, p.clone(), db);
        if exists.id != 0 {
            return exists;
        };
        let db = db.get().unwrap();
        match diesel::insert_into(stars)
            .values(NewStar {
                user_id: uid,
                path: p,
            })
            .get_result::<Self>(&db)
        {
            Ok(u) => u,
            _ => Self {
                id: 0,
                user_id: 0,
                path: String::new(),
            },
        }
    }
}
