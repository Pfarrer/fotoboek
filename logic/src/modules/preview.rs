use persistance::models::{File, FileMetadata, Task};
use persistance::FotoboekDatabase;
use shared::models::FotoboekConfig;
use shared::path_utils::rel_to_abs;
use std::string::ToString;

pub const MODULE_ID: &str = "preview";

pub async fn create_tasks_on_new_file(db: &FotoboekDatabase, file: &File) -> Result<(), String> {
    Task {
        id: None,
        file_id: file.id.unwrap(),
        module: MODULE_ID.into(),
        priority: 200,
        work_started_at: chrono::NaiveDateTime::from_timestamp(0, 0),
    }
    .insert(db)
    .await?;

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

    pub fn run_task(
        config: &FotoboekConfig,
        abs_path: &String,
        file_hash: &String,
    ) -> Result<(), String> {
        let large_preview_bytes =
            resize_by_path(abs_path, &PreviewSize::Large).expect("Resize large failed");
        fs::store_preview(config, file_hash, &PreviewSize::Large, &large_preview_bytes)?;

        let small_preview_bytes =
            resize_by_vec(large_preview_bytes, &PreviewSize::Small).expect("Resize small failed");
        fs::store_preview(config, file_hash, &PreviewSize::Small, &small_preview_bytes)
    }

    fn resize_by_vec(raw: Vec<u8>, preview_size: &PreviewSize) -> Result<Vec<u8>, ()> {
        let cv_vector: opencv::core::Vector<u8> = opencv::core::Vector::from(raw);
        let img =
            imgcodecs::imdecode(&cv_vector, imgcodecs::IMREAD_COLOR).expect("Preview invalid");
        resize_by_cv_mat(&img, preview_size)
    }

    fn resize_by_path(path: &str, preview_size: &PreviewSize) -> Result<Vec<u8>, ()> {
        let img =
            imgcodecs::imread(path, imgcodecs::IMREAD_COLOR).expect("Image not found or invalid");
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
        fn max(a: f64, b: f64) -> f64 {
            if a > b {
                a
            } else {
                b
            }
        }
        fn min(a: f64, b: f64) -> f64 {
            if a < b {
                a
            } else {
                b
            }
        }

        match preview_size {
            PreviewSize::Large => {
                let target_max_pixels = 2000 as f64;
                let min_factor = min(
                    target_max_pixels / image_size.width as f64,
                    target_max_pixels / image_size.height as f64,
                );
                min(min_factor, 1.)
            }
            PreviewSize::Small => {
                let target_min_pixels = 200 as f64;
                let max_factor = max(
                    target_min_pixels / image_size.width as f64,
                    target_min_pixels / image_size.height as f64,
                );
                min(max_factor, 1.)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn preview_size_larger_image_to_large_preview() {
            let scale_factor_horizontal = to_scale_factor(
                &PreviewSize::Large,
                Size {
                    width: 3000,
                    height: 1000,
                },
            );
            assert_eq!(2. / 3., scale_factor_horizontal);

            let scale_factor_vertical = to_scale_factor(
                &PreviewSize::Large,
                Size {
                    width: 2000,
                    height: 4000,
                },
            );
            assert_eq!(0.5, scale_factor_vertical);

            let scale_factor_quadratic = to_scale_factor(
                &PreviewSize::Large,
                Size {
                    width: 4000,
                    height: 4000,
                },
            );
            assert_eq!(0.5, scale_factor_quadratic);
        }

        #[test]
        fn preview_size_smaller_image_to_large_preview() {
            let scale_factor_horizontal = to_scale_factor(
                &PreviewSize::Large,
                Size {
                    width: 300,
                    height: 100,
                },
            );
            assert_eq!(1., scale_factor_horizontal);

            let scale_factor_vertical = to_scale_factor(
                &PreviewSize::Large,
                Size {
                    width: 200,
                    height: 400,
                },
            );
            assert_eq!(1., scale_factor_vertical);

            let scale_factor_quadratic = to_scale_factor(
                &PreviewSize::Large,
                Size {
                    width: 400,
                    height: 400,
                },
            );
            assert_eq!(1., scale_factor_quadratic);
        }

        #[test]
        fn preview_size_larger_image_to_small_preview() {
            let scale_factor_horizontal = to_scale_factor(
                &PreviewSize::Small,
                Size {
                    width: 2000,
                    height: 1000,
                },
            );
            assert_eq!(0.2, scale_factor_horizontal);

            let scale_factor_vertical = to_scale_factor(
                &PreviewSize::Small,
                Size {
                    width: 1000,
                    height: 2000,
                },
            );
            assert_eq!(0.2, scale_factor_vertical);

            let scale_factor_quadratic = to_scale_factor(
                &PreviewSize::Small,
                Size {
                    width: 2000,
                    height: 2000,
                },
            );
            assert_eq!(0.1, scale_factor_quadratic);
        }

        #[test]
        fn preview_size_smaller_image_to_small_preview() {
            let scale_factor_horizontal = to_scale_factor(
                &PreviewSize::Small,
                Size {
                    width: 400,
                    height: 100,
                },
            );
            assert_eq!(1., scale_factor_horizontal);

            let scale_factor_vertical = to_scale_factor(
                &PreviewSize::Small,
                Size {
                    width: 100,
                    height: 600,
                },
            );
            assert_eq!(1., scale_factor_vertical);

            let scale_factor_quadratic = to_scale_factor(
                &PreviewSize::Small,
                Size {
                    width: 150,
                    height: 150,
                },
            );
            assert_eq!(1., scale_factor_quadratic);
        }
    }
}

mod video {
    use log::warn;
    use opencv::prelude::*;
    use opencv::videoio::{VideoCapture, CAP_FFMPEG};

    use persistance::fs;
    use shared::models::{FotoboekConfig, PreviewSize};

    use crate::modules::preview::image;

    pub fn run_task(
        config: &FotoboekConfig,
        abs_path: &String,
        file_hash: &String,
    ) -> Result<(), String> {
        let mut cap =
            VideoCapture::from_file(abs_path, CAP_FFMPEG).expect("Failed to open video file");

        let mut frame = Mat::default();
        if cap.read(&mut frame).unwrap_or(false) {
            let resized_small =
                image::resize_by_cv_mat(&frame, &PreviewSize::Small).expect("Resize failed");
            fs::store_preview(config, file_hash, &PreviewSize::Small, &resized_small)
                .expect("Store video small preview failed");

            let resized_large =
                image::resize_by_cv_mat(&frame, &PreviewSize::Large).expect("Resize failed");
            fs::store_preview(config, file_hash, &PreviewSize::Large, &resized_large)
                .expect("Store video large preview failed");
        } else {
            warn!(
                "Could not read video frames, skipping preview generation for: {}",
                abs_path
            );
        }
        Ok(())
    }
}
