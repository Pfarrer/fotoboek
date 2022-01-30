use persistance::models::FileMetadata;
use persistance::{fs, FotoboekDatabase};
use rocket::fs::NamedFile;
use rocket::State;
use shared::models::{FotoboekConfig, PreviewSize};
use std::path::Path;

#[derive(Debug, PartialEq, FromFormField)]
pub enum RestImageSize {
    Large,
    Small,
}

impl RestImageSize {
    fn to_preview_size(&self) -> PreviewSize {
        match self {
            RestImageSize::Large => PreviewSize::Large,
            RestImageSize::Small => PreviewSize::Small,
        }
    }
}

#[get("/images/<file_id>?<size>")]
pub async fn image_by_id_and_size(
    db: FotoboekDatabase,
    config: &State<FotoboekConfig>,
    file_id: i32,
    size: RestImageSize,
) -> Option<NamedFile> {
    let metadata = FileMetadata::by_file_id(&db, file_id).await?;
    let path = fs::file_preview_path(config, &metadata.file_hash, &size.to_preview_size());
    NamedFile::open(Path::new(&path)).await.ok()
}
