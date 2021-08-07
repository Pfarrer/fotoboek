use diesel::{self, prelude::*};
use serde::Serialize;

use crate::db::schema::tasks;
use crate::db::schema::tasks::dsl::*;

#[derive(Insertable, Queryable, QueryableByName, Clone, Serialize, Debug)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub image_id: i32,
    pub module: String,
    pub priority: i32,
    pub work_started_at: Option<chrono::NaiveDateTime>,
}

impl Task {
    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Task> {
        tasks.load::<Task>(conn).expect("Query tasks failed")
    }

    pub fn next_workable_by_priority_and_lock(conn: &diesel::SqliteConnection) -> Option<Task> {
        let dt_now = chrono::Utc::now().naive_utc();
        let dt_one_hour_ago =
            chrono::NaiveDateTime::from_timestamp(dt_now.timestamp() - 60 * 60, 0);

        let workable_tasks = diesel::sql_query(
            r#"
            UPDATE tasks
            SET work_started_at = ?
            WHERE
                work_started_at IS NULL
                OR work_started_at < ?
            ORDER BY priority ASC
            LIMIT 1
            RETURNING *
            "#,
        )
        .bind::<diesel::sql_types::Timestamp, _>(dt_now)
        .bind::<diesel::sql_types::Timestamp, _>(dt_one_hour_ago)
        .get_results::<Task>(conn)
        .expect("Update + Returning workable task failed");

        workable_tasks.first().cloned()
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
