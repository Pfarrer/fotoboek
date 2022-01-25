use crate::FotoboekDatabase;
use std::collections::BTreeMap;
use serde::Serialize;
use crate::diesel::RunQueryDsl;
use diesel::sql_types::{Integer, Text};

#[derive(Serialize)]
pub struct TimelineFileInfo {
    id: i32,
    r#type: String,
}
pub type TimelineDates = BTreeMap<String, Vec<TimelineFileInfo>>;

pub async fn dates(db: FotoboekDatabase) -> TimelineDates {
    #[derive(QueryableByName)]
    struct DateAndFileInfo {
        #[sql_type = "Text"]
        date: String,
        #[sql_type = "Integer"]
        file_id: i32,
        #[sql_type = "Text"]
        file_type: String,
    }

    db.run(move |conn| {
        let sql = r#"
            SELECT
                DATE(file_metadata.effective_date) as date,
                files.id AS file_id,
                files.file_type AS file_type
            FROM files
            INNER JOIN file_metadata
                ON files.id = file_metadata.file_id
       "#;

        let image_dates: Vec<DateAndFileInfo> = diesel::sql_query(sql)
            .load(conn)
            .expect("Query timeline.dates failed");

        image_dates.iter().fold(BTreeMap::new(), |mut map, it| {
            let entry = map
                .entry(it.date.clone())
                .or_insert(Vec::new());
            entry.push(TimelineFileInfo {
                id: it.file_id,
                r#type: it.file_type.clone()
            });
            return map;
        })
    }).await
}
