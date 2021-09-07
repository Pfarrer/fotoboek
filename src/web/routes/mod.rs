use gallery::DisplaySettings;
use rocket::response::Redirect;

mod api;
mod flashback;
pub mod gallery;
mod images;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        gallery::gallery,
        gallery::image_by_id,
        flashback::flashback,
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

#[get("/")]
fn index() -> Redirect {
    let settings = DisplaySettings {
        path: None,
        deep: None,
    };
    Redirect::to(rocket::uri!(gallery::gallery(settings)))
}
