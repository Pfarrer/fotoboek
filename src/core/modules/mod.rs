use crate::db::models::{Image, Task};
use crate::db::Database;
use std::time::Instant;

mod metadata;
mod preview;

pub async fn create_tasks_on_new_image(
    db: &Database,
    image: &Image,
) -> std::result::Result<(), std::string::String> {
    metadata::create_tasks_on_new_image(db, image).await?;
    preview::create_tasks_on_new_image(db, image).await?;
    Ok(())
}

pub fn run_task(conn: &diesel::SqliteConnection, task: &Task) -> Result<(), String> {
    let start_time = Instant::now();

    match task.module.as_str() {
        metadata::MODULE_ID => metadata::run_task(conn, task),
        preview::MODULE_ID => preview::run_task(conn, task),
        &_ => Err(format!("Unknown module in task {:?}", task).into()),
    }?;

    info!(
        "Task {:?} successfully finished after {:.4}ms",
        task,
        start_time.elapsed().as_millis()
    );

    Ok(())
}
