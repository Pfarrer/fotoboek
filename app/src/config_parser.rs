use shared::models::FotoboekConfig;

pub fn parse() -> FotoboekConfig {
    FotoboekConfig {
        media_source_path: get_string_env_value("MEDIA_SOURCE_PATH"),
        file_storage_path: get_string_env_value("FILE_STORAGE_PATH"),
        webapp_files_path: get_string_env_value("WEBAPP_FILES_PATH"),
        num_worker_threads: get_usize_env_value("NUM_WORKER_THREADS"),
        task_lock_timeout_sec: get_usize_env_value("TASK_LOCK_TIMEOUT_SEC"),
    }
}

fn get_string_env_value(name: &str) -> String {
    dotenv::var(name)
        .expect(format!("Environment \"{}\" property not found", name).as_str())
        .parse()
        .expect(format!("Environment \"{}\" property has invalid value", name).as_str())
}

fn get_usize_env_value(name: &str) -> usize {
    dotenv::var(name)
        .expect(format!("Environment \"{}\" property not found", name).as_str())
        .parse()
        .expect(format!("Environment \"{}\" property has invalid value", name).as_str())
}
