use diesel::{self, insert_into, prelude::*};
use serde::Serialize;

use crate::db::schema::previews;
use crate::db::schema::previews::dsl::*;

#[derive(Insertable, Queryable, Serialize)]
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
        by_size: String,
    ) -> Result<Preview, String> {
        previews
            .filter(image_id.eq(by_image_id).and(size.eq(by_size)))
            .first::<Preview>(conn)
            .map_err(|err| err.to_string())
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<Preview, String> {
        insert_into(previews)
            .values(self)
            .execute(conn)
            .map_err(|err| err.to_string())?;
        previews
            .order(id.desc())
            .first(conn)
            .map_err(|err| err.to_string())
    }
}
