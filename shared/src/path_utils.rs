use std::path::{Path, PathBuf};
use crate::models::FotoboekConfig;

pub fn abs_pathbuf_to_rel<'a, 'b>(config: &'a FotoboekConfig, abs_path: &'b PathBuf) -> &'b str {
    abs_to_rel(config, abs_path.to_str().unwrap())
}

pub fn abs_to_rel<'a, 'b>(config: &'a FotoboekConfig, abs_path: &'b str) -> &'b str {
    let rel_path = Path::new(abs_path).strip_prefix(config.media_source_path.clone()).unwrap();
    rel_path.to_str().unwrap()
}

pub fn rel_to_abs(config: &FotoboekConfig, rel_path: &str) -> String {
    let abs_path = Path::new(&config.media_source_path).join(rel_path);
    abs_path.to_str().unwrap().to_string()
}

pub fn get_filename(path: &PathBuf) -> String {
    path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::path_utils::FotoboekConfig;

    pub fn make_config() -> FotoboekConfig {
        FotoboekConfig {
            media_source_path: "/mnt/images".to_string(),
            file_storage_path: "".to_string(),
            webapp_files_path: "".to_string(),
            num_worker_threads: 1,
            task_lock_timeout_sec: 1,
        }
    }

    #[test]
    fn abs_to_rel() {
        let config = make_config();
        assert_eq!(super::abs_to_rel(&config, "/mnt/images/dir/image.jpg"), "dir/image.jpg");
        assert_eq!(super::abs_to_rel(&config, "/mnt/images/image.jpg"), "image.jpg");
    }

    #[test]
    fn rel_to_abs() {
        let config = make_config();
        assert_eq!(super::rel_to_abs(&config, "dir/image.jpg"), "/mnt/images/dir/image.jpg");
        assert_eq!(super::rel_to_abs(&config, "image.jpg"), "/mnt/images/image.jpg");
    }
}
