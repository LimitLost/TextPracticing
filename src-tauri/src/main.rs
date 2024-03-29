#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Context;
use command_error::{CommandError, ForUserAnyError, ForUserError};

mod cache;
mod command_error;
mod logger;
mod practicing_file;

async fn setup_base(app_handle: tauri::AppHandle) -> Result<Option<String>, CommandError> {
    let path_resolver = app_handle.path_resolver();

    let cache_dir = path_resolver
        .app_cache_dir()
        .context_for_user("Getting cache path failed!")?;

    let last_open = cache::setup(cache_dir)
        .await
        .for_user("Getting cache failed!")?;

    Ok(last_open)
}

/// # Return
/// Last practicing file open, if any
#[tauri::command(async)]
async fn setup(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    match setup_base(app_handle).await {
        Ok(o) => Ok(o),
        Err(err) => Err(err.show()),
    }
}

async fn open_file_base(file_path: String) -> Result<(), CommandError> {
    let file = std::fs::File::open(&file_path)
        .with_context(|| format!("Opening file | path: {:?}", file_path))
        .for_user("Opening file failed!")?;

    //TODO Handle File Contents

    //TODO Open Cache file

    Ok(())
}

#[tauri::command(async)]
async fn open_file(file_path: String) -> Result<(), String> {
    match open_file_base(file_path).await {
        Ok(o) => Ok(o),
        Err(err) => Err(err.show()),
    }
}

fn main() {
    logger::setup().expect("Setting up logger failed!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![setup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
