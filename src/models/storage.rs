// use crate::schema::storage;
// use diesel::prelude::*;
// use diesel::r2d2;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Queryable, Serialize, Deserialize, Default)]
// pub struct Storage {
//     pub id: i32,
//     pub paths: Vec<String>,
//     pub name: String,
//     pub exceptions: Vec<String>,
// }

// #[derive(Insertable)]
// #[table_name = "storage"]
// pub struct NewStorageEntry {
//     pub paths: Vec<String>,
//     pub name: String,
//     pub exceptions: Vec<String>,
// }

// impl Storage {
//     pub fn get(
//         p: String,
//         db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
//     ) -> Result<Self, String> {
//         use crate::schema::storage::dsl::*;
//         let db = db.get().unwrap();
//         match storage.filter(path.eq(&p)).first::<Self>(&db) {
//             Ok(e) => Ok(e),
//             Err(_) => Err(String::from("Folder not found.")),
//         }
//     }

//     pub fn new(
//         n: String,
//         p: Vec<String>,
//         e: Vec<String>,
//         db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
//     ) -> Self {
//         use crate::schema::storage::dsl::*;
//         let db = db.get().unwrap();
//         let entry = NewStorageEntry {
//             paths: p,
//             name: n,
//             exceptions: e,
//         };
//         match diesel::insert_into(storage)
//             .values(entry)
//             .get_result::<Self>(&db)
//         {
//             Ok(u) => u,
//             _ => Self::default(),
//         }
//     }
// }
