#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod fs;
pub mod models;
pub mod queries;
mod sqlite;
mod schema;
mod migrations;

pub use sqlite::FotoboekDatabase;
pub use migrations::migration_fairing;
