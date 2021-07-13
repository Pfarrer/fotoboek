use crate::db::models::Image;
use crate::db::Database;
use std::path::PathBuf;

pub async fn on_add(db: &Database, source_path: &PathBuf) -> Result<(), ()> {
    let image = Image::from_path_buf(source_path);
    db.run(move |c| image.insert(c)).await
}
