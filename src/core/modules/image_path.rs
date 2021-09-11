use crate::db::models::Image;
use crate::db::models::ImagePath;
use crate::db::models::Task;
use crate::db::Database;
use std::path::Path;

pub const MODULE_ID: &str = "image_path";

pub async fn create_tasks_on_new_image(db: &Database, image: &Image) -> Result<(), String> {
    let image_id = image.id.expect("Image must have an id");

    db.run(move |c| Task::new(image_id, MODULE_ID.into(), 100).insert(c))
        .await?;

    Ok(())
}

pub fn run_task(conn: &diesel::SqliteConnection, task: &Task) -> Result<(), String> {
    let source_image_root = dotenv::var("IMAGE_ROOT").unwrap();

    let image = Image::by_id(conn, task.image_id)?;
    let abs_path = Path::new(&image.abs_path);

    let indexes_and_paths = abs_path
        .ancestors()
        .skip(1)
        .enumerate()
        .collect::<Vec<(usize, &Path)>>();
    let reversed_indexes_and_paths = indexes_and_paths.iter().rev();
    let mut parent_dir_path: Option<String> = None;
    for (i, path) in reversed_indexes_and_paths {
        if !path.is_dir() || !path.starts_with(&source_image_root) {
            continue;
        }

        ImagePath {
            abs_dir_path: path.to_str().unwrap().into(),
            image_id: task.image_id,
            distance: *i as i32,
            parent_dir_path,
        }
        .save(conn)?;

        parent_dir_path = Some(path.to_str().unwrap().into());
    }

    Ok(())
}
