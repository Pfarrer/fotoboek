use diesel::{self, prelude::*};
use serde::Serialize;

use crate::db::schema::tasks;
use crate::db::schema::tasks::dsl;

#[derive(Insertable, Queryable, Clone, Serialize, Debug)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub image_id: i32,
    pub module: String,
    pub priority: i32,
    pub work_started_at: chrono::NaiveDateTime,
}

impl Task {
    pub fn new(image_id: i32, module: String, priority: i32) -> Task {
        Task {
            id: None,
            image_id,
            module,
            priority,
            work_started_at: chrono::NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn all(conn: &diesel::SqliteConnection) -> Vec<Task> {
        dsl::tasks.load::<Task>(conn).expect("Query tasks failed")
    }

    pub fn next_workable_by_priority_and_lock(conn: &diesel::SqliteConnection) -> Option<Task> {
        let dt_now = chrono::Utc::now().naive_utc();
        let dt_one_hour_ago =
            chrono::NaiveDateTime::from_timestamp(dt_now.timestamp() - 60 * 60, 0);

        loop {
            let workable_tasks: Vec<Task> = dsl::tasks
                .filter(dsl::work_started_at.le(dt_one_hour_ago))
                .order(dsl::priority.asc())
                .limit(1)
                .load::<Task>(conn)
                .expect("Query tasks failed");
            
            if workable_tasks.len() == 1 {
                let task = workable_tasks.get(0).unwrap();
                let success = diesel::update(dsl::tasks)
                    .set(dsl::work_started_at.eq(dt_now))
                    .filter(dsl::id.eq(task.id).and(dsl::work_started_at.eq(task.work_started_at)))
                    .execute(conn)
                    .expect("Lock task update failed") == 1;
                if success {
                    return Some(task.to_owned())
                } else {
                    debug!("Task {} locked by now, looking fo the next...", task.id.unwrap());
                }
            } else {
                return None
            }
        }
    }

    pub fn insert(self, conn: &diesel::SqliteConnection) -> Result<Task, String> {
        diesel::insert_into(dsl::tasks)
            .values(&self)
            .execute(conn)
            .map_err(|err| err.to_string())?;

        dsl::tasks
            .order(dsl::id.desc())
            .first(conn)
            .map_err(|err| err.to_string())
    }

    pub fn delete(self, conn: &diesel::SqliteConnection) -> Result<usize, String> {
        diesel::delete(dsl::tasks.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map_err(|err| err.to_string())
    }
}
