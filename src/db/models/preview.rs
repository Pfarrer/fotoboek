use diesel::{self, prelude::*};
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
            diesel::replace_into(dsl::previews)
                .values(&self)
                .execute(conn)?;

            Ok(self)
        })
        .map_err(|err: diesel::result::Error| err.to_string())
    }
}
