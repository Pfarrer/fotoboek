use diesel::{self, prelude::*};
use serde::Serialize;

use crate::db::schema::tasks;
use crate::db::schema::tasks::dsl::*;

#[derive(Insertable, Queryable, Serialize, Debug)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub image_id: i32,
    pub module: String,
    pub priority: i32,
}

impl Task {
    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Task> {
        tasks.load::<Task>(conn).expect("Query tasks failed")
    }

    pub fn all_unblocked_ordered_by_priority(conn: &diesel::SqliteConnection) -> Vec<Task> {
        tasks
            .order(priority.asc())
            .load::<Task>(conn)
            .expect("Query tasks failed")
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<Task, String> {
        diesel::insert_into(tasks)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;

        tasks
            .order(id.desc())
            .first(conn)
            .map_err(|err| err.to_string())
    }

    pub fn delete(self, conn: &diesel::SqliteConnection) -> Result<usize, String> {
        diesel::delete(tasks.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| err.to_string())
    }
}
