use crate::db::models::Image;
use crate::db::Database;
use std::path::PathBuf;

pub async fn on_add(db: &Database, source_path: &PathBuf) -> Result<(), String> {
    let image = Image::from_path_buf(source_path);
    if let Some(image) = db.run(move |c| image.insert(c)).await? {
        // TODO remove await
        crate::core::modules::create_tasks_on_new_image(db, &image)
            .await
            .expect("Core modules create_tasks_on_new_image failed");
        Ok(())
    } else {
        Err("Image already registered".into())
    }
}
