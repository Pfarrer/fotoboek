use std::time::Instant;
use persistance::models::{File, Task};
use persistance::FotoboekDatabase;
use log::info;
use shared::models::FotoboekConfig;

mod metadata;
mod preview;

pub async fn create_tasks_on_new_image(
    db: &FotoboekDatabase,
    file: &File,
) -> std::result::Result<(), std::string::String> {
    metadata::create_tasks_on_new_image(db, file).await?;
    preview::create_tasks_on_new_image(db, file).await?;
    Ok(())
}

pub async fn run_task(db: &FotoboekDatabase, config: &FotoboekConfig, task: &Task) -> Result<(), String> {
    let start_time = Instant::now();

    match task.module.as_str() {
        metadata::MODULE_ID => metadata::run_task(db, config, task).await,
        preview::MODULE_ID => preview::run_task(db, config, task).await,
        &_ => Err(format!("Unknown module in {:?}", task).into()),
    }?;

    info!(
        "{:?} successfully finished after {:.4}ms",
        task,
        start_time.elapsed().as_millis()
    );

    Ok(())
}
