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
        .map(|source_path| try_add_video(&db, config, source_path))
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

pub async fn try_add_video(
    db: &FotoboekDatabase,
    config: &FotoboekConfig,
    source_path: &PathBuf,
) -> Result<(), String> {
    let file = File {
        id: None,
        file_type: "VIDEO".into(),
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
    glob_with(&format!("{}/**/*.mp4", cfg.media_source_path), options)
        .unwrap()
        .filter_map(|entry| entry.ok().map(|path| path.canonicalize().unwrap()))
        .collect()
}
