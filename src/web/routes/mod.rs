mod ui;
mod images;
mod api;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        ui::index,
        images::image_by_id_and_original,
        images::image_by_id_and_size,
        api::scan
    ]
}
