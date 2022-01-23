use std::iter::Iterator;
use std::path::PathBuf;

use futures::future;
use glob::{glob_with, MatchOptions};
use log::warn;
use persistance::models::File;
use persistance::FotoboekDatabase;
use shared::models::FotoboekConfig;
use shared::path_utils::{abs_pathbuf_to_rel, get_filename};

pub struct SearchAndUpdateResult {
    pub total_count: usize,
    pub added_count: usize,
    pub removed_count: usize,
}

pub async fn search_and_update_db(
    db: &FotoboekDatabase,
    config: &FotoboekConfig,
) -> SearchAndUpdateResult {
    let source_paths = search_fs(config);
    let add_futures = source_paths
        .iter()
        .map(|source_path| try_add_image(&db, config, source_path))
        .collect::<Vec<_>>();
    let add_results = future::join_all(add_futures).await;
    let (ok_results, err_results): (Vec<_>, Vec<_>) = add_results.iter().partition(|r| r.is_ok());
    err_results
        .iter()
        .for_each(|r| warn!("Failed to add file: {:?}", r));

    SearchAndUpdateResult {
        total_count: source_paths.len(),
        added_count: ok_results.len(),
        removed_count: 0,
    }
}

pub async fn try_add_image(
    db: &FotoboekDatabase,
    config: &FotoboekConfig,
    source_path: &PathBuf,
) -> Result<(), String> {
    let file = File {
        id: None,
        file_type: "IMAGE".into(),
        file_name: get_filename(source_path),
        rel_path: abs_pathbuf_to_rel(config, source_path).into(),
    };
    if let Some(file) = file.insert(db).await? {
        crate::modules::create_tasks_on_new_file(db, &file)
            .await
            .expect("Core modules create_tasks_on_new_file failed");
        Ok(())
    } else {
        Err("File already registered".into())
    }
}

fn search_fs(cfg: &FotoboekConfig) -> Vec<PathBuf> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    glob_with(&format!("{}/**/*.jpg", cfg.media_source_path), options)
        .unwrap()
        .chain(glob_with(&format!("{}/**/*.jpeg", cfg.media_source_path), options).unwrap())
        .filter_map(|entry| entry.ok().map(|path| path.canonicalize().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempdir::TempDir;

    static NO_IMAGES_DIRNAME: &str = "no_images";
    static TEST_IMAGES_DIRNAME: &str = "test_images";

    pub fn setup_temp_dir() -> TempDir {
        let temp_dir = TempDir::new("source_images_unittest").unwrap();

        std::fs::create_dir(temp_dir.path().join(NO_IMAGES_DIRNAME)).unwrap();

        let images_dir_path = temp_dir.path().join(TEST_IMAGES_DIRNAME);
        std::fs::create_dir(&images_dir_path).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image1.jpg")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image2.JPG")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image3.jpeg")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image4.JPEG")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("other-file.txt")).unwrap();

        temp_dir
    }

    #[test]
    fn no_images_found_in_empty_dir() {
        let temp_dir = setup_temp_dir();
        let root = String::from(temp_dir.path().join(NO_IMAGES_DIRNAME).to_str().unwrap());
        let config = FotoboekConfig {
            media_source_path: root,
            file_storage_path: "".to_string(),
            webapp_files_path: "".to_string(),
            num_worker_threads: 1,
            task_lock_timeout_sec: 1,
        };

        let source_images = search_fs(&config);
        assert_eq!(source_images.len(), 0);
    }

    #[test]
    fn test_images_found_in_dir() {
        let temp_dir = setup_temp_dir();
        let root = String::from(temp_dir.path().join(TEST_IMAGES_DIRNAME).to_str().unwrap());
        let config = FotoboekConfig {
            media_source_path: root,
            file_storage_path: "".to_string(),
            webapp_files_path: "".to_string(),
            num_worker_threads: 1,
            task_lock_timeout_sec: 1,
        };

        let source_images = search_fs(&config);
        assert_eq!(source_images.len(), 4);
    }

    #[test]
    fn images_are_searched_recursively() {
        let temp_dir = setup_temp_dir();
        let root = String::from(temp_dir.path().to_str().unwrap());
        let config = FotoboekConfig {
            media_source_path: root,
            file_storage_path: "".to_string(),
            webapp_files_path: "".to_string(),
            num_worker_threads: 1,
            task_lock_timeout_sec: 1,
        };

        let source_images = search_fs(&config);
        assert_eq!(source_images.len(), 4);
    }
}

/*

    static NO_IMAGES_DIRNAME: &str = "no_images";
    static TEST_IMAGES_DIRNAME: &str = "test_images";

    pub fn setup(source_sub_path: &str) -> FotoboekConfig {
        let temp_dir = TempDir::new("source_images_unittest").unwrap();

        std::fs::create_dir(temp_dir.path().join(NO_IMAGES_DIRNAME)).unwrap();

        let images_dir_path = temp_dir.path().join(TEST_IMAGES_DIRNAME);
        std::fs::create_dir(&images_dir_path).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image1.jpg")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image2.JPG")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image3.jpeg")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image4.JPEG")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("other-file.txt")).unwrap();

        let media_source_path = String::from(temp_dir.path().join(source_sub_path).to_str().unwrap());
        FotoboekConfig {
            media_source_path,
            file_storage_path: "".to_string(),
            num_worker_threads: 1,
            task_lock_timeout_sec: 1,
        }
    }

    #[test]
    fn no_images_found_in_empty_dir() {
        let config = setup(NO_IMAGES_DIRNAME);
        let source_images = search_fs(&config);
        assert_eq!(source_images.len(), 0);
    }

    #[test]
    fn test_images_found_in_dir() {
        let config = setup(TEST_IMAGES_DIRNAME);
        let source_images = search_fs(&config);
        assert_eq!(source_images.len(), 4);
    }

    #[test]
    fn images_are_searched_recursively() {
        let config = setup("");
        let source_images = search_fs(&config);
        assert_eq!(source_images.len(), 4);
    }
}

*/
