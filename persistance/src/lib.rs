#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod fs;
mod migrations;
pub mod models;
pub mod queries;
mod schema;
mod sqlite;

pub use migrations::migration_fairing;
pub use sqlite::FotoboekDatabase;
