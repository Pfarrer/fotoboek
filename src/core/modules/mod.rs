use crate::db::models::{Image, Task};
use crate::db::Database;
use std::time::Instant;

pub mod image_previews;

pub async fn create_tasks_on_new_image(
    db: &Database,
    image: &Image,
) -> std::result::Result<(), std::string::String> {
    image_previews::create_tasks_on_new_image(db, image).await?;
    Ok(())
}

pub fn run_task(conn: &diesel::SqliteConnection, task: &Task) -> Result<(), String> {
    let start_time = Instant::now();

    match task.module.as_str() {
        image_previews::MODULE_ID => image_previews::run_task(task),
        &_ => Err(format!("Unknown module in task {:?}", task).into()),
    }?;

    info!(
        "Task {:?} successfully finished after {:.4}ms",
        task,
        start_time.elapsed().as_millis()
    );

    Ok(())
}
