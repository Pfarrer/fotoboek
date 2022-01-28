use crate::internal::naive_date_time_rocket::NaiveDateTimeRocket;
use persistance::queries::timeline;
use persistance::queries::timeline::TimelineDates;
use persistance::FotoboekDatabase;
use rocket::serde::json::Json;

#[derive(Debug, FromFormField)]
pub enum Direction {
    Newer,
    Older,
}

#[derive(Debug, FromForm)]
pub struct GetByDateQueryParams {
    pub start_date: NaiveDateTimeRocket,
    pub direction: Direction,
    pub limit: usize,
}

#[get("/timeline/dates")]
pub async fn get_dates(db: FotoboekDatabase) -> Json<TimelineDates> {
    let dates = timeline::dates(db).await;
    Json(dates)
}
