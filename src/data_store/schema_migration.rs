use log::{debug, info};
use rusqlite::{params, Connection, Result};

static CONFIG_KEY_SCHEMA_VERSION: &str = "schema_version";

static SCHEMA_V1: &str = "CREATE TABLE config (
    k VARCHAR(255) UNIQUE,
    v VARCHAR(4000)
)";

static SCHEMA_V2: &str = "CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    rel_path VARCHAR(4000)
)";

pub fn migrate(conn: &Connection) -> Result<()> {
    debug!("Trying to find database schema version");
    let schema_version_result = query_schema_version(&conn);
    let schema_version = if let Ok(version) = schema_version_result {
        info!("Found database schema version {}", version);
        version
    } else {
        info!("No schema version found, seems to be a new database");
        0
    };

    if schema_version < 1 {
        info!("Applying schema update to version 1");
        conn.execute(SCHEMA_V1, params![])?;
        upsert_schema_version(conn, 1)?;
    }
    if schema_version < 2 {
        info!("Applying schema update to version 2");
        conn.execute(SCHEMA_V2, params![])?;
        upsert_schema_version(conn, 2)?;
    }

    debug!("Database schema migration finished");
    Ok(())
}

fn query_schema_version(conn: &Connection) -> Result<u8> {
    let result: String = conn.query_row(
        "SELECT v FROM config WHERE k = ?1",
        params![CONFIG_KEY_SCHEMA_VERSION],
        |row| row.get(0),
    )?;

    Ok(result.parse().expect("Schema version not numeric"))
}

fn upsert_schema_version(conn: &Connection, version: u8) -> Result<()> {
    conn.execute(
        "REPLACE INTO config VALUES (?1, ?2)",
        params![CONFIG_KEY_SCHEMA_VERSION, version],
    )
    .expect("Upsert schema version into config table failed");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rusqlite::Connection;

    #[test]
    fn migrations_work_on_empty_database() {
        let conn = Connection::open_in_memory().unwrap();
        let result = migrate(&conn);
        assert!(result.is_ok());
    }

    #[test]
    fn migrations_work_on_already_migrated_database() {
        let conn = Connection::open_in_memory().unwrap();
        let result = migrate(&conn);
        assert!(result.is_ok());

        // Migrate again
        let result = migrate(&conn);
        assert!(result.is_ok());
    }
}
