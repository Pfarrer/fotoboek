use std::fs;
use std::fs::File;
use std::io::Write;

use shared::models::{FotoboekConfig, PreviewSize};

pub fn init(config: &FotoboekConfig) {
    fs::create_dir_all(preview_base_dir_path(&config)).unwrap();
    fs::create_dir_all(video_base_dir_path(&config)).unwrap();
}

pub fn store_preview(
    config: &FotoboekConfig,
    file_hash: &String,
    preview_size: &PreviewSize,
    preview_bytes: &Vec<u8>,
) -> Result<(), String> {
    assert_eq!(file_hash.len(), 64);

    // Make sure, the directory exists
    fs::create_dir_all(preview_dir_path(config, file_hash)).unwrap();

    let file_path = file_preview_path(config, file_hash, preview_size);
    let mut file = File::create(file_path).map_err(|err| err.to_string())?;
    file.write_all(preview_bytes)
        .map_err(|err| err.to_string())?;
    file.flush().map_err(|err| err.to_string())?;

    Ok(())
}

/// Returns the path to the base folder that contains all preview images.
fn preview_base_dir_path(config: &FotoboekConfig) -> String {
    format!("{}/previews", config.file_storage_path)
}

/// Returns the path to the folder that contains preview images for the given arguments.
fn preview_dir_path(config: &FotoboekConfig, file_hash: &String) -> String {
    let hash_prefix = &file_hash[0..2];
    format!("{}/{}", preview_base_dir_path(config), hash_prefix)
}

/// Returns the path to the base folder that contains all transcoded videos.
fn video_base_dir_path(config: &FotoboekConfig) -> String {
    format!("{}/videos", config.file_storage_path)
}

/// Returns the path to the folder that contains preview images for the given arguments.
pub fn video_dir_path(config: &FotoboekConfig, file_hash: &String) -> String {
    let hash_prefix = &file_hash[0..2];
    format!("{}/{}", video_base_dir_path(config), hash_prefix)
}

/// Returns the path to the preview file for the given arguments.
pub fn file_preview_path(
    config: &FotoboekConfig,
    file_hash: &String,
    preview_size: &PreviewSize,
) -> String {
    let size_prefix = match preview_size {
        PreviewSize::Small => "small",
        PreviewSize::Large => "large",
    };
    format!(
        "{}/{}-{}.webp",
        preview_dir_path(config, file_hash),
        size_prefix,
        file_hash
    )
}

pub fn video_path(config: &FotoboekConfig, file_hash: &String) -> String {
    format!("{}/{}.webm", video_dir_path(config, file_hash), file_hash)
}
