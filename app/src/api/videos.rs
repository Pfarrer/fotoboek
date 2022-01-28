use persistance::models::{FileMetadata};
use persistance::{fs, FotoboekDatabase};
use rocket::fs::NamedFile;
use rocket::State;
use shared::models::{FotoboekConfig};
use std::path::Path;

#[get("/videos/<file_id>")]
pub async fn video_by_id(
    db: FotoboekDatabase,
    config: &State<FotoboekConfig>,
    file_id: i32,
) -> Option<NamedFile> {
    let metadata = FileMetadata::by_file_id(&db, file_id).await?;
    let path = fs::video_path(config, &metadata.file_hash);
    NamedFile::open(Path::new(&path)).await.ok()
}
