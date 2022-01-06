use crate::FotoboekDatabase;
use std::collections::BTreeMap;
use chrono::{Datelike, Local};
use crate::diesel::RunQueryDsl;
use diesel::sql_types::{Integer, Text};

pub type FlashbackDates = BTreeMap<String, Vec<i32>>;

pub async fn dates(db: FotoboekDatabase) -> FlashbackDates {
    #[derive(QueryableByName)]
    struct ImageDate {
        #[sql_type = "Text"]
        date: String,
        #[sql_type = "Integer"]
        file_id: i32,
    }

    db.run(move |conn| {
        let date = Local::today().naive_local();
        let day = date.day() as i32;
        let month = date.month() as i32;

        let sql = r#"
           SELECT DATE(effective_date) as date, file_id
           FROM file_metadata
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
            entry.push(it.file_id);
            return map;
        })
    }).await
}
