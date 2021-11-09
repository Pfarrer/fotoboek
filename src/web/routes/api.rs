use crate::core::image_event_handler;
use crate::db::models::*;
use crate::db::Database;
use rocket::futures::future;
use rocket::serde::json::Json;
use serde::Serialize;

#[get("/api/images")]
pub async fn images(db: Database) -> Json<Vec<Image>> {
    let images = db.run(move |conn| Image::all(conn)).await;
    Json(images)
}

#[get("/api/metadata")]
pub async fn metadata(db: Database) -> Json<Vec<Metadata>> {
    let metadata = db.run(move |conn| Metadata::all(conn)).await;
    Json(metadata)
}

#[get("/api/image_paths")]
pub async fn image_paths(db: Database) -> Json<Vec<ImagePath>> {
    let image_paths = db.run(move |conn| ImagePath::all(conn)).await;
    Json(image_paths)
}

#[get("/api/tasks")]
pub async fn get_tasks(db: Database) -> Json<Vec<Task>> {
    let tasks = db.run(move |conn| Task::all(conn)).await;
    Json(tasks)
}

#[post("/api/tasks?<image_id>")]
pub async fn post_tasks_for_image_id(db: Database, image_id: i32) -> Result<(), String> {
    let image: Image = db.run(move |conn| Image::by_id(conn, image_id)).await?;
    crate::core::modules::create_tasks_on_new_image(&db, &image).await?;
    Ok(())
}

#[derive(Serialize)]
pub struct ScanResponse {
    total_count: u32,
    added_count: u32,
}

#[get("/api/scan")]
pub async fn scan(db: Database) -> Json<ScanResponse> {
    let source_paths = crate::source_images::search_by_env_root();
    let total_count = source_paths.len() as u32;

    // let remove_futures = // TODOsource_paths
    //     .iter()
    //     .map(|source_path| image_event_handler::on_add(&db, source_path))
    //     .collect::<Vec<_>>();
    // let remove_results = future::join_all(add_futures).await;
    // let removed_count = remove_results.iter().filter(|r| r.is_ok()).count() as u32;

    let add_futures = source_paths
        .iter()
        .map(|source_path| image_event_handler::try_add(&db, source_path))
        .collect::<Vec<_>>();
    let add_results = future::join_all(add_futures).await;
    let added_count = add_results.iter().filter(|r| r.is_ok()).count() as u32;

    Json(ScanResponse {
        total_count,
        added_count,
        // removed_count,
    })
}
