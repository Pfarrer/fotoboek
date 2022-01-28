use diesel::{self, insert_into, prelude::*};
use serde::Serialize;

use crate::schema::files;
use crate::schema::files::dsl;
use crate::FotoboekDatabase;

#[derive(Insertable, Queryable, Serialize)]
pub struct File {
    pub id: Option<i32>,
    pub rel_path: String,
    pub file_type: String,
    pub file_name: String,
}

impl File {
    pub async fn all(db: FotoboekDatabase) -> Vec<File> {
        db.run(move |conn| dsl::files.load(conn).expect("Load all files failed"))
            .await
    }

    pub async fn by_id(db: &FotoboekDatabase, file_id: i32) -> Result<File, String> {
        db.run(move |conn| {
            dsl::files
                .filter(dsl::id.eq(file_id))
                .first::<File>(conn)
                .map_err(|err| err.to_string())
        })
        .await
    }

    pub async fn insert(self, db: &FotoboekDatabase) -> Result<Option<File>, String> {
        db.run(move |conn| {
            insert_into(dsl::files)
                .values(&self)
                .execute(conn)
                .map_err(|err| err.to_string())?;
            let file = dsl::files
                .filter(dsl::rel_path.eq(self.rel_path))
                .limit(1)
                .load(conn)
                .map_err(|err| err.to_string())?
                .into_iter()
                .next();

            Ok(file)
        })
        .await
    }
}
