use crate::core::models::ImageSize;
use crate::db::models::Image;
use crate::db::models::Preview;
use crate::db::models::Task;
use crate::db::Database;
use opencv::{core::Size, imgcodecs, imgproc, prelude::*};
use std::path::Path;

pub const MODULE_ID: &str = "preview";

pub async fn create_tasks_on_new_image(db: &Database, image: &Image) -> Result<(), String> {
    let image_id = image.id.expect("Image must have an id");

    db.run(move |c| {
        Task {
            id: None,
            image_id,
            module: MODULE_ID.into(),
            priority: 200,
        }
        .insert(c)
    })
    .await?;

    Ok(())
}

pub fn run_task(conn: &diesel::SqliteConnection, task: &Task) -> Result<(), String> {
    let preview_large = {
        let image = Image::by_id(conn, task.image_id)?;
        let abs_path = Path::new(&image.abs_path);
        let data = resize_by_path(abs_path, &ImageSize::Large).expect("Resize large failed");

        Preview {
            id: None,
            image_id: task.image_id,
            size: ImageSize::Large.to_string(),
            data,
        }
        .insert(conn)?
    };

    let preview_medium = {
        let data =
            resize_by_vec(preview_large.data, &ImageSize::Medium).expect("Resize medium failed");

        Preview {
            id: None,
            image_id: task.image_id,
            size: ImageSize::Medium.to_string(),
            data,
        }
        .insert(conn)?
    };

    {
        let data =
            resize_by_vec(preview_medium.data, &ImageSize::Small).expect("Resize medium failed");

        Preview {
            id: None,
            image_id: task.image_id,
            size: ImageSize::Small.to_string(),
            data,
        }
        .insert(conn)?;
    };

    Ok(())
}

fn resize_by_vec(raw: Vec<u8>, image_size: &ImageSize) -> Result<Vec<u8>, ()> {
    let cv_vector: opencv::core::Vector<u8> = opencv::core::Vector::from(raw);
    let img = imgcodecs::imdecode(&cv_vector, imgcodecs::IMREAD_COLOR).expect("Preview invalid");
    resize_by_cv_mat(&img, image_size)
}

fn resize_by_path(path: &Path, image_size: &ImageSize) -> Result<Vec<u8>, ()> {
    let img = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)
        .expect("Image not found or invalid");
    resize_by_cv_mat(&img, image_size)
}

fn resize_by_cv_mat(img: &Mat, image_size: &ImageSize) -> Result<Vec<u8>, ()> {
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
                width: 2000,
                height: 1500,
            },
            ImageSize::Medium => Size {
                width: 600,
                height: 450,
            },
            ImageSize::Small => Size {
                width: 200,
                height: 150,
            },
        }
    }
}
