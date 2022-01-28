mod admin;
mod flashback;
mod gallery;
mod images;
mod timeline;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        admin::scan,
        images::image_by_id_and_size,
        timeline::get_dates,
        gallery::get_paths,
        flashback::get_dates,
    ]
}
