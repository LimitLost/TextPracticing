#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use command_error::{CommandError, ForUserAnyError, ForUserError};

mod cache;
mod command_error;
mod logger;

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

fn main() {
    logger::setup().expect("Setting up logger failed!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![setup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
