use crate::FotoboekDatabase;
use log::error;
use rocket_sync_db_pools::rocket::{Build, Rocket};

embed_migrations!();

pub async fn migration_fairing(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = FotoboekDatabase::get_one(&rocket)
        .await
        .expect("Failed to get database connection");
    db.run(|conn| match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}
