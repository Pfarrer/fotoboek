use crate::db::models::Task;
use crate::db::Database;
use rocket::tokio::time::{sleep, Duration};

pub fn spawn(db: Database, worker_id: usize) {
    rocket::tokio::task::spawn(async move {
        loop {
            let task_option = db
                .run(|conn| Task::next_workable_by_priority_and_lock(conn))
                .await;

            if let Some(task) = task_option {
                debug!(
                    "Worker {} locked task {:?} and starts working",
                    worker_id, task
                );
                db.run(|conn| run_task(conn, task)).await;
            } else {
                debug!("Worker {} has no workable tasks, going to sleep", worker_id);
                sleep(Duration::from_secs(60)).await;
            }
        }
    });
}

fn run_task(conn: &diesel::SqliteConnection, task: Task) {
    let result = crate::core::modules::run_task(conn, &task);
    match result {
        Ok(_) => {
            let task_id = task.id;
            let delete_result = task.delete(conn);
            if let Err(err) = delete_result {
                error!(
                    "Deleting finished task failed, task id: {:?}, error: {}",
                    task_id, err
                );
            }
            ()
        }
        Err(err) => error!("Running task {:?} failed with error: {}", task, err),
    }
}
