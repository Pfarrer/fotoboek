mod api;
mod images;
mod ui;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        ui::index,
        images::image_by_id_and_original,
        images::image_by_id_and_size,
        api::images,
        api::tasks,
        api::metadata,
        api::image_paths,
        api::scan,
    ]
}
