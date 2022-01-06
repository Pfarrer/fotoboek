use persistance::FotoboekDatabase;
use rocket::serde::json::Json;
use persistance::queries::gallery;
use persistance::queries::gallery::GalleryPath;

#[get("/gallery/paths")]
pub async fn get_paths(db: FotoboekDatabase) -> Json<GalleryPath> {
    let paths = gallery::paths(db).await;
    Json(paths)
}
