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
            abs_path: source_path.to_string_lossy().into_owned()
        }
    }

    pub fn all(c: &diesel::SqliteConnection) -> Vec<Image> {
        images.load::<Image>(c).expect("Query all images")
    }
    pub fn by_id(c: &diesel::SqliteConnection, image_id: i32) -> Option<Image> {
        images.filter(id.eq(image_id)).first::<Image>(c).ok()
    }

    pub fn insert(&self, c: &diesel::SqliteConnection) -> Result<(), ()> {
        insert_into(images)
            .values(self)
            .execute(c)
            .map(|_| ())
            .map_err(|_| ())
    }
}
