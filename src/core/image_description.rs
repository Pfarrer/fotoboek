use crate::db::models::*;
use crate::db::Database;
use std::fs;

pub async fn get_for_image_id(id: i32, db: &Database) -> Option<String> {
    let image = db
        .run(move |conn| Image::by_id(conn, id).expect("Image not found"))
        .await;
    let description_path = description_file_path(&image);

    fs::read_to_string(description_path).ok()
}

pub async fn set_for_image_id(id: i32, description: &String, db: &Database) -> Result<(), String> {
    let image = db
        .run(move |conn| Image::by_id(conn, id).expect("Image not found"))
        .await;
    let description_path = description_file_path(&image);

    fs::write(description_path, description).map_err(|err| err.to_string())
}

fn description_file_path(image: &Image) -> String {
    format!("{}.description.txt", image.abs_path)
}
