use crate::db::models::Image;
use crate::db::Database;

mod image_previews;

pub async fn create_tasks_on_new_image(
    db: &Database,
    image: &Image,
) -> std::result::Result<(), std::string::String> {
    image_previews::create_tasks_on_new_image(db, image).await?;
    Ok(())
}
