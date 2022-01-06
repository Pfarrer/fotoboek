use persistance::models::Task;
use persistance::FotoboekDatabase;
use tokio::task;
use tokio::time::{sleep, Duration};
use log::{debug, error};
use shared::models::FotoboekConfig;

pub fn spawn(db: FotoboekDatabase, config: &FotoboekConfig, worker_id: usize) {
    let config_copy = config.clone();
    task::spawn(async move {
        loop {
            let task_option = Task::next_workable_by_priority_and_lock(&db, &config_copy).await;

            if let Some(task) = task_option {
                debug!(
                    "Worker {} locked task {:?} and starts working",
                    worker_id, task
                );
                run_task(&db, &config_copy, task).await;
            } else {
                debug!("Worker {} has no workable tasks, going to sleep", worker_id);
                sleep(Duration::from_secs(60)).await;
            }
        }
    });
}

async fn run_task(db: &FotoboekDatabase, config: &FotoboekConfig, task: Task) {
    match crate::modules::run_task(db, config, &task).await {
        Ok(_) => {
            let task_id = task.id;
            let delete_result = task.delete(db).await;
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
