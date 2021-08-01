use crate::core::models::ImageSize;
use crate::db::models::Preview;
use crate::db::Database;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use std::path::Path;

use crate::db::models::Image;
use crate::web::binary_response::BinaryResponse;

#[derive(Debug, PartialEq, FromFormField)]
pub enum RestImageSize {
    Large,
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
    let image_size = match size {
        RestImageSize::Large => ImageSize::Large,
        RestImageSize::Small => ImageSize::Small,
    };
    let preview = db
        .run(move |conn| Preview::by_image_id_and_size(&conn, id, image_size.to_string()))
        .await
        .unwrap();

    Some(BinaryResponse {
        body: preview.data,
        content_type: ContentType::JPEG,
    })
}
