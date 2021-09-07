use crate::core::utils::abs_to_rel_path;
use crate::db::models::*;
use crate::db::Database;
use rocket::http::uri::fmt::FromUriParam;
use rocket::http::uri::fmt::Query;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

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

    let abs_dir_path = get_requested_path(&settings).map_or_else(
        |error| {
            warn!("Invalid path requested: {}", error);
            None
        },
        |path| Some(path),
    )?;
    let subdir_paths = db
        .run(move |conn| ImagePath::subdirs_of(conn, &&abs_dir_path))
        .await;

    let sub_dirs: Vec<TmplDirectory> = subdir_paths
        .iter()
        .map(|abs_path| abs_to_rel_path(abs_path))
        .map(|rel_path| TmplDirectory::new(rel_path.to_string(), &settings))
        .collect();

    let parent_dirs = get_parent_dirs(&settings);

    let is_deep = settings.deep.unwrap_or(false);
    let toggle_deep_url = rocket::uri!(gallery(DisplaySettings {
        deep: Some(!is_deep),
        ..settings
    }))
    .to_string();

    let context = GalleryContext {
        image_metadata,
        gallery_image_urls: image_urls,
        sub_dirs,
        parent_dirs,
        is_deep,
        toggle_deep_url,
    };

    Some(Template::render("gallery", context))
}

#[derive(Serialize)]
struct GalleryContext {
    image_metadata: Vec<Metadata>,
    gallery_image_urls: HashMap<i32, String>,
    sub_dirs: Vec<TmplDirectory>,
    parent_dirs: Vec<TmplDirectory>,
    is_deep: bool,
    toggle_deep_url: String,
}

#[derive(Serialize)]
struct TmplDirectory {
    name: String,
    url: String,
}

impl<'a> TmplDirectory {
    fn new(abs_path: String, settings: &DisplaySettings<'_>) -> TmplDirectory {
        let name = Path::new(&&abs_path)
            .file_name()
            .map(|os_str| os_str.to_str())
            .flatten()
            .unwrap_or("Start")
            .to_owned();
        TmplDirectory {
            name,
            url: rocket::uri!(gallery(DisplaySettings {
                path: Some(&&abs_path),
                ..*settings
            }))
            .to_string(),
        }
    }
}

async fn get_image_metadata_for(db: &Database, settings: &DisplaySettings<'_>) -> Vec<Metadata> {
    let abs_dir_path = get_requested_path(settings).map_or_else(
        |error| {
            warn!("Invalid path requested: {}", error);
            dotenv::var("IMAGE_ROOT").unwrap()
        },
        |path| path,
    );
    let max_distance = match settings.deep {
        Some(true) => 99999,
        _ => 0,
    };

    db.run(move |conn| Metadata::by_image_path_and_ordered(conn, &abs_dir_path, max_distance))
        .await
}

fn get_requested_path(settings: &DisplaySettings) -> Result<String, String> {
    let image_root_string = dotenv::var("IMAGE_ROOT").unwrap();
    let root_path = Path::new(&image_root_string);

    let requested_rel_path = settings.path.as_ref().unwrap_or(&"");

    let requested_abs_path: String = match root_path.join(&requested_rel_path).canonicalize() {
        Ok(path_buf) => path_buf.to_str().expect("Path not a string").to_string(),
        Err(err) => {
            return Err(format!(
                "Requested path is invalid {:?}: {}",
                root_path.join(&requested_rel_path).to_str(),
                err
            ));
        }
    };

    if !requested_abs_path.starts_with(&image_root_string) {
        return Err(format!(
            "Requested abs path {:?} does not start with IMAGE_ROOT {}",
            requested_abs_path, image_root_string
        ));
    }

    Ok(requested_abs_path)
}

fn get_parent_dirs(settings: &DisplaySettings) -> Vec<TmplDirectory> {
    let path_sep: String = String::from(std::path::MAIN_SEPARATOR);

    let mut dirs = vec![TmplDirectory {
        name: "Start".to_string(),
        url: rocket::uri!(gallery(DisplaySettings {
            path: Some(""),
            ..*settings
        }))
        .to_string(),
    }];

    let path_elements = settings
        .path
        .map(|p| {
            PathBuf::from(p)
                .iter()
                .map(|os_str| os_str.to_str().unwrap().to_string())
                .collect()
        })
        .unwrap_or(vec![]);

    let mut previous_path_elements = vec![];
    for path in path_elements {
        previous_path_elements.push(path);

        let dir = TmplDirectory::new(previous_path_elements.join(&path_sep), &settings);
        dirs.push(dir);
    }

    dirs
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn abs_to_rel_path_single_subdir() {
        let image_root = "/my/image/root";
        env::set_var("IMAGE_ROOT", &image_root);

        assert_eq!(
            "dir",
            abs_to_rel_path(&format!("{}/dir", image_root).to_string())
        );
    }

    #[test]
    fn abs_to_rel_path_single_multi_subdirs() {
        let image_root = "/my/image/root";
        env::set_var("IMAGE_ROOT", &image_root);

        assert_eq!(
            "multi/sub/dir",
            abs_to_rel_path(&format!("{}/multi/sub/dir/", image_root).to_string())
        );
    }

    #[test]
    fn abs_to_rel_path_no_subdir() {
        let image_root = "/my/image/root";
        env::set_var("IMAGE_ROOT", &image_root);

        assert_eq!("", abs_to_rel_path(&format!("{}/", image_root).to_string()));
        assert_eq!("", abs_to_rel_path(&image_root.to_string()));
    }
}
