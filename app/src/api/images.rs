use persistance::models::{File, FileMetadata};
use persistance::{fs, FotoboekDatabase};
use rocket::fs::NamedFile;
use rocket::State;
use shared::models::{FotoboekConfig, PreviewSize};
use std::path::Path;

#[derive(Debug, PartialEq, FromFormField)]
pub enum RestImageSize {
    Large,
    Small,
    Original,
}

impl RestImageSize {
    fn to_preview_size(&self) -> PreviewSize {
        match self {
            RestImageSize::Large => PreviewSize::Large,
            RestImageSize::Small => PreviewSize::Small,
            RestImageSize::Original => panic!("Not mappable"),
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
    let path = match &size {
        RestImageSize::Original => {
            let file = File::by_id(&db, file_id).await.ok()?;
            format!("{}/{}", config.media_source_path, file.rel_path)
        }
        _ => {
            let metadata = FileMetadata::by_file_id(&db, file_id).await?;
            fs::preview_file_path(config, &metadata.file_hash, &size.to_preview_size())
        }
    };
    NamedFile::open(Path::new(&path)).await.ok()
}
