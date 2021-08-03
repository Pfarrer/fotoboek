use crate::db::models::*;
use crate::db::Database;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::path::Path;

#[get("/?<path>&<deep>")]
pub async fn index(db: Database, path: Option<String>, deep: Option<bool>) -> Option<Template> {
    let parent_dir_path = if let Some(val) = path {
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
    let max_distance = match deep {
        Some(true) => 99999,
        _ => 0,
    };
    let current_directory_name = parent_dir_path.clone().unwrap_or("Start".into());
    let is_root_dir = parent_dir_path.is_none();

    let image_root_string = dotenv::var("IMAGE_ROOT").unwrap();
    let abs_dir_path = parent_dir_path.clone().unwrap_or(image_root_string);
    let image_metadata: Vec<Metadata> = db
        .run(move |conn| Metadata::by_image_path_and_ordered(conn, &abs_dir_path, max_distance))
        .await;

    let parent_dir_path_ref = parent_dir_path.clone();
    let subdir_paths = db
        .run(move |conn| ImagePath::subdirs_of(conn, parent_dir_path_ref.as_deref()))
        .await;

    let sub_dirs: Vec<TmplDirectory> = subdir_paths
        .iter()
        .map(|abs_path| TmplDirectory::new(abs_path, deep.clone()))
        .collect();

    #[derive(Serialize)]
    struct IndexContext<'a> {
        image_metadata: Vec<Metadata>,
        current_directory_name: String,
        is_root_dir: bool,
        sub_dirs: Vec<TmplDirectory<'a>>,
    }

    #[derive(Serialize)]
    struct TmplDirectory<'a> {
        name: &'a str,
        url: String,
    }

    impl<'a> TmplDirectory<'a> {
        fn new(abs_path: &'a String, uri_deep: Option<bool>) -> TmplDirectory<'a> {
            let name = Path::new(abs_path).file_name().unwrap().to_str().unwrap();
            TmplDirectory {
                name,
                url: rocket::uri!(index(Some(abs_path), uri_deep)).to_string(),
            }
        }
    }

    let context = IndexContext {
        image_metadata,
        current_directory_name,
        is_root_dir,
        sub_dirs,
    };

    Some(Template::render("index", context))
}

fn get_requested_path(path: &String) -> Result<String, String> {
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
