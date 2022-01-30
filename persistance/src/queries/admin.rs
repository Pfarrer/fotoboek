use chrono::NaiveDate;
use diesel::sql_types::{Bigint, Date, Integer, Text};
use serde::Serialize;
use std::collections::btree_map::BTreeMap;

use crate::diesel::RunQueryDsl;
use crate::FotoboekDatabase;

#[derive(Serialize)]
pub struct MediaDateInfo {
    pub images_count: i32,
    pub images_size_bytes: i64,
    pub videos_count: i32,
    pub videos_size_bytes: i64,
}

pub type MediaDateMap = BTreeMap<String, MediaDateInfo>;

/// Returns a map that contains information about how many images and videos were taken at any day.
pub async fn get_media_date_map(db: &FotoboekDatabase) -> MediaDateMap {
    let media_dates: Vec<MediaDate> = db
        .run(move |conn| {
            let sql = r#"
            SELECT
                DATE(file_metadata.effective_date) as date,
                files.file_type AS file_type,
                COUNT(files.id) AS files_count,
                SUM(file_metadata.file_size_bytes) AS files_size_bytes
            FROM files
            INNER JOIN file_metadata
                ON files.id = file_metadata.file_id
            GROUP BY DATE(file_metadata.effective_date), files.file_type
        "#;

            diesel::sql_query(sql)
                .load(conn)
                .expect("Query get_index_info failed")
        })
        .await;

    map_to_media_date_map(media_dates)
}

fn map_to_media_date_map(media_dates: Vec<MediaDate>) -> MediaDateMap {
    media_dates.iter().fold(BTreeMap::new(), |mut map, it| {
        let media_date_info = map.entry(it.date.to_string()).or_insert(MediaDateInfo {
            images_count: 0,
            images_size_bytes: 0,
            videos_count: 0,
            videos_size_bytes: 0,
        });
        match it.file_type.as_ref() {
            "IMAGE" => {
                media_date_info.images_count += it.files_count;
                media_date_info.images_size_bytes += it.files_size_bytes;
            }
            "VIDEO" => {
                media_date_info.videos_count += it.files_count;
                media_date_info.videos_size_bytes += it.files_size_bytes;
            }
            _ => panic!("Unsupported file_type: {}", it.file_type),
        };
        return map;
    })
}

#[derive(QueryableByName, Debug)]
struct MediaDate {
    #[sql_type = "Date"]
    pub date: NaiveDate,
    #[sql_type = "Text"]
    pub file_type: String,
    #[sql_type = "Integer"]
    pub files_count: i32,
    #[sql_type = "Bigint"]
    pub files_size_bytes: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_database() {
        assert_eq!(0, map_to_media_date_map(vec![]).len());
    }

    #[test]
    fn single_date_with_images_only() {
        let db_date_media_info = MediaDate {
            date: NaiveDate::from_ymd(2022, 1, 30),
            file_type: "IMAGE".to_string(),
            files_count: 5,
            files_size_bytes: 100,
        };

        let date_media_map = map_to_media_date_map(vec![db_date_media_info]);
        assert_eq!(1, date_media_map.len());

        let date_media = date_media_map.get("2022-01-30").unwrap();
        assert_eq!(5, date_media.images_count);
        assert_eq!(100, date_media.images_size_bytes);
        assert_eq!(0, date_media.videos_count);
        assert_eq!(0, date_media.videos_size_bytes);
    }

    #[test]
    fn single_date_with_videos_only() {
        let db_date_media_info = MediaDate {
            date: NaiveDate::from_ymd(2022, 1, 30),
            file_type: "VIDEO".to_string(),
            files_count: 2,
            files_size_bytes: 30000,
        };

        let date_media_map = map_to_media_date_map(vec![db_date_media_info]);
        assert_eq!(1, date_media_map.len());

        let date_media = date_media_map.get("2022-01-30").unwrap();
        assert_eq!(0, date_media.images_count);
        assert_eq!(0, date_media.images_size_bytes);
        assert_eq!(2, date_media.videos_count);
        assert_eq!(30000, date_media.videos_size_bytes);
    }

    #[test]
    fn multiple_dates_mixed_media() {
        let db_image_info1 = MediaDate {
            date: NaiveDate::from_ymd(2022, 2, 2),
            file_type: "IMAGE".to_string(),
            files_count: 2,
            files_size_bytes: 220,
        };

        let db_image_info2 = MediaDate {
            date: NaiveDate::from_ymd(2022, 1, 30),
            file_type: "IMAGE".to_string(),
            files_count: 5,
            files_size_bytes: 100,
        };
        let db_video_info = MediaDate {
            date: NaiveDate::from_ymd(2022, 1, 30),
            file_type: "VIDEO".to_string(),
            files_count: 2,
            files_size_bytes: 30000,
        };

        let date_media_map =
            map_to_media_date_map(vec![db_image_info1, db_image_info2, db_video_info]);
        assert_eq!(2, date_media_map.len());

        let date_media = date_media_map.get("2022-01-30").unwrap();
        assert_eq!(5, date_media.images_count);
        assert_eq!(100, date_media.images_size_bytes);
        assert_eq!(2, date_media.videos_count);
        assert_eq!(30000, date_media.videos_size_bytes);

        let date_media = date_media_map.get("2022-02-02").unwrap();
        assert_eq!(2, date_media.images_count);
        assert_eq!(220, date_media.images_size_bytes);
        assert_eq!(0, date_media.videos_count);
        assert_eq!(0, date_media.videos_size_bytes);
    }
}
