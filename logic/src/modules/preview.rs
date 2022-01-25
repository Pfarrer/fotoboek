use persistance::FotoboekDatabase;
use persistance::models::{File, FileMetadata, Task};
use shared::models::FotoboekConfig;
use shared::path_utils::rel_to_abs;

pub const MODULE_ID: &str = "preview";

pub async fn create_tasks_on_new_file(db: &FotoboekDatabase, file: &File) -> Result<(), String> {
    Task {
        id: None,
        file_id: file.id.unwrap(),
        module: MODULE_ID.into(),
        priority: 200,
        work_started_at: chrono::NaiveDateTime::from_timestamp(0, 0),
    }.insert(db).await?;

    Ok(())
}

pub async fn run_task(
    db: &FotoboekDatabase,
    config: &FotoboekConfig,
    task: &Task,
) -> Result<(), String> {
    let metadata = FileMetadata::by_file_id(db, task.file_id).await.ok_or(
        "File metadata not found, very likely because the metadata task did not finish yet"
            .to_string(),
    )?;
    let file = File::by_id(db, task.file_id).await?;
    let abs_path = rel_to_abs(config, &file.rel_path);

    match file.file_type.as_str() {
        "IMAGE" => image::run_task(config, &abs_path, &metadata.file_hash),
        "VIDEO" => video::run_task(config, &abs_path, &metadata.file_hash),
        _ => panic!("Unsupported file type: {}", file.file_type),
    }
}

mod image {
    use opencv::{core::Size, imgcodecs, imgproc, prelude::*};
    use persistance::fs;
    use shared::models::{FotoboekConfig, PreviewSize};

    const SIZE_ZERO: Size = Size {
        height: 0,
        width: 0,
    };

    pub fn run_task(config: &FotoboekConfig, abs_path: &String, file_hash: &String) -> Result<(), String> {
        let large_preview_bytes =
            resize_by_path(abs_path, &PreviewSize::Large).expect("Resize large failed");
        fs::store_preview(
            config,
            file_hash,
            &PreviewSize::Large,
            &large_preview_bytes,
        )?;

        let small_preview_bytes =
            resize_by_vec(large_preview_bytes, &PreviewSize::Small).expect("Resize small failed");
        fs::store_preview(
            config,
            file_hash,
            &PreviewSize::Small,
            &small_preview_bytes,
        )
    }

    fn resize_by_vec(raw: Vec<u8>, preview_size: &PreviewSize) -> Result<Vec<u8>, ()> {
        let cv_vector: opencv::core::Vector<u8> = opencv::core::Vector::from(raw);
        let img = imgcodecs::imdecode(&cv_vector, imgcodecs::IMREAD_COLOR).expect("Preview invalid");
        resize_by_cv_mat(&img, preview_size)
    }

    fn resize_by_path(path: &str, preview_size: &PreviewSize) -> Result<Vec<u8>, ()> {
        let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR).expect("Image not found or invalid");
        resize_by_cv_mat(&img, preview_size)
    }

    pub fn resize_by_cv_mat(img: &Mat, preview_size: &PreviewSize) -> Result<Vec<u8>, ()> {
        let scale_factor = to_scale_factor(preview_size, img.size().expect("Image size"));

        let mut resize_out = Mat::default();
        imgproc::resize(
            &img,
            &mut resize_out,
            SIZE_ZERO,
            scale_factor,
            scale_factor,
            imgproc::INTER_AREA,
        )
            .expect("Resize failed");

        let mut encode_params = opencv::core::Vector::<i32>::new();
        encode_params.push(opencv::imgcodecs::IMWRITE_WEBP_QUALITY);
        encode_params.push(85);

        let mut encode_out = opencv::core::Vector::<u8>::new();
        imgcodecs::imencode(".webp", &resize_out, &mut encode_out, &encode_params)
            .expect("Encode failed");

        Ok(encode_out.to_vec())
    }

    fn to_scale_factor(preview_size: &PreviewSize, image_size: Size) -> f64 {
        let target_max_pixels = preview_size.to_max_pixels();
        let target_size = Size {
            width: target_max_pixels as i32,
            height: target_max_pixels as i32,
        };
        let x_ratio = target_size.width as f64 / image_size.width as f64;
        let y_ratio = target_size.height as f64 / image_size.height as f64;

        if x_ratio > y_ratio {
            y_ratio
        } else {
            x_ratio
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn too_wide() {
            let image_size = Size {
                width: 3000,
                height: 1000,
            };
            let scale_factor = to_scale_factor(&PreviewSize::Large, image_size);

            assert_eq!(2. / 3., scale_factor);
        }

        #[test]
        fn too_tall() {
            let image_size = Size {
                width: 2000,
                height: 4000,
            };
            let scale_factor = to_scale_factor(&PreviewSize::Large, image_size);

            assert_eq!(0.5, scale_factor);
        }

        #[test]
        fn both_sides_too_large() {
            let image_size = Size {
                width: 4000,
                height: 4000,
            };
            let scale_factor = to_scale_factor(&PreviewSize::Large, image_size);

            assert_eq!(0.5, scale_factor);
        }
    }
}

mod video {
    use log::warn;
    use opencv::prelude::*;
    use opencv::videoio::{CAP_FFMPEG, VideoCapture};
    use persistance::fs;
    use shared::models::{FotoboekConfig, PreviewSize};
    use crate::modules::preview::image;

    pub fn run_task(config: &FotoboekConfig, abs_path: &String, file_hash: &String) -> Result<(), String> {
        let mut cap = VideoCapture::from_file(abs_path, CAP_FFMPEG)
            .expect("Failed to open video file");

        let mut frame = Mat::default();
        if cap.read(&mut frame).unwrap_or(false) {
            let resized_small = image::resize_by_cv_mat(&frame, &PreviewSize::Small)
                .expect("Resize failed");
            fs::store_preview(
                config,
                file_hash,
                &PreviewSize::Small,
                &resized_small
            ).expect("Store video small preview failed");

            let resized_large = image::resize_by_cv_mat(&frame, &PreviewSize::Large)
                .expect("Resize failed");
            fs::store_preview(
                config,
                file_hash,
                &PreviewSize::Large,
                &resized_large
            ).expect("Store video large preview failed");
        } else {
            warn!("Could not read video frames, skipping preview generation for: {}", abs_path);
        }
        Ok(())
    }
}
