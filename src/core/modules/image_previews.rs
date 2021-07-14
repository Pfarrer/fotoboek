use crate::db::models::Image;
use crate::db::models::Task;
use crate::db::Database;

static MODULE_ID: &str = "image_previews";

pub async fn create_tasks_on_new_image(db: &Database, image: &Image) -> Result<Vec<Task>, String> {
    let image_id = image.id.expect("Image must have an id");
    let task_medium = db
        .run(move |c| {
            Task {
                id: None,
                image_id,
                module: MODULE_ID.into(),
                action: "medium".into(),
                priority: 100,
                blocked_by_task_id: None,
            }
            .insert(c)
        })
        .await?;

    let task_medium_id = task_medium.id;
    let task_small = db
        .run(move |c| {
            Task {
                id: None,
                image_id,
                module: MODULE_ID.into(),
                action: "small".into(),
                priority: 100,
                blocked_by_task_id: task_medium_id,
            }
            .insert(c)
        })
        .await?;

    Ok(vec![task_medium, task_small])
}
