use crate::core::models::ImageSize;
use crate::db::models::Image;
use crate::db::models::Preview;
use crate::db::models::Task;
use crate::db::Database;
use opencv::{core::Size, imgcodecs, imgproc, prelude::*};
use std::path::Path;

pub const MODULE_ID: &str = "preview";

const SIZE_ZERO: Size = Size {
    height: 0,
    width: 0,
};

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

    {
        let data =
            resize_by_vec(preview_large.data, &ImageSize::Small).expect("Resize small failed");

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
    let scale_factor = image_size.to_scale_factor(img.size().expect("Image size"));
    imgproc::resize(
        &img,
        &mut dst,
        SIZE_ZERO,
        scale_factor,
        scale_factor,
        imgproc::INTER_AREA,
    )
    .expect("Resize failed");
    let mut out = opencv::core::Vector::<u8>::new();
    let params = opencv::core::Vector::<i32>::new();
    imgcodecs::imencode(".jpg", &dst, &mut out, &params).expect("Encode failed");

    Ok(out.to_vec())
}

impl ImageSize {
    fn target_size(&self) -> Size {
        match self {
            ImageSize::Large => Size {
                width: 2000,
                height: 2000,
            },
            ImageSize::Small => Size {
                width: 200,
                height: 200,
            },
        }
    }

    fn to_scale_factor(&self, image_size: Size) -> f64 {
        let target_size = self.target_size();
        let x_ratio = target_size.width as f64 / image_size.width as f64;
        let y_ratio = target_size.height as f64 / image_size.height as f64;

        if x_ratio > y_ratio {
            y_ratio
        } else {
            x_ratio
        }
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
        let scale_factor = ImageSize::Large.to_scale_factor(image_size);

        assert_eq!(2. / 3., scale_factor);
    }

    #[test]
    fn too_tall() {
        let image_size = Size {
            width: 2000,
            height: 4000,
        };
        let scale_factor = ImageSize::Large.to_scale_factor(image_size);

        assert_eq!(0.5, scale_factor);
    }

    #[test]
    fn both_sides_too_large() {
        let image_size = Size {
            width: 4000,
            height: 4000,
        };
        let scale_factor = ImageSize::Large.to_scale_factor(image_size);

        assert_eq!(0.5, scale_factor);
    }
}
