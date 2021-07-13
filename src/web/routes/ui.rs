use crate::db::Database;
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::db::models::Image;

#[get("/")]
pub async fn index(db: Database) -> Template {
    let images = db.run(|c| Image::all(&c)).await;

    #[derive(Serialize)]
    struct IndexContext {
        image_count: usize,
        images: Vec<Image>,
    }

    let context = IndexContext {
        image_count: images.len(),
        images,
    };

    Template::render("index", context)
}
