use chrono::NaiveDateTime;
use diesel::sql_types::{Integer, Text, Timestamp};
use crate::diesel::RunQueryDsl;
use crate::FotoboekDatabase;

#[derive(QueryableByName)]
pub struct GalleryFileInfo {
    #[sql_type = "Integer"]
    pub file_id: i32,
    #[sql_type = "Text"]
    pub rel_path: String,
    #[sql_type = "Text"]
    pub file_type: String,
    #[sql_type = "Timestamp"]
    pub effective_date: NaiveDateTime,
}

pub async fn get_gallery_file_infos(db: &FotoboekDatabase) -> Vec<GalleryFileInfo> {
    db.run(move |conn| {
        let sql = r#"
           SELECT
               files.id AS file_id,
               files.rel_path AS rel_path,
               files.file_type AS file_type,
               file_metadata.effective_date AS effective_date
           FROM files
           INNER JOIN file_metadata
               ON files.id = file_metadata.file_id
           ORDER BY file_metadata.effective_date
       "#;

        diesel::sql_query(sql)
            .load(conn)
            .expect("Query get_gallery_file_infos failed")
    }).await
}
