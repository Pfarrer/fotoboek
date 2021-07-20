use crate::core::models::ImageSize;
use crate::db::models::Image;
use crate::db::models::Task;
use crate::db::Database;
use opencv::{core::Size, imgcodecs, imgproc, prelude::*};
use std::path::Path;

pub const MODULE_ID: &str = "image_previews";

pub async fn create_tasks_on_new_image(db: &Database, image: &Image) -> Result<(), String> {
    let image_id = image.id.expect("Image must have an id");
    let task_large = db
        .run(move |c| {
            Task {
                id: None,
                image_id,
                module: MODULE_ID.into(),
                action: "large".into(),
                priority: 100,
                blocked_by_task_id: None,
            }
            .insert(c)
        })
        .await?;

    let task_medium = db
        .run(move |c| {
            Task {
                id: None,
                image_id,
                module: MODULE_ID.into(),
                action: "medium".into(),
                priority: 100,
                blocked_by_task_id: task_large.id,
            }
            .insert(c)
        })
        .await?;

    db.run(move |c| {
        Task {
            id: None,
            image_id,
            module: MODULE_ID.into(),
            action: "small".into(),
            priority: 100,
            blocked_by_task_id: task_medium.id,
        }
        .insert(c)
    })
    .await?;

    Ok(())
}

pub fn run_task(task: &Task) -> Result<(), String> {
    Err("No error".into())
}

pub fn resize(path: &Path, image_size: &ImageSize) -> Result<Vec<u8>, ()> {
    let img = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)
        .expect("Image not found");

    let mut dst = Mat::default();
    let size = image_size.to_opencv_size();
    imgproc::resize(&img, &mut dst, size, 0., 0., imgproc::INTER_AREA).expect("Resize failed");
    let mut out = opencv::core::Vector::<u8>::new();
    let params = opencv::core::Vector::<i32>::new();
    imgcodecs::imencode(".jpg", &dst, &mut out, &params).expect("Encode failed");

    Ok(out.to_vec())
}

impl ImageSize {
    fn to_opencv_size(&self) -> Size {
        match self {
            ImageSize::Large => Size {
                width: 1920,
                height: 1080,
            },
            ImageSize::Medium => Size {
                width: 500,
                height: 375,
            },
            ImageSize::Small => Size {
                width: 100,
                height: 75,
            },
        }
    }
}
