use std::io::Read;
use chrono::NaiveDateTime;
use opencv::imgcodecs;
use opencv::prelude::MatTraitManual;
use rexif::{ExifData, ExifTag};
use log::info;
use sha256::digest_bytes;

use persistance::FotoboekDatabase;
use persistance::models::{Task, File, FileMetadata};
use shared::path_utils::rel_to_abs;
use shared::models::FotoboekConfig;

pub const MODULE_ID: &str = "metadata";

pub async fn create_tasks_on_new_image(db: &FotoboekDatabase, file: &File) -> Result<(), String> {
    let file_id = file.id.unwrap();
    db.run(move |c|
        Task {
            id: None,
            file_id,
            module: MODULE_ID.into(),
            priority: 100,
            work_started_at: chrono::NaiveDateTime::from_timestamp(0, 0),
        }.insert(c)
    ).await?;

    Ok(())
}

pub async fn run_task(db: &FotoboekDatabase, config: &FotoboekConfig, task: &Task) -> Result<(), String> {
    let file = File::by_id(db, task.file_id).await?;
    let abs_path = rel_to_abs(config, &file.rel_path);

    let (file_size_bytes, file_date) = get_file_size_and_date(&abs_path)?;

    let file_contents = read_file_contents(&abs_path, file_size_bytes as usize);
    let file_hash = digest_bytes(&file_contents);
    let exif_opt = rexif::parse_buffer_quiet(&file_contents).0.ok();
    let (exif_date, exif_gps_lat_lon) = if let Some(ref exif) = &exif_opt {
        (
            get_exif_date(exif).ok().flatten(),
            get_exif_gps_lat_lon(exif),
        )
    } else {
        info!("No EXIF data found for image {}", file.rel_path);
        (None, None)
    };

    let (resolution_x, resolution_y) = get_image_resolution(&abs_path)?;

    let metadata = FileMetadata {
        file_id: Some(task.file_id),
        file_hash,
        file_size_bytes,
        file_date,
        resolution_x,
        resolution_y,
        exif_date,
        exif_camera_manufacturer: exif_opt
            .as_ref()
            .and_then(|exif| get_exif_value(&exif, ExifTag::Make)),
        exif_camera_model: exif_opt
            .as_ref()
            .and_then(|exif| get_exif_value(&exif, ExifTag::Model)),
        exif_aperture: exif_opt
            .as_ref()
            .and_then(|exif| get_exif_value(&exif, ExifTag::ApertureValue)),
        exif_exposure_time: exif_opt
            .as_ref()
            .and_then(|exif| get_exif_value(&exif, ExifTag::ExposureTime)),
        exif_iso: exif_opt
            .as_ref()
            .and_then(|exif| get_exif_value(&exif, ExifTag::ISOSpeedRatings)),
        exif_gps_lat: exif_gps_lat_lon.map(|lat_lon| lat_lon.0),
        exif_gps_lon: exif_gps_lat_lon.map(|lat_lon| lat_lon.1),
        effective_date: exif_date.unwrap_or(file_date),
    };
    metadata.save(db).await?;

    Ok(())
}

fn read_file_contents(abs_path: &String, file_size: usize) -> Vec<u8> {
    let mut contents: Vec<u8> = Vec::with_capacity(file_size);

    let mut file = std::fs::File::open(abs_path).unwrap();
    file.read_to_end(&mut contents).unwrap();

    contents
}

fn get_file_size_and_date(abs_path: &String) -> Result<(i32, NaiveDateTime), String> {
    let metadata = std::fs::metadata(abs_path)
        .map_err(|err| format!("Failed to get fs metadata: {}, abs_path: {}", err, abs_path))?;

    let file_size = metadata.len() as i32;
    let file_date_time: chrono::DateTime<chrono::Utc> = metadata
        .created()
        .map_err(|err| format!("Failed to get created date: {:?}", err))?
        .into();

    Ok((file_size, file_date_time.naive_utc()))
}

fn get_image_resolution(abs_path: &str) -> Result<(i32, i32), String> {
    let img = imgcodecs::imread(abs_path, imgcodecs::IMREAD_GRAYSCALE)
        .expect("Image not found or invalid");
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
        .map(|s| latlon::parse_lng(s).ok())
        .flatten();
    let lon_option = get_exif_value(&exif, ExifTag::GPSLongitude)
        .map(|s| latlon::parse_lng(s).ok())
        .flatten();

    if let (Some(lat), Some(lon)) = (lat_option, lon_option) {
        Some((lat as f32, lon as f32))
    } else {
        None
    }
}
