use persistance::queries::flashback;
use persistance::queries::flashback::FlashbackDates;
use persistance::FotoboekDatabase;
use rocket::serde::json::Json;

#[get("/flashback/dates")]
pub async fn get_dates(db: FotoboekDatabase) -> Json<FlashbackDates> {
    let dates = flashback::dates(db).await;
    Json(dates)
}
