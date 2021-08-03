use crate::db::Database;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;

mod binary_response;
mod routes;

pub async fn init() {
    rocket::build()
        .attach(Template::fairing())
        .attach(crate::db::Database::fairing())
        .attach(worker_thread_fairing())
        .mount(
            "/assets",
            rocket::fs::FileServer::from(rocket::fs::relative!("assets")),
        )
        .mount("/", routes::routes())
        .launch()
        .await
        .unwrap();
}

fn worker_thread_fairing() -> AdHoc {
    AdHoc::on_liftoff("worker_thread", |rocket| {
        Box::pin(async move {
            let db = Database::get_one(rocket).await.unwrap();
            crate::core::worker::spawn(db);
        })
    })
}
