use log::debug;
use rusqlite::{Connection, Result};

mod schema_migration;

pub struct DataStore {
    conn: Connection,
}

impl DataStore {
    pub fn open(root: &String) -> Result<DataStore, String> {
        debug!("Opening data_store at {}", root);
        let db_path = format!("{}/family_album.db3", root);

        let conn = Connection::open(&db_path).unwrap();
        schema_migration::migrate(&conn).map_err(|err| err.to_string())?;

        Ok(DataStore { conn: conn })
    }
}
