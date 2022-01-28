#[macro_use]
extern crate rocket;
extern crate core;

use dotenv::dotenv;
use persistance::fs;

mod api;
mod config_parser;
mod internal;
mod web;

#[rocket::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = config_parser::parse();

    fs::init(&config);
    web::init(&config).await;
}
