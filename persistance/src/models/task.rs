use diesel::{self, prelude::*};
use log::debug;
use serde::Serialize;

use shared::models::FotoboekConfig;

use crate::schema::tasks;
use crate::schema::tasks::dsl;
use crate::FotoboekDatabase;

#[derive(Insertable, Queryable, Clone, Serialize, Debug)]
pub struct Task {
    pub id: Option<i32>,
    pub file_id: i32,
    pub module: String,
    pub priority: i32,
    pub work_started_at: chrono::NaiveDateTime,
    pub max_worker_id: i32,
}

impl Task {
    pub async fn all(db: &FotoboekDatabase) -> Vec<Task> {
        db.run(move |conn| dsl::tasks.load::<Task>(conn).expect("Query tasks failed"))
            .await
    }

    pub async fn next_workable_by_priority_and_lock(
        db: &FotoboekDatabase,
        config: &FotoboekConfig,
        worker_id: usize,
    ) -> Option<Task> {
        let dt_now = chrono::Utc::now().naive_utc();
        let dt_one_hour_ago = chrono::NaiveDateTime::from_timestamp(
            dt_now.timestamp() - config.task_lock_timeout_sec as i64,
            0,
        );

        db.run(move |conn| loop {
            let workable_tasks: Vec<Task> = dsl::tasks
                .filter(
                    dsl::work_started_at
                        .le(dt_one_hour_ago)
                        .and(dsl::max_worker_id.ge(worker_id as i32)),
                )
                .order(dsl::priority.asc())
                .limit(1)
                .load::<Task>(conn)
                .expect("Query tasks failed");

            if workable_tasks.len() == 1 {
                let task = workable_tasks.get(0).unwrap();
                let success = diesel::update(dsl::tasks)
                    .set(dsl::work_started_at.eq(dt_now))
                    .filter(
                        dsl::id
                            .eq(task.id)
                            .and(dsl::work_started_at.eq(task.work_started_at)),
                    )
                    .execute(conn)
                    .expect("Lock task update failed")
                    == 1;
                if success {
                    return Some(task.to_owned());
                } else {
                    debug!(
                        "Task {} locked by now, looking fo the next...",
                        task.id.unwrap()
                    );
                }
            } else {
                return None;
            }
        })
        .await
    }

    pub async fn insert(self, db: &FotoboekDatabase) -> Result<(), String> {
        db.run(move |conn| {
            diesel::insert_into(dsl::tasks)
                .values(&self)
                .execute(conn)
                .map_err(|err| err.to_string())?;
            Ok(())
        })
        .await
    }

    pub async fn delete(self, db: &FotoboekDatabase) -> Result<usize, String> {
        db.run(move |conn| {
            diesel::delete(dsl::tasks.filter(dsl::id.eq(self.id)))
                .execute(conn)
                .map_err(|err| err.to_string())
        })
        .await
    }
}
