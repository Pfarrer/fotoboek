use crate::FotoboekDatabase;
use std::collections::BTreeMap;
use crate::diesel::RunQueryDsl;
use diesel::sql_types::{Integer, Text};

pub type TimelineDates = BTreeMap<String, Vec<i32>>;

pub async fn dates(db: FotoboekDatabase) -> TimelineDates {
    #[derive(QueryableByName)]
    struct ImageDate {
        #[sql_type = "Text"]
        date: String,
        #[sql_type = "Integer"]
        file_id: i32,
    }

    db.run(move |conn| {
        let sql = r#"
           SELECT DATE(effective_date) as date, file_id
           FROM file_metadata
       "#;

        let image_dates: Vec<ImageDate> = diesel::sql_query(sql)
            .load(conn)
            .expect("Query timeline.dates failed");

        image_dates.iter().fold(BTreeMap::new(), |mut map, it| {
            let entry = map
                .entry(it.date.clone())
                .or_insert(Vec::new());
            entry.push(it.file_id);
            return map;
        })
    }).await
}
