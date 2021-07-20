use crate::core::models::ImageSize;
use crate::db::Database;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use std::path::Path;

use crate::db::models::Image;
use crate::web::binary_response::BinaryResponse;

#[derive(Debug, PartialEq, FromFormField)]
pub enum RestImageSize {
    Large,
    Medium,
    Small,
}

#[get("/images/<id>?size=original")]
pub async fn image_by_id_and_original(id: i32, db: Database) -> Option<NamedFile> {
    let image = db.run(move |c| Image::by_id(&c, id)).await.unwrap();
    let path = Path::new(&image.abs_path);
    NamedFile::open(path).await.ok()
}

#[get("/images/<id>?<size>")]
pub async fn image_by_id_and_size(
    id: i32,
    size: RestImageSize,
    db: Database,
) -> Option<BinaryResponse> {
    let image = db.run(move |c| Image::by_id(&c, id)).await.unwrap();
    let original_abs_path = Path::new(&image.abs_path);

    let image_size = match size {
        RestImageSize::Large => ImageSize::Large,
        RestImageSize::Medium => ImageSize::Medium,
        RestImageSize::Small => ImageSize::Small,
    };
    let resized_image_raw =
        crate::core::modules::image_previews::resize(&original_abs_path, &image_size).ok()?;
    Some(BinaryResponse {
        body: resized_image_raw,
        content_type: ContentType::JPEG,
    })
}
