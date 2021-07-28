use diesel::{self, insert_into, prelude::*};
use serde::Serialize;

use crate::db::schema::images;
use crate::db::schema::images::dsl::*;
use std::path::PathBuf;

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "images"]
pub struct Image {
    pub id: Option<i32>,
    pub filename: String,
    pub abs_path: String,
}

impl Image {
    pub fn from_path_buf(source_path: &PathBuf) -> Image {
        Image {
            id: None,
            filename: source_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            abs_path: source_path.to_string_lossy().into_owned(),
        }
    }

    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Image> {
        images.load::<Image>(conn).expect("Query all images")
    }
    pub fn by_id(conn: &diesel::SqliteConnection, image_id: i32) -> Result<Image, String> {
        images
            .filter(id.eq(image_id))
            .first::<Image>(conn)
            .map_err(|err| err.to_string())
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<Option<Image>, String> {
        insert_into(images)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;
        let image = images
            .filter(abs_path.eq(self.abs_path))
            .limit(1)
            .load(conn)
            .map_err(|err| err.to_string())?
            .into_iter()
            .next();
        Ok(image)
    }
}
