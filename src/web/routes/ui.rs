use crate::db::models::*;
use crate::db::Database;
use rocket::http::uri::fmt::FromUriParam;
use rocket::http::uri::fmt::Query;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(FromForm, UriDisplayQuery, Clone, Copy, Serialize)]
pub struct DisplaySettings<'a> {
    pub path: Option<&'a str>,
    pub deep: Option<bool>,
}

impl<'a> FromUriParam<Query, (Option<&'a str>, Option<bool>)> for DisplaySettings<'a> {
    type Target = DisplaySettings<'a>;

    fn from_uri_param((path, deep): (Option<&'a str>, Option<bool>)) -> DisplaySettings {
        DisplaySettings { path, deep }
    }
}

#[get("/gallery?<settings..>")]
pub async fn gallery(settings: DisplaySettings<'_>, db: Database) -> Option<Template> {
    let image_metadata = get_image_metadata_for(&db, &settings).await;
    let image_urls: HashMap<i32, String> = image_metadata
        .iter()
        .map(|m| {
            (
                m.image_id,
                rocket::uri!(image_by_id(m.image_id, settings)).to_string(),
            )
        })
        .into_iter()
        .collect();

    let parent_dir_path = if let Some(val) = settings.path.clone() {
        match get_requested_path(&val) {
            Ok(path) => Some(path),
            Err(err) => {
                warn!("get_requested_path failed: {}", err);
                None
            }
        }
    } else {
        None
    };
    let current_directory_name = parent_dir_path.clone().unwrap_or("Start".into());

    // let image_root_string = dotenv::var("IMAGE_ROOT").unwrap();
    // let abs_dir_path = parent_dir_path.clone().unwrap_or(image_root_string);
    let subdir_paths = db
        .run(move |conn| ImagePath::subdirs_of(conn, parent_dir_path.as_deref()))
        .await;

    let sub_dirs: Vec<TmplDirectory> = subdir_paths
        .iter()
        .map(|abs_path| TmplDirectory::new(&abs_path, &settings))
        .collect();

    #[derive(Serialize)]
    struct GalleryContext<'a> {
        image_metadata: Vec<Metadata>,
        gallery_image_urls: HashMap<i32, String>,
        current_directory_name: String,
        sub_dirs: Vec<TmplDirectory<'a>>,
        is_deep: bool,
        toggle_deep_url: String,
    }

    #[derive(Serialize)]
    struct TmplDirectory<'a> {
        name: &'a str,
        url: String,
    }

    impl<'a> TmplDirectory<'a> {
        fn new(abs_path: &'a String, settings: &DisplaySettings<'_>) -> TmplDirectory<'a> {
            let name = Path::new(abs_path).file_name().unwrap().to_str().unwrap();
            TmplDirectory {
                name,
                url: rocket::uri!(gallery(DisplaySettings {
                    path: Some(abs_path),
                    ..*settings
                }))
                .to_string(),
            }
        }
    }

    let is_deep = settings.deep.unwrap_or(false);
    let toggle_deep_url = rocket::uri!(gallery(DisplaySettings {
        deep: Some(!is_deep),
        ..settings
    }))
    .to_string();
    let context = GalleryContext {
        image_metadata,
        gallery_image_urls: image_urls,
        current_directory_name,
        sub_dirs,
        is_deep,
        toggle_deep_url,
    };

    Some(Template::render("gallery", context))
}

async fn get_image_metadata_for(db: &Database, settings: &DisplaySettings<'_>) -> Vec<Metadata> {
    let parent_dir_path = if let Some(val) = settings.path.clone() {
        match get_requested_path(val) {
            Ok(path) => Some(path),
            Err(err) => {
                warn!("get_requested_path failed: {}", err);
                None
            }
        }
    } else {
        None
    };
    let max_distance = match settings.deep {
        Some(true) => 99999,
        _ => 0,
    };
    let image_root_string = dotenv::var("IMAGE_ROOT").unwrap();
    let abs_dir_path = parent_dir_path.clone().unwrap_or(image_root_string);

    db.run(move |conn| Metadata::by_image_path_and_ordered(conn, &abs_dir_path, max_distance))
        .await
}

fn get_requested_path(path: &'_ str) -> Result<String, String> {
    let image_root_string = dotenv::var("IMAGE_ROOT").unwrap();
    let root_path = Path::new(&image_root_string);
    let requested_path: String = match root_path.join(&path).canonicalize() {
        Ok(path_buf) => path_buf.to_str().expect("Path not a string").to_string(),
        Err(err) => {
            return Err(format!(
                "Requested path is invalid {:?}: {}",
                root_path.join(&path).to_str(),
                err
            ));
        }
    };

    if !requested_path.starts_with(&image_root_string) {
        return Err(format!(
            "Requested path {:?} does not start with IMAGE_ROOT {}",
            requested_path, image_root_string
        ));
    }

    Ok(requested_path)
}

#[get("/gallery/<id>?<settings..>")]
pub async fn image_by_id(id: i32, settings: DisplaySettings<'_>, db: Database) -> Option<Template> {
    let gallery_metadata = get_image_metadata_for(&db, &settings).await;
    let (prev_image_metadata, this_image_metadata, next_image_metadata) =
        find_image_metadata_and_neighbors(&gallery_metadata, id)?;

    #[derive(Serialize)]
    struct Context<'a> {
        image_metadata: &'a Metadata,
        prev_image_url: Option<String>,
        next_image_url: Option<String>,
        back_to_gallery_url: String,
    }

    let back_to_gallery_url = rocket::uri!(gallery(settings)).to_string();
    let context = Context {
        image_metadata: this_image_metadata,
        prev_image_url: prev_image_metadata
            .map(|m| rocket::uri!(image_by_id(m.image_id, settings)).to_string())
            .or(Some(back_to_gallery_url.clone())),
        next_image_url: next_image_metadata
            .map(|m| rocket::uri!(image_by_id(m.image_id, settings)).to_string())
            .or(Some(back_to_gallery_url.clone())),
        back_to_gallery_url,
    };

    Some(Template::render("image", context))
}

fn find_image_metadata_and_neighbors<'a>(
    metadata: &'a Vec<Metadata>,
    image_id: i32,
) -> Option<(Option<&'a Metadata>, &'a Metadata, Option<&'a Metadata>)> {
    let index = metadata.iter().position(|m| m.image_id == image_id)?;

    let prev = if index > 0 {
        metadata.get(index - 1)
    } else {
        None
    };

    Some((prev, metadata.get(index).unwrap(), metadata.get(index + 1)))
}
