use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

pub mod models;
mod schema;

#[database("db")]
pub struct Database(diesel::SqliteConnection);
