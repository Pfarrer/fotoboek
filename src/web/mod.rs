use rocket_dyn_templates::Template;

mod routes;

pub async fn init() {
    rocket::build()
        .attach(Template::fairing())
        .attach(crate::db::Database::fairing())
        .mount("/", routes::routes())
        .launch()
        .await
        .unwrap();
}
