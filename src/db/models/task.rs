use diesel::{self, insert_into, prelude::*};
use serde::Serialize;

use crate::db::schema::tasks;
use crate::db::schema::tasks::dsl::*;

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub image_id: i32,
    pub module: String,
    pub action: String,
    pub blocked_by_task_id: Option<i32>,
}

impl Task {
    pub fn insert(&self, c: &diesel::SqliteConnection) -> Result<(), ()> {
        insert_into(tasks)
            .values(self)
            .execute(c)
            .map(|_| ())
            .map_err(|_| ())
    }
}
