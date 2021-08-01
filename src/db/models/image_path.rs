use diesel::{self, prelude::*};
use serde::Serialize;

use crate::db::schema::image_paths;
use crate::db::schema::image_paths::dsl::*;

#[derive(Insertable, Queryable, Serialize, Debug)]
#[table_name = "image_paths"]
pub struct ImagePath {
    pub abs_dir_path: String,
    pub image_id: i32,
    pub distance: i32,
    pub parent_dir_path: Option<String>,
}

impl ImagePath {
    pub fn all(conn: &diesel::SqliteConnection) -> Vec<ImagePath> {
        image_paths
            .load::<ImagePath>(conn)
            .expect("Query image_paths failed")
    }

    pub fn by_abs_dir_path(
        conn: &diesel::SqliteConnection,
        by_abs_dir_path: &str,
        max_distance: i32,
    ) -> Vec<ImagePath> {
        image_paths
            .filter(
                abs_dir_path
                    .eq(by_abs_dir_path)
                    .and(distance.le(max_distance)),
            )
            .load::<ImagePath>(conn)
            .expect("Query image_paths failed")
    }

    pub fn subdirs_of(
        conn: &diesel::SqliteConnection,
        by_parent_dir_path: Option<&str>,
    ) -> Vec<String> {
        match by_parent_dir_path {
            Some(path) => image_paths
                .select(abs_dir_path)
                .distinct()
                .filter(parent_dir_path.eq(path))
                .load::<String>(conn)
                .expect("Query image_paths failed"),
            None => image_paths
                .select(abs_dir_path)
                .distinct()
                .filter(parent_dir_path.is_null())
                .load::<String>(conn)
                .expect("Query image_paths failed"),
        }
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<(), String> {
        diesel::insert_into(image_paths)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}
