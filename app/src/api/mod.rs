mod admin;
mod flashback;
mod gallery;
mod images;
mod timeline;
mod videos;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        admin::scan,
        admin::tasks,
        admin::media_statistics,
        images::image_by_id_and_size,
        videos::video_by_id,
        timeline::get_dates,
        gallery::get_paths,
        flashback::get_dates,
    ]
}
