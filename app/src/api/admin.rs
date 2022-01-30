use persistance::models::Task;
use persistance::queries::admin::MediaDateMap;
use persistance::{queries, FotoboekDatabase};
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use shared::models::FotoboekConfig;

#[derive(Serialize)]
pub struct ScanResponse {
    images_total: usize,
    images_added: usize,
    images_removed: usize,
    videos_total: usize,
    videos_added: usize,
    videos_removed: usize,
}

#[post("/admin/scan")]
pub async fn scan(db: FotoboekDatabase, config: &State<FotoboekConfig>) -> Json<ScanResponse> {
    let images_result = logic::source_images::search_and_update_db(&db, &config).await;
    let videos_result = logic::source_videos::search_and_update_db(&db, &config).await;
    Json(ScanResponse {
        images_total: images_result.total_count,
        images_added: images_result.added_count,
        images_removed: images_result.removed_count,
        videos_total: videos_result.total_count,
        videos_added: videos_result.added_count,
        videos_removed: videos_result.removed_count,
    })
}

#[get("/admin/tasks")]
pub async fn tasks(db: FotoboekDatabase) -> Json<Vec<Task>> {
    let tasks = Task::all(&db).await;
    Json(tasks)
}

#[get("/admin/media-statistics")]
pub async fn media_statistics(db: FotoboekDatabase) -> Json<MediaDateMap> {
    let media_date_map = queries::admin::get_media_date_map(&db).await;
    Json(media_date_map)
}
