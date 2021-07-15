use crate::schema::torrent_queue;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};
use transmission_rpc::types::BasicAuth;
use transmission_rpc::types::TorrentAddArgs;
use transmission_rpc::TransClient;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Torrent {
    pub id: i32,
    pub link: String,
    pub completed: bool,
    pub requested_by: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "torrent_queue"]
pub struct NewTorrent {
    pub link: String,
    pub requested_by: i32,
}

impl NewTorrent {
    pub fn insert(self, db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Torrent {
        use crate::schema::torrent_queue::dsl::*;
        let db = &db.get().unwrap();
        diesel::insert_into(torrent_queue)
            .values(self)
            .get_result::<Torrent>(db)
            .unwrap()
    }
}

impl Torrent {
    pub fn new(
        torrent_link: String,
        requester: i32,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Self {
        use crate::schema::torrent_queue::dsl::*;
        let db = &db.get().unwrap();
        diesel::insert_into(torrent_queue)
            .values(NewTorrent { link: torrent_link, requested_by: requester })
            .get_result::<Self>(db)
            .unwrap()
    }

    pub fn get(
        torrent_id: i32,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> Result<Self, String> {
        use crate::schema::torrent_queue::dsl::*;
        let db = db.get().unwrap();
        match torrent_queue.filter(id.eq(&torrent_id)).first::<Self>(&db) {
            Ok(i) => Ok(i),
            Err(_) => Err(String::from("Torrent not found.")),
        }
    }

    pub fn get_all(pool: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>) -> Vec<Self> {
        use crate::schema::torrent_queue::dsl::*;
        let db = pool.get().unwrap();
        torrent_queue.get_results::<Self>(&db).unwrap()
    }

    pub async fn start(
        self,
        path: String,
        transmission_username: String,
        transmission_password: String,
        transmission_url: String,
        db: &r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
    ) -> String {
        use crate::schema::torrent_queue::dsl::*;
        diesel::update(torrent_queue.find(self.id))
            .set(completed.eq(true))
            .execute(&db.get().unwrap())
            .unwrap();
        let client = TransClient::with_auth(
            &transmission_url,
            BasicAuth {
                user: transmission_username,
                password: transmission_password,
            },
        );
        if client
            .torrent_add(TorrentAddArgs {
                filename: Some(self.link),
                download_dir: Some(path),
                ..TorrentAddArgs::default()
            })
            .await
            .unwrap()
            .is_ok()
        {
            "Torrent added.".to_string()
        } else {
            "There was an error adding the torrent.".to_string()
        }
    }
}
