use strum_macros::{EnumString, ToString};

#[derive(Clone, Debug)]
pub struct FotoboekConfig {
    pub media_source_path: String,
    pub file_storage_path: String,
    pub webapp_files_path: String,
    pub num_worker_threads: usize,
    pub task_lock_timeout_sec: usize,
}

#[derive(PartialEq, EnumString, ToString)]
pub enum PreviewSize {
    #[strum(serialize = "large")]
    Large,
    #[strum(serialize = "small")]
    Small,
}
