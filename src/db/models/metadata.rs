use diesel::{self, prelude::*};
use serde::Serialize;

use crate::db::schema::metadata;
use crate::db::schema::metadata::dsl::*;

#[derive(Insertable, Queryable, QueryableByName, Serialize, Debug)]
#[table_name = "metadata"]
pub struct Metadata {
    pub image_id: i32,
    pub file_size_bytes: i32,
    pub file_date: chrono::NaiveDateTime,
    pub resolution_x: i32,
    pub resolution_y: i32,
    pub exif_date: Option<chrono::NaiveDateTime>,
    pub exif_aperture: Option<String>,
    pub exif_exposure_time: Option<String>,
    pub exif_iso: Option<String>,
    pub exif_camera_manufacturer: Option<String>,
    pub exif_camera_model: Option<String>,
    pub exif_gps_lat: Option<f32>,
    pub exif_gps_lon: Option<f32>,
}

impl Metadata {
    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Metadata> {
        metadata.load::<Metadata>(conn).expect("Query all metadata")
    }
    pub fn by_image_id(conn: &diesel::SqliteConnection, by_image_id: i32) -> Option<Metadata> {
        metadata
            .filter(image_id.eq(by_image_id))
            .first::<Metadata>(conn)
            .ok()
    }
    pub fn by_image_path_and_ordered(
        conn: &diesel::SqliteConnection,
        by_abs_dir_path: &str,
        max_distance: i32,
    ) -> Vec<Metadata> {
        diesel::sql_query(
            r#"
                SELECT m.*
                FROM metadata m
                INNER JOIN image_paths p
                    ON p.image_id = m.image_id
                WHERE p.abs_dir_path = ?
                    AND distance <= ?
                ORDER BY COALESCE(m.exif_date, m.file_date)
            "#,
        )
        .bind::<diesel::sql_types::Text, _>(by_abs_dir_path)
        .bind::<diesel::sql_types::Integer, _>(max_distance)
        .load(conn)
        .expect("Query by_image_path_and_ordered")
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<(), String> {
        diesel::insert_into(metadata)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}
