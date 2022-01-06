use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

#[database("db")]
pub struct FotoboekDatabase(diesel::SqliteConnection);
