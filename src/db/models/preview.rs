// use diesel::sqlite::upsert::*;
use diesel::{
    self,
    prelude::*,
    result::{DatabaseErrorKind, Error},
};
use serde::Serialize;

use crate::db::schema::previews;
use crate::db::schema::previews::dsl;

#[derive(Insertable, AsChangeset, Queryable, Serialize)]
#[table_name = "previews"]
pub struct Preview {
    pub id: Option<i32>,
    pub image_id: i32,
    pub size: String,
    pub data: Vec<u8>,
}

impl Preview {
    pub fn by_image_id_and_size(
        conn: &diesel::SqliteConnection,
        by_image_id: i32,
        by_size: &str,
    ) -> Result<Preview, String> {
        dsl::previews
            .filter(dsl::image_id.eq(by_image_id).and(dsl::size.eq(by_size)))
            .first::<Preview>(conn)
            .map_err(|err| err.to_string())
    }

    pub fn save(self, conn: &diesel::SqliteConnection) -> Result<Preview, String> {
        conn.immediate_transaction(|| {
            if self.id.is_none() {
                self.insert(conn)
            } else {
                self.update(conn).map(|_| self)
            }
        })
        .map_err(|err| err.to_string())
    }
    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<Preview, Error> {
        let result = diesel::insert_into(dsl::previews)
            .values(&self)
            .execute(conn);

        if let Err(error) = result {
            if let Error::DatabaseError(db_err, _) = error {
                match db_err {
                    DatabaseErrorKind::UniqueViolation => {
                        // Upsert
                        let old_preview = Preview::by_image_id_and_size(conn, self.image_id, &self.size).expect("Find preview to upsert failed");
                        let upsert_preview = Preview {
                            id: old_preview.id,
                            ..self
                        };
                        upsert_preview.update(conn)?;
                        Ok(upsert_preview)
                    },
                    _ => Err(error),
                }
            } else {
                Err(error)
            }
        } else {
            // TODO select only the inserted ID instead of the full row including image blob
            dsl::previews.order(dsl::id.desc()).first(conn)
        }
    }
    pub fn update(&self, conn: &diesel::SqliteConnection) -> Result<usize, Error> {
        diesel::update(dsl::previews.filter(dsl::id.eq(self.id)))
            .set(self)
            .execute(conn)
    }
}
