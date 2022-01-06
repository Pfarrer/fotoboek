#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use persistance::fs;

mod web;
mod api;
mod internal;
mod config_parser;

#[rocket::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = config_parser::parse();

    fs::init(&config);
    web::init(&config).await;
}
