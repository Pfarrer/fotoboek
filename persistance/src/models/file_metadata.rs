use diesel::{self, prelude::*};
use serde::Serialize;
use chrono::NaiveDateTime;

use crate::FotoboekDatabase;
use crate::schema::file_metadata;
use crate::schema::file_metadata::dsl;

#[derive(Insertable, Queryable, QueryableByName, Serialize, Debug)]
#[table_name = "file_metadata"]
pub struct FileMetadata {
    pub file_id: Option<i32>,
    pub file_size_bytes: i32,
    pub file_hash: String,
    pub file_date: NaiveDateTime,
    pub resolution_x: i32,
    pub resolution_y: i32,
    pub exif_date: Option<NaiveDateTime>,
    pub exif_aperture: Option<String>,
    pub exif_exposure_time: Option<String>,
    pub exif_iso: Option<String>,
    pub exif_camera_manufacturer: Option<String>,
    pub exif_camera_model: Option<String>,
    pub exif_gps_lat: Option<f32>,
    pub exif_gps_lon: Option<f32>,
    pub effective_date: NaiveDateTime,
}

impl FileMetadata {
    pub async fn by_file_id(db: &FotoboekDatabase, file_id: i32) -> Option<FileMetadata> {
        db.run(move |conn| {
            dsl::file_metadata
                .filter(dsl::file_id.eq(file_id))
                .first::<FileMetadata>(conn)
                .ok()
        }).await
    }

    // pub fn all(conn: &diesel::SqliteConnection) -> Vec<Metadata> {
    //     metadata.load::<Metadata>(conn).expect("Query all metadata")
    // }
    // pub fn by_image_id(conn: &diesel::SqliteConnection, by_image_id: i32) -> Option<Metadata> {
    //     metadata
    //         .filter(image_id.eq(by_image_id))
    //         .first::<Metadata>(conn)
    //         .ok()
    // }
    // pub fn by_image_path_and_ordered(
    //     conn: &diesel::SqliteConnection,
    //     by_abs_dir_path: &str,
    //     max_distance: i32,
    // ) -> Vec<Metadata> {
    //     diesel::sql_query(
    //         r#"
    //             SELECT m.*
    //             FROM metadata m
    //             INNER JOIN image_paths p
    //                 ON p.image_id = m.image_id
    //             WHERE p.abs_dir_path = ?
    //                 AND distance <= ?
    //             ORDER BY COALESCE(m.exif_date, m.file_date)
    //         "#,
    //     )
    //     .bind::<diesel::sql_types::Text, _>(by_abs_dir_path)
    //     .bind::<diesel::sql_types::Integer, _>(max_distance)
    //     .load(conn)
    //     .expect("Query by_image_path_and_ordered")
    // }
    //
    // pub fn by_day_and_month(
    //     conn: &diesel::SqliteConnection,
    //     day: i32,
    //     month: i32,
    // ) -> Vec<Metadata> {
    //     diesel::sql_query(
    //         r#"
    //             SELECT *
    //             FROM metadata
    //             WHERE strftime('%m-%d', COALESCE(exif_date, file_date)) = ?
    //         "#,
    //     )
    //     .bind::<diesel::sql_types::Text, _>(format!("{:0>2}-{:0>2}", month, day))
    //     .load(conn)
    //     .expect("Query by_image_path_and_ordered")
    // }

    pub async fn by_start_date(db: &FotoboekDatabase, start_date: NaiveDateTime, direction: isize, limit: usize) -> Vec<FileMetadata> {
        db.run(move |conn| {
            let date_operator = match direction {
                d if d > 0 => ">",
                d if d < 0 => "<",
                _ => panic!("Invalid direction parameter"),
            };
            let sql = format!(
                r#"
                   SELECT *
                   FROM file_metadata
                   WHERE DATETIME(effective_date) {} DATETIME(?)
                   ORDER BY effective_date
                   LIMIT ?
               "#, date_operator
            );

            diesel::sql_query(sql)
                .bind::<diesel::sql_types::Text, _>(format!("{}", start_date))
                .bind::<diesel::sql_types::Integer, _>(limit as i32)
                .load(conn)
                .expect("Query by_start_date failed")
        }).await
    }

    pub async fn save(self, db: &FotoboekDatabase) -> Result<(), String> {
        db.run(move |conn|
            conn.immediate_transaction(|| {
                diesel::replace_into(dsl::file_metadata).values(&self).execute(conn)?;
                Ok(())
            })
                .map_err(|err: diesel::result::Error| err.to_string())
        ).await
    }
}
