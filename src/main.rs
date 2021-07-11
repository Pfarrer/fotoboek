#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use dotenv;

mod core;
mod db;
mod source_images;
mod web;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    web::init().await;
}
