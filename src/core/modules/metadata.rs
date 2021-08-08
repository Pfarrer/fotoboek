use chrono::NaiveDateTime;
use opencv::imgcodecs;
use opencv::prelude::MatTraitManual;
use rexif::{ExifData, ExifTag};

use crate::db::models::Image;
use crate::db::models::Metadata;
use crate::db::models::Task;
use crate::db::Database;

pub const MODULE_ID: &str = "metadata";

pub async fn create_tasks_on_new_image(db: &Database, image: &Image) -> Result<(), String> {
    let image_id = image.id.expect("Image must have an id");

    db.run(move |c| {
        Task::new(image_id, MODULE_ID.into(), 100)
            .insert(c)
    })
    .await?;

    Ok(())
}

pub fn run_task(conn: &diesel::SqliteConnection, task: &Task) -> Result<(), String> {
    let image = Image::by_id(conn, task.image_id)?;

    let (file_size_bytes, file_date) = get_file_size_and_date(&image.abs_path)?;

    let exif = rexif::parse_file(&image.abs_path).map_err(|err| err.to_string())?;
    let exif_date = get_exif_date(&exif)?;
    let exif_gps_lat_lon = get_exif_gps_lat_lon(&exif);

    let (resolution_x, resolution_y) = get_image_resolution(&image.abs_path)?;

    Metadata {
        image_id: task.image_id,
        file_size_bytes,
        file_date,
        resolution_x,
        resolution_y,
        exif_date,
        exif_camera_manufacturer: get_exif_value(&exif, ExifTag::Make),
        exif_camera_model: get_exif_value(&exif, ExifTag::Model),
        exif_aperture: get_exif_value(&exif, ExifTag::ApertureValue),
        exif_exposure_time: get_exif_value(&exif, ExifTag::ExposureTime),
        exif_iso: get_exif_value(&exif, ExifTag::ISOSpeedRatings),
        exif_gps_lat: exif_gps_lat_lon.map(|lat_lon| lat_lon.0),
        exif_gps_lon: exif_gps_lat_lon.map(|lat_lon| lat_lon.1),
    }
    .insert(conn)?;

    Ok(())
}

fn get_file_size_and_date(abs_path: &String) -> Result<(i32, NaiveDateTime), String> {
    let metadata =
        std::fs::metadata(abs_path).map_err(|err| format!("Failed to get fs metadata: {}", err))?;

    let file_size = metadata.len() as i32;
    let file_date_time: chrono::DateTime<chrono::Utc> = metadata
        .created()
        .map_err(|err| format!("Failed to get created date: {:?}", err))?
        .into();

    Ok((file_size, file_date_time.naive_utc()))
}

fn get_image_resolution(abs_path: &String) -> Result<(i32, i32), String> {
    let img =
        imgcodecs::imread(abs_path, imgcodecs::IMREAD_COLOR).expect("Image not found or invalid");
    let size = img.size().expect("Failed to get image size");
    Ok((size.width, size.height))
}

fn get_exif_value(exif: &ExifData, tag: ExifTag) -> Option<String> {
    exif.entries
        .iter()
        .filter(|entry| entry.tag == tag)
        .map(|entry| entry.value_more_readable.clone().into_owned())
        .next()
}

fn get_exif_date(exif: &ExifData) -> Result<Option<NaiveDateTime>, String> {
    if let Some(value) = get_exif_value(&exif, ExifTag::DateTime) {
        let date = NaiveDateTime::parse_from_str(&value, &"%Y:%m:%d %H:%M:%S")
            .map_err(|err| format!("Failed to parse EXIF date: {:?}", err))?;
        return Ok(Some(date));
    }

    Ok(None)
}

fn get_exif_gps_lat_lon(exif: &ExifData) -> Option<(f32, f32)> {
    let lat_option = get_exif_value(&exif, ExifTag::GPSLatitude)
        .map(|s| latlon::parse_lng(s).ok()).flatten();
    let lon_option = get_exif_value(&exif, ExifTag::GPSLongitude)
        .map(|s| latlon::parse_lng(s).ok()).flatten();

    if let (Some(lat), Some(lon)) = (lat_option, lon_option) {
        Some((lat as f32, lon as f32))
    } else {
        None
    }
}
