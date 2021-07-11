use opencv::{core::Size, imgcodecs, imgproc, prelude::*};
use std::path::Path;
use crate::core::ImageSize;

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
            ImageSize::Medium => Size {
                width: 500,
                height: 375,
            },
            ImageSize::Small => Size {
                width: 240,
                height: 180,
            },
            ImageSize::Thumbnail => Size {
                width: 100,
                height: 75,
            },
        }
    }
}