use persistance::FotoboekDatabase;
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

#[get("/admin/scan")]
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
