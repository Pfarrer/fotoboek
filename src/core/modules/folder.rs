use crate::db::models::Folder;
use crate::db::models::Image;
use crate::db::models::Task;
use crate::db::Database;
use std::path::Path;

pub const MODULE_ID: &str = "folder";

pub async fn create_tasks_on_new_image(db: &Database, image: &Image) -> Result<(), String> {
    let image_id = image.id.expect("Image must have an id");

    db.run(move |c| {
        Task {
            id: None,
            image_id,
            module: MODULE_ID.into(),
            priority: 100,
        }
        .insert(c)
    })
    .await?;

    Ok(())
}

pub fn run_task(conn: &diesel::SqliteConnection, task: &Task) -> Result<(), String> {
    let source_image_root = dotenv::var("IMAGE_ROOT").unwrap();

    let image = Image::by_id(conn, task.image_id)?;
    let abs_path = Path::new(&image.abs_path);

    for (i, path) in abs_path.ancestors().skip(1).enumerate() {
        if !path.is_dir() {
            continue;
        }
        if !path.starts_with(&source_image_root) {
            break;
        }

        Folder {
            abs_path: path.to_str().unwrap().into(),
            image_id: task.image_id,
            distance: i as i32,
        }
        .insert(conn)?;
    }

    Ok(())
}
