use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::Serialize;

use crate::schema::file_metadata;
use crate::schema::file_metadata::dsl;
use crate::FotoboekDatabase;

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
    pub filename_date: Option<NaiveDateTime>
}

impl FileMetadata {
    pub async fn by_file_id(db: &FotoboekDatabase, file_id: i32) -> Option<FileMetadata> {
        db.run(move |conn| {
            dsl::file_metadata
                .filter(dsl::file_id.eq(file_id))
                .first::<FileMetadata>(conn)
                .ok()
        })
        .await
    }

    pub async fn save(self, db: &FotoboekDatabase) -> Result<(), String> {
        db.run(move |conn| {
            conn.immediate_transaction(|| {
                diesel::replace_into(dsl::file_metadata)
                    .values(&self)
                    .execute(conn)?;
                Ok(())
            })
            .map_err(|err: diesel::result::Error| err.to_string())
        })
        .await
    }
}
