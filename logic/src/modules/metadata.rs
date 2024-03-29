use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::str::FromStr;

use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use log::warn;
use mp4::{Mp4Reader, TrackType};
use opencv::imgcodecs;
use opencv::prelude::MatTraitManual;
use rexif::{ExifData, ExifTag};
use sha256::digest_bytes;
use regex::{Captures, Regex};

use persistance::models::{File, FileMetadata, Task};
use persistance::FotoboekDatabase;
use shared::models::FotoboekConfig;
use shared::path_utils;
use shared::path_utils::rel_to_abs;

pub const MODULE_ID: &str = "metadata";

pub async fn create_tasks_on_new_file(db: &FotoboekDatabase, file: &File) -> Result<(), String> {
    Task {
        id: None,
        file_id: file.id.unwrap(),
        module: MODULE_ID.into(),
        priority: 100,
        max_worker_id: 1024,
        work_started_at: chrono::NaiveDateTime::from_timestamp(0, 0),
    }
    .insert(db)
    .await?;

    Ok(())
}

trait MetadataExtractor {
    fn resolution(&self) -> (i32, i32);
    fn creation_date(&self) -> Option<NaiveDateTime>;
    fn filename_date(&self) -> Option<NaiveDateTime>;
    fn camera_manufacturer(&self) -> Option<String> {
        None
    }
    fn camera_model(&self) -> Option<String> {
        None
    }
    fn exif_aperture(&self) -> Option<String> {
        None
    }
    fn exif_exposure_time(&self) -> Option<String> {
        None
    }
    fn exif_iso(&self) -> Option<String> {
        None
    }
    fn video_duration(&self) -> Option<i32> {
        None
    }
    fn gps_lat_lon(&self) -> Option<(f32, f32)> {
        None
    }
}

struct ImageMetadataExtractor {
    abs_path: String,
    exif_opt: Option<ExifData>,
}

impl ImageMetadataExtractor {
    fn parse(abs_path: String, file_contents: &[u8]) -> Box<dyn MetadataExtractor> {
        let exif_opt = rexif::parse_buffer_quiet(&file_contents).0.ok();
        Box::new(ImageMetadataExtractor { abs_path, exif_opt })
    }

    fn get_exif_value(&self, tag: ExifTag) -> Option<String> {
        self.exif_opt
            .as_ref()
            .map(|exif| {
                exif.entries
                    .iter()
                    .filter(|entry| entry.tag == tag)
                    .map(|entry| entry.value_more_readable.clone().into_owned())
                    .next()
            })
            .flatten()
    }
}

impl MetadataExtractor for ImageMetadataExtractor {
    fn resolution(&self) -> (i32, i32) {
        let img = imgcodecs::imread(&self.abs_path, imgcodecs::IMREAD_GRAYSCALE)
            .expect("Image not found or invalid");
        let size = img.size().expect("Failed to get image size");

        (size.width, size.height)
    }

    fn creation_date(&self) -> Option<NaiveDateTime> {
        if let Some(value) = self.get_exif_value(ExifTag::DateTime) {
            NaiveDateTime::parse_from_str(&value, &"%Y:%m:%d %H:%M:%S").ok()
        } else {
            None
        }
    }

    fn filename_date(&self) -> Option<NaiveDateTime> {
        let path_buf = PathBuf::from(&self.abs_path);
        let filename = path_utils::get_filename(&path_buf);
        search_for_date_time_in_filename(filename)
    }

    fn camera_manufacturer(&self) -> Option<String> {
        self.get_exif_value(ExifTag::Make)
    }
    fn camera_model(&self) -> Option<String> {
        self.get_exif_value(ExifTag::Model)
    }
    fn exif_aperture(&self) -> Option<String> {
        self.get_exif_value(ExifTag::ApertureValue)
    }
    fn exif_exposure_time(&self) -> Option<String> {
        self.get_exif_value(ExifTag::ExposureTime)
    }
    fn exif_iso(&self) -> Option<String> {
        self.get_exif_value(ExifTag::ISOSpeedRatings)
    }
    fn gps_lat_lon(&self) -> Option<(f32, f32)> {
        let lat_option = self
            .get_exif_value(ExifTag::GPSLatitude)
            .map(|s| latlon::parse_lng(s).ok())
            .flatten();
        let lon_option = self
            .get_exif_value(ExifTag::GPSLongitude)
            .map(|s| latlon::parse_lng(s).ok())
            .flatten();

        if let (Some(lat), Some(lon)) = (lat_option, lon_option) {
            Some((lat as f32, lon as f32))
        } else {
            None
        }
    }
}

struct VideoMetadataExtractor {
    abs_path: String,
    mp4: Option<Mp4Reader<Cursor<Vec<u8>>>>,
}

impl VideoMetadataExtractor {
    fn parse(abs_path: String, file_contents: Vec<u8>) -> Box<(dyn MetadataExtractor)> {
        let size = file_contents.len() as u64;
        let cursor = Cursor::new(file_contents);
        let mp4 = Mp4Reader::read_header(cursor, size).ok();
        Box::new(VideoMetadataExtractor { abs_path, mp4 })
    }
}

impl MetadataExtractor for VideoMetadataExtractor {
    fn resolution(&self) -> (i32, i32) {
        let opt = self
            .mp4
            .as_ref()
            .map(|mp4| {
                mp4.tracks()
                    .values()
                    .filter(|track| {
                        track.track_type().is_ok()
                            && track.track_type().unwrap() == TrackType::Video
                    })
                    .map(|track| (track.width() as i32, track.height() as i32))
                    .collect::<Vec<_>>()
                    .first()
                    .copied()
            })
            .flatten();

        opt.unwrap_or_else(|| {
            warn!(
                "Could not extract resolution from video file, will use (0,0): {}",
                self.abs_path
            );
            (0, 0)
        })
    }

    fn filename_date(&self) -> Option<NaiveDateTime> {
        let path_buf = PathBuf::from(&self.abs_path);
        let filename = path_utils::get_filename(&path_buf);
        search_for_date_time_in_filename(filename)
    }

    fn creation_date(&self) -> Option<NaiveDateTime> {
        self.mp4.as_ref().map(|mp4| {
            let mut creation_time = mp4.moov.mvhd.creation_time;

            // convert from MP4 epoch (1904-01-01) to Unix epoch (1970-01-01)
            creation_time = if creation_time >= 2082844800 {
                creation_time - 2082844800
            } else {
                creation_time
            };

            if creation_time == 0 {
                None
            } else {
                Some(NaiveDateTime::from_timestamp(creation_time as i64, 0))
            }
        }).flatten()
    }

    fn video_duration(&self) -> Option<i32> {
        self.mp4.as_ref().map(|mp4| mp4.duration().as_secs() as i32)
    }
}

pub async fn run_task(
    db: &FotoboekDatabase,
    config: &FotoboekConfig,
    task: &Task,
) -> Result<(), String> {
    let file = File::by_id(db, task.file_id).await?;
    let abs_path = rel_to_abs(config, &file.rel_path);

    let (file_size_bytes, file_date) = get_file_size_and_date(&abs_path)?;
    let file_contents = read_file_contents(&abs_path, file_size_bytes as usize);
    let file_hash = digest_bytes(&file_contents);

    let metadata = {
        let metadata_extractor = match file.file_type.as_str() {
            "IMAGE" => ImageMetadataExtractor::parse(abs_path, &file_contents),
            "VIDEO" => VideoMetadataExtractor::parse(abs_path, file_contents),
            _ => panic!("Unsupported file type: {}", file.file_type),
        };

        let (resolution_x, resolution_y) = metadata_extractor.resolution();
        let creation_date = metadata_extractor.creation_date();
        let filename_date = metadata_extractor.filename_date();
        let exif_gps_lat_lon = metadata_extractor.gps_lat_lon();

        FileMetadata {
            file_id: Some(task.file_id),
            file_hash,
            file_size_bytes,
            file_date,
            resolution_x,
            resolution_y,
            exif_date: creation_date,
            exif_camera_manufacturer: metadata_extractor.camera_manufacturer(),
            exif_camera_model: metadata_extractor.camera_model(),
            exif_aperture: metadata_extractor.exif_aperture(),
            exif_exposure_time: metadata_extractor.exif_exposure_time(),
            exif_iso: metadata_extractor.exif_iso(),
            exif_gps_lat: exif_gps_lat_lon.map(|lat_lon| lat_lon.0),
            exif_gps_lon: exif_gps_lat_lon.map(|lat_lon| lat_lon.1),
            effective_date: creation_date.or(filename_date).unwrap_or(file_date),
            filename_date,
        }
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

fn search_for_date_time_in_filename(filename: String) -> Option<NaiveDateTime> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<year>\d{4})(?P<month>\d{2})(?P<day>\d{2})([^\d]*)((?P<hour>\d{2})(?P<minute>\d{2})(?P<second>\d{2}))?"
        ).unwrap();
    }

    RE.captures(filename.as_str()).map(|cap: Captures| {
        let year: i32 = cap.name("year").unwrap().as_str().parse().unwrap();
        let month: u32 = cap.name("month").unwrap().as_str().parse().unwrap();
        let day: u32 = cap.name("day").unwrap().as_str().parse().unwrap();

        let hour: u32 = cap.name("hour").map(|str| str.as_str().parse().unwrap()).unwrap_or(0);
        let minute: u32 = cap.name("minute").map(|str| str.as_str().parse().unwrap()).unwrap_or(0);
        let second: u32 = cap.name("second").map(|str| str.as_str().parse().unwrap()).unwrap_or(0);

        ymd_hms_to_native_datetime(year, month, day, hour, minute, second)
    }).flatten()
}

fn ymd_hms_to_native_datetime(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Option<NaiveDateTime> {
    NaiveDateTime::from_str(&format!("{year}-{month}-{day}T{hour}:{minute}:{second}")).ok()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use chrono::NaiveDateTime;

    #[test]
    fn test_search_for_date_time_in_filename() {
        use crate::modules::metadata::search_for_date_time_in_filename as test_method;

        assert_eq!(None, test_method("".to_string()));
        assert_eq!(None, test_method("VID123.mp4".to_string()));

        // Successful cases
        assert_eq!(
            Some(NaiveDateTime::from_str("2020-05-15T00:00:00").unwrap()),
            test_method("VID-20200515-WA0002.mp4".to_string())
        );
        assert_eq!(
            Some(NaiveDateTime::from_str("1990-12-31T13:14:15").unwrap()),
            test_method("IMG_19901231131415.jpg".to_string())
        );
        assert_eq!(
            Some(NaiveDateTime::from_str("1990-12-31T13:14:15").unwrap()),
            test_method("IMG_19901231_131415Z.jpg".to_string())
        );

        // Error cases
        assert_eq!(None, test_method("VID-20201315-WA0002.mp4".to_string()));
        assert_eq!(None, test_method("VID-20200015-WA0002.mp4".to_string()));
        assert_eq!(None, test_method("VID-20200942-WA0002.mp4".to_string()));
        assert_eq!(None, test_method("VID-20200900-WA0002.mp4".to_string()));
        assert_eq!(None, test_method("VID-20200901-241415.jpg".to_string()));
        assert_eq!(None, test_method("VID-20200901-236015.jpg".to_string()));
        assert_eq!(None, test_method("VID-20200901-235961.jpg".to_string()));
    }
}
