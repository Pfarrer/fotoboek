use crate::core::image_event_handler;
use crate::db::models::{Image, Task};
use crate::db::Database;
use rocket::futures::future;
use rocket::serde::json::Json;
use serde::Serialize;

#[get("/api/images")]
pub async fn images(db: Database) -> Json<Vec<Image>> {
    let images = db.run(move |conn| Image::all(conn)).await;
    Json(images)
}

#[get("/api/tasks")]
pub async fn tasks(db: Database) -> Json<Vec<Task>> {
    let tasks = db.run(move |conn| Task::all(conn)).await;
    Json(tasks)
}

#[derive(Serialize)]
pub struct ScanResponse {
    total_count: u32,
    new_count: u32,
    old_count: u32,
}

#[get("/api/scan")]
pub async fn scan(db: Database) -> Json<ScanResponse> {
    let source_paths = crate::source_images::search_by_env_root();
    let total_count = source_paths.len() as u32;

    let add_futures = source_paths
        .iter()
        .map(|source_path| image_event_handler::on_add(&db, source_path))
        .collect::<Vec<_>>();
    let add_results = future::join_all(add_futures).await;
    let new_count = add_results.iter().filter(|r| r.is_ok()).count() as u32;

    Json(ScanResponse {
        total_count,
        new_count,
        old_count: total_count - new_count,
    })
}
