use log::debug;
use persistance::models::{File, FileMetadata, Task};
use persistance::{fs, FotoboekDatabase};
use shared::models::FotoboekConfig;
use shared::path_utils::rel_to_abs;
use std::process::Command;
use std::str::from_utf8;

pub const MODULE_ID: &str = "transcode";

pub async fn create_tasks_on_new_file(db: &FotoboekDatabase, file: &File) -> Result<(), String> {
    // Only videos can be transcoded
    if file.file_type == "VIDEO" {
        Task {
            id: None,
            file_id: file.id.unwrap(),
            module: MODULE_ID.into(),
            priority: 300,
            max_worker_id: 0,
            work_started_at: chrono::NaiveDateTime::from_timestamp(0, 0),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}

pub async fn run_task(
    db: &FotoboekDatabase,
    config: &FotoboekConfig,
    task: &Task,
) -> Result<(), String> {
    let metadata = FileMetadata::by_file_id(db, task.file_id).await.ok_or(
        "File metadata not found, very likely because the metadata task did not finish yet"
            .to_string(),
    )?;
    let file = File::by_id(db, task.file_id).await?;
    let abs_source_path = rel_to_abs(config, &file.rel_path);
    let abs_target_path = fs::video_path(config, &metadata.file_hash);

    // Make sure, the directory exists
    std::fs::create_dir_all(fs::video_dir_path(config, &metadata.file_hash)).unwrap();

    execute_command(abs_source_path, abs_target_path, config.num_worker_threads)
}

fn execute_command(source_path: String, target_path: String, threads: usize) -> Result<(), String> {
    // Recommodations from http://wiki.webmproject.org/ffmpeg/vp9-encoding-guide

    debug!("Starting transcode video {}, pass 1...", source_path);
    let output = Command::new("ffmpeg")
        .args(vec![
            "-i",
            source_path.as_str(),
            "-c:v",
            "libvpx-vp9",
            "-pass",
            "1",
            "-b:v",
            "1000K",
            "-threads",
            format!("{}", threads).as_str(),
            "-speed",
            "4",
            "-tile-columns",
            "6",
            "-frame-parallel",
            "1",
            "-an",
            "-f",
            "webm",
            "-y",
            "/dev/null",
        ])
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(format!("Pass 1 failed: ExitStatus: {}", output.status));
    }

    debug!("Transcode pass 1 done, starting with pass 2...");
    let output = Command::new("ffmpeg")
        .args(vec![
            "-i",
            source_path.as_str(),
            "-c:v",
            "libvpx-vp9",
            "-pass",
            "2",
            "-b:v",
            "1000K",
            "-threads",
            format!("{}", threads).as_str(),
            "-speed",
            "1",
            "-tile-columns",
            "6",
            "-frame-parallel",
            "1",
            "-auto-alt-ref",
            "1",
            "-lag-in-frames",
            "25",
            "-c:a",
            "libopus",
            "-b:a",
            "64k",
            "-f",
            "webm",
            "-y",
            target_path.as_str(),
        ])
        .output()
        .map_err(|err| err.to_string())?;

    if output.status.success() {
        debug!("Transcode pass 2 done, result stored at {}", target_path);
        Ok(())
    } else {
        Err(format!(
            "Pass 2 failed: ExitStatus: {},\nStderr: {}",
            output.status,
            from_utf8(&output.stderr).unwrap()
        ))
    }
}
