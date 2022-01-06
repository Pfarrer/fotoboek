use rocket::serde::{Serialize, json::Json};
use persistance::FotoboekDatabase;
use shared::models::FotoboekConfig;
use rocket::State;

#[derive(Serialize)]
pub struct ScanResponse {
    total_count: usize,
    added_count: usize,
    removed_count: usize,
}

#[get("/admin/scan")]
pub async fn scan(db: FotoboekDatabase, config: &State<FotoboekConfig>) -> Json<ScanResponse> {
    let result = logic::source_images::search_and_update_db(db, &config).await;
    Json(ScanResponse {
        total_count: result.total_count,
        added_count: result.added_count,
        removed_count: result.removed_count,
    })
}
