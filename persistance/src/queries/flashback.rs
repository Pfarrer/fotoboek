use crate::FotoboekDatabase;
use std::collections::BTreeMap;
use serde::Serialize;
use chrono::{Datelike, Local};
use crate::diesel::RunQueryDsl;
use diesel::sql_types::{Integer, Text};

#[derive(Serialize)]
pub struct FlashbackFileInfo {
    id: i32,
    r#type: String,
}
pub type FlashbackDates = BTreeMap<String, Vec<FlashbackFileInfo>>;

pub async fn dates(db: FotoboekDatabase) -> FlashbackDates {
    #[derive(QueryableByName)]
    struct ImageDate {
        #[sql_type = "Text"]
        date: String,
        #[sql_type = "Integer"]
        file_id: i32,
        #[sql_type = "Text"]
        file_type: String,
    }

    db.run(move |conn| {
        let date = Local::today().naive_local();
        let day = date.day() as i32;
        let month = date.month() as i32;

        let sql = r#"
            SELECT
                DATE(file_metadata.effective_date) as date,
                files.id AS file_id,
                files.file_type AS file_type
            FROM files
            INNER JOIN file_metadata
                ON files.id = file_metadata.file_id
            WHERE STRFTIME('%m-%d', effective_date) = ?
        "#;

        let image_dates: Vec<ImageDate> = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Text, _>(format!("{:0>2}-{:0>2}", month, day))
            .load(conn)
            .expect("Query flashback.dates failed");

        image_dates.iter().fold(BTreeMap::new(), |mut map, it| {
            let entry = map
                .entry(it.date.clone())
                .or_insert(Vec::new());
            entry.push(FlashbackFileInfo {
                id: it.file_id,
                r#type: it.file_type.clone(),
            });
            return map;
        })
    }).await
}
