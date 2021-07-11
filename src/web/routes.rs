use crate::db::Database;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::path::Path;

use crate::db::models::Image;
use crate::web::binary_response::BinaryResponse;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        image_by_id_and_original,
        image_by_id_and_size,
        api_scan
    ]
}

#[get("/")]
async fn index(db: Database) -> Template {
    let images = db.run(|c| Image::all(&c)).await;

    #[derive(Serialize)]
    struct IndexContext {
        image_count: usize,
        images: Vec<Image>,
    }

    let context = IndexContext {
        image_count: images.len(),
        images: images,
    };

    Template::render("index", context)
}

#[derive(Debug, PartialEq, FromFormField)]
enum ImageSize {
    Medium,
    Small,
    Thumbnail,
}

#[get("/images/<id>?size=original")]
async fn image_by_id_and_original(id: i32, db: Database) -> Option<NamedFile> {
    let image = db.run(move |c| Image::by_id(&c, id)).await.unwrap();
    let path = Path::new(&image.abs_path);
    NamedFile::open(path).await.ok()
}

#[get("/images/<id>?<size>")]
async fn image_by_id_and_size(id: i32, size: ImageSize, db: Database) -> Option<BinaryResponse> {
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

#[derive(Serialize)]
struct ScanResponse {
    total_count: u32,
    new_count: u32,
    old_count: u32,
}

#[get("/api/scan")]
async fn api_scan(db: Database) -> Json<ScanResponse> {
    let source_paths = crate::source_images::search_by_env_root();
    let total_count = source_paths.len() as u32;
    let (news, olds): (Vec<_>, Vec<_>) = db
        .run(move |c| {
            source_paths
                .iter()
                .map(|source_path| Image {
                    id: None,
                    filename: source_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    abs_path: source_path.to_string_lossy().into_owned(),
                })
                .map(|image| image.insert(c))
                .partition(|result| result.is_ok())
        })
        .await;
    Json(ScanResponse {
        total_count: total_count,
        new_count: news.len() as u32,
        old_count: olds.len() as u32,
    })
}
