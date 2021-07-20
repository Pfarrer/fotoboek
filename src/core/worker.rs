use crate::db::models::Task;
use crate::db::Database;

pub fn spawn(db: Database) {
    rocket::tokio::task::spawn(async move {
        let tasks = db
            .run(|conn| Task::all_unblocked_ordered_by_priority(conn))
            .await;
        info!("Found {} unblocked tasks", tasks.len());

        db.run(|conn| run_tasks(conn, tasks)).await;
    });
}

fn run_tasks(conn: &diesel::SqliteConnection, tasks: Vec<Task>) {
    for task in tasks {
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
}
