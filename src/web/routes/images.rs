use crate::db::Database;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use std::path::Path;

use crate::db::models::Image;
use crate::web::binary_response::BinaryResponse;

#[derive(Debug, PartialEq, FromFormField)]
pub enum ImageSize {
    Medium,
    Small,
    Thumbnail,
}

#[get("/images/<id>?size=original")]
pub async fn image_by_id_and_original(id: i32, db: Database) -> Option<NamedFile> {
    let image = db.run(move |c| Image::by_id(&c, id)).await.unwrap();
    let path = Path::new(&image.abs_path);
    NamedFile::open(path).await.ok()
}

#[get("/images/<id>?<size>")]
pub async fn image_by_id_and_size(id: i32, size: ImageSize, db: Database) -> Option<BinaryResponse> {
    let image = db.run(move |c| Image::by_id(&c, id)).await.unwrap();
    let original_abs_path = Path::new(&image.abs_path);

    let image_size = match size {
        ImageSize::Medium => crate::core::ImageSize::Medium,
        ImageSize::Small => crate::core::ImageSize::Small,
        ImageSize::Thumbnail => crate::core::ImageSize::Thumbnail,
    };
    let resized_image_raw = crate::core::resize(&original_abs_path, &image_size).ok()?;
    Some(BinaryResponse {
        body: resized_image_raw,
        content_type: ContentType::JPEG,
    })
}
