#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::database;

mod cl_args;
mod source_images;

#[database("db")]
struct Database(diesel::SqliteConnection);

#[get("/")]
fn hello(mut conn: Database) -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let args = cl_args::parse_args().unwrap();
    let source_image = source_images::search(&args.image_root);

    rocket::build()
        .attach(Database::fairing())
        .mount("/hello", routes![hello])
        .launch()
        .await.unwrap();
}
