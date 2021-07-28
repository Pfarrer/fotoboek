use diesel::{self, prelude::*};
use serde::Serialize;

use crate::db::schema::folders;
use crate::db::schema::folders::dsl::*;

#[derive(Insertable, Queryable, Serialize, Debug)]
#[table_name = "folders"]
pub struct Folder {
    pub abs_path: String,
    pub image_id: i32,
    pub distance: i32,
}

impl Folder {
    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Folder> {
        folders.load::<Folder>(conn).expect("Query folders failed")
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<(), String> {
        diesel::insert_into(folders)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}
