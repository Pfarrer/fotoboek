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

    pub fn subdirs_of(conn: &diesel::SqliteConnection, by_parent_dir_path: &str) -> Vec<String> {
        image_paths
            .select(abs_dir_path)
            .distinct()
            .filter(parent_dir_path.eq(by_parent_dir_path))
            .load::<String>(conn)
            .expect("Query image_paths failed")
    }

    pub fn by_image_id(conn: &diesel::SqliteConnection, by_image_id: i32) -> ImagePath {
        image_paths
            .filter(image_id.eq(by_image_id).and(distance.eq(0)))
            .first::<ImagePath>(conn)
            .expect("Query image_paths failed")
    }

    pub fn save(self, conn: &diesel::SqliteConnection) -> Result<(), String> {
        conn.immediate_transaction(|| {
            diesel::replace_into(image_paths)
                .values(&self)
                .execute(conn)?;
            Ok(())
        })
        .map_err(|err: diesel::result::Error| err.to_string())
    }
}
