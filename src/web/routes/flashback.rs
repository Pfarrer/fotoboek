use crate::core::utils::abs_to_rel_path;
use crate::db::models::*;
use crate::db::Database;
use crate::web::routes::gallery::DisplaySettings;
use chrono::{Datelike, Duration, Local, NaiveDate};
use itertools::Itertools;
use rocket::futures::stream::{self, StreamExt};
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct FlashbackContext<'a> {
    flashbacks: Vec<Flashback<'a>>,
}

#[derive(Serialize)]
struct Flashback<'a> {
    date: (u32, u32, i32),
    title: String,
    image_metadata: Vec<&'a Metadata>,
    gallery_image_urls: HashMap<i32, String>,
}

#[get("/flashback")]
pub async fn flashback(db: Database) -> Template {
    let last_date_and_flashback_images = find_last_flashback_images(&db).await;
    debug!(
        "Latest flashback found on {:?}",
        last_date_and_flashback_images
            .as_ref()
            .map(|(date, _)| date)
    );

    let flashbacks = if let Some((ref date, ref image_metadata)) = last_date_and_flashback_images {
        map_image_metadata_to_flashbacks(&db, date, image_metadata).await
    } else {
        vec![]
    };

    let context = FlashbackContext { flashbacks };
    Template::render("flashback", context)
}

async fn find_last_flashback_images(db: &Database) -> Option<(NaiveDate, Vec<Metadata>)> {
    let today = Local::today().naive_local();

    for offset in 0..30 {
        let date = today.checked_add_signed(Duration::days(-offset)).unwrap();
        let day = date.day() as i32;
        let month = date.month() as i32;

        let metadata: Vec<Metadata> = db
            .run(move |conn| Metadata::by_day_and_month(conn, day, month))
            .await
            .into_iter()
            .filter(|m| m.effective_date().year() < today.year())
            .collect();

        if metadata.len() > 0 {
            return Some((date, metadata));
        }
    }

    None
}

async fn map_image_metadata_to_flashbacks<'a>(
    db: &Database,
    date: &NaiveDate,
    image_metadata: &'a Vec<Metadata>,
) -> Vec<Flashback<'a>> {
    let year_and_metadata_map = image_metadata
        .into_iter()
        .into_group_map_by(|metadata| metadata.effective_date().year());

    stream::iter(year_and_metadata_map)
        .then(|(year, metadata)| async move {
            let title = get_flashback_title(&date.with_year(year).unwrap());

            let image_ids = metadata.iter().map(|m| m.image_id).collect();
            let image_urls = make_image_urls(db, image_ids).await;

            Flashback {
                date: (date.day(), date.month(), year),
                title,
                image_metadata: metadata.to_vec(),
                gallery_image_urls: image_urls,
            }
        })
        .collect()
        .await
}

fn get_flashback_title(date: &NaiveDate) -> String {
    let diff_years = get_diff_in_years_since(date);

    match diff_years {
        1 => "Last year".to_owned(),
        x => format!("{} years ago", x),
    }
}

fn get_diff_in_years_since(date: &NaiveDate) -> u32 {
    let today = Local::today().naive_local();

    (today.year() - date.year()) as u32
}

async fn make_image_urls<'a>(db: &Database, image_ids: Vec<i32>) -> HashMap<i32, String> {
    db.run(move |conn| {
        image_ids
            .iter()
            .map(|image_id| {
                let image_path = ImagePath::by_image_id(conn, *image_id);
                let settings = DisplaySettings {
                    path: Some(abs_to_rel_path(&image_path.abs_dir_path)),
                    deep: None,
                };

                (
                    *image_id,
                    rocket::uri!(crate::web::routes::gallery::image_by_id(image_id, settings))
                        .to_string(),
                )
            })
            .into_iter()
            .collect()
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diff_in_years_since() {
        let today = Local::today().naive_local();

        assert_eq!(0, get_diff_in_years_since(&today));

        assert_eq!(
            2,
            get_diff_in_years_since(&NaiveDate::from_ymd(
                today.year() - 2,
                today.month(),
                today.day()
            ))
        );

        assert_eq!(
            19,
            get_diff_in_years_since(&NaiveDate::from_ymd(
                today.year() - 19,
                today.month(),
                today.day()
            ))
        );
    }
}
