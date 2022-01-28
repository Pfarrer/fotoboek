use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use std::path::Path;

use crate::api;
use persistance::FotoboekDatabase;
use shared::models::FotoboekConfig;

pub async fn init(config: &FotoboekConfig) {
    rocket::build()
        .attach(FotoboekDatabase::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            persistance::migration_fairing,
        ))
        .attach(worker_thread_fairing(config))
        .manage(config.clone())
        .mount("/api", api::routes())
        .mount("/", webapp_route(config))
        .launch()
        .await
        .expect("Rocket start failed");
}

fn webapp_route(config: &FotoboekConfig) -> FileServer {
    let relative_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(&config.webapp_files_path);
    FileServer::from(relative_path)
}

fn worker_thread_fairing(config: &FotoboekConfig) -> AdHoc {
    let num_worker_threads = config.num_worker_threads;
    let config_copy = config.clone();
    AdHoc::on_liftoff("worker_thread", move |rocket| {
        Box::pin(async move {
            for i in 0..num_worker_threads {
                let db = FotoboekDatabase::get_one(rocket).await.unwrap();
                logic::worker::spawn(db, &config_copy, i);
            }
        })
    })
}
