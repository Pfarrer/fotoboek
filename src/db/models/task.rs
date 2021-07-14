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
    pub priority: i32,
    pub blocked_by_task_id: Option<i32>,
}

impl Task {
    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Task> {
        tasks.load::<Task>(conn).expect("Query all task")
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<Task, String> {
        insert_into(tasks)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;

        tasks
            .order(id.desc())
            .first(conn)
            .map_err(|err| err.to_string())
    }
}
