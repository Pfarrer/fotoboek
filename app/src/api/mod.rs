mod admin;
mod images;
mod timeline;
mod gallery;
mod flashback;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        admin::scan,
        images::image_by_id_and_size,
        timeline::get_dates,
        gallery::get_paths,
        flashback::get_dates,
    ]
}
