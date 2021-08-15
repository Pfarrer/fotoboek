mod api;
mod images;
mod ui;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        ui::gallery,
        ui::image_by_id,
        images::image_by_id_and_original,
        images::image_by_id_and_size,
        api::images,
        api::get_tasks,
        api::post_tasks_for_image_id,
        api::metadata,
        api::image_paths,
        api::scan,
    ]
}
