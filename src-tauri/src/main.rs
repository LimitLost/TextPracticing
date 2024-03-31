#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use anyhow::Context;
use cache::update_last_open;
use command_error::{CommandError, CommandResult, ForUserAnyError, ForUserAnyError2, ForUserError};
use lazy_static::lazy_static;
use practicing_file::{open_practicing_file, PracticingFileCache, PracticingFileData};
use rand::seq::IteratorRandom;
use tauri::async_runtime::Mutex;

mod cache;
mod command_error;
mod logger;
mod practicing_file;

lazy_static! {
    static ref CURRENT_FILE_CACHE: Mutex<Option<PracticingFileCache>> = Default::default();
    static ref CURRENT_FILE_CACHE_PATH: Mutex<Option<PathBuf>> = Default::default();
    static ref CURRENT_FILE_DATA: Mutex<Option<PracticingFileData>> = Default::default();
    static ref CURRENT_SUBJECT: Mutex<Option<String>> = Default::default();
}

///Converts provided value to JSON
///
/// On error converted value is shown by using `Display` trait
fn to_json_display<T: std::fmt::Display + serde::Serialize>(
    el: &T,
) -> Result<String, CommandError> {
    serde_json::to_string(el)
        .with_context(|| format!("Converting `{}` to json failed", el))
        .for_user("Converting a value into JSON failed")
}

///Converts provided value to JSON
///
/// On error converted value is shown by using `Debug` trait
fn to_json_debug<T: std::fmt::Debug + serde::Serialize>(el: &T) -> Result<String, CommandError> {
    serde_json::to_string(el)
        .with_context(|| format!("Converting {:?} to json failed", el))
        .for_user("Converting a value into JSON failed")
}

///Evaluates javascript in tauri window
fn tauri_command(window: &tauri::Window, command: &str) -> anyhow::Result<()> {
    window
        .eval(command)
        .with_context(|| format!("Performing tauri command failed | command: {:?}", command))
}

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

fn update_possible_selections(
    window: &tauri::Window,
    not_done_possible: bool,
    done_possible: bool,
) -> Result<(), CommandError> {
    tauri_command(
        window,
        &format!("window[\"update_possible_selections\"]({not_done_possible},{done_possible})"),
    )
    .for_user("Updating User Interface Failed!")
}

async fn open_file_base(window: tauri::Window, file_path: String) -> Result<(), CommandError> {
    let (file_data, file_cache_path, file_cache) = open_practicing_file(&file_path)
        .with_context(|| format!("Opening practicing files | path gotten: {:?}", file_path))?;

    if file_data.subjects.is_empty() {
        return Err(CommandError::only_for_user(
            "There is no practicing data found in the selected file!".to_owned(),
        ));
    }

    update_last_open(command_error::ForUserAnyError::context_for_user(
        file_path.parse(),
        "Provided file path is invalid!",
    )?)
    .await
    .context("Updating cache failed!")?;

    update_possible_selections(
        &window,
        file_data.subjects.len() != file_cache.done_subjects.len(),
        !file_cache.done_subjects.is_empty(),
    )
    .context("Updating possible selections")?;

    *CURRENT_FILE_CACHE.lock().await = Some(file_cache);
    *CURRENT_FILE_CACHE_PATH.lock().await = Some(file_cache_path);
    *CURRENT_FILE_DATA.lock().await = Some(file_data);

    Ok(())
}

#[tauri::command(async)]
async fn open_file(window: tauri::Window, file_path: String) -> Result<(), String> {
    match open_file_base(window, file_path).await {
        Ok(o) => Ok(o),
        Err(err) => Err(err.show()),
    }
}

fn reset_learning_panel(window: &tauri::Window, subject: &str) -> Result<(), CommandError> {
    let subject = to_json_debug(&subject).context("Converting subject to json")?;

    tauri_command(
        window,
        &format!("window[\"reset_learning_panel\"]({subject})"),
    )
    .for_user("Resetting learning panel Failed!")
}

fn create_learning_panel_field(
    window: &tauri::Window,
    title: &str,
    data: &str,
) -> Result<(), CommandError> {
    let title = to_json_debug(&title).context("Converting title to json")?;
    let data = to_json_debug(&data).context("Converting data to json")?;

    tauri_command(
        window,
        &format!("window[\"create_learning_panel_field\"]({title},{data})"),
    )
    .for_user("Creating learning panel field Failed!")
}

async fn open_random_subject_base(window: tauri::Window, done: bool) -> Result<(), CommandError> {
    let file_cache_lock = CURRENT_FILE_CACHE.lock().await;
    let file_data_lock = CURRENT_FILE_DATA.lock().await;

    let (file_cache, file_data) =
        if let (Some(a), Some(b)) = (file_cache_lock.as_ref(), file_data_lock.as_ref()) {
            (a, b)
        } else {
            return Err(CommandError::only_for_user(
                "Practicing File is not open!".to_owned(),
            ));
        };

    let (random_subject_name, random_subject) = {
        let mut rng = rand::thread_rng();

        file_data
            .subjects
            .iter()
            .filter(|(name, _)| file_cache.done_subjects.contains(*name) == done)
            .choose(&mut rng)
            .context_for_user("There is no practicing data found in the selected file!")?
    };

    *CURRENT_SUBJECT.lock().await = Some(random_subject_name.clone());

    reset_learning_panel(&window, random_subject_name.as_str())
        .context("Resetting learning panel")?;

    for (key, data) in random_subject.captures.iter() {
        create_learning_panel_field(&window, key.as_str(), data.as_str())
            .context("Creating learning panel field")?;
    }

    Ok(())
}

#[tauri::command(async)]
async fn open_random_subject(window: tauri::Window, done: bool) -> Result<(), String> {
    match open_random_subject_base(window, done).await {
        Ok(o) => Ok(o),
        Err(err) => Err(err.show()),
    }
}
///Updates Cache
async fn subject_done_base() -> Result<(), CommandError> {
    let mut current_cache_lock = CURRENT_FILE_CACHE.lock().await;

    let current_cache = current_cache_lock
        .as_mut()
        .context("Cache")
        .context_for_user("Practicing file is not open!")?;

    let subject = CURRENT_SUBJECT
        .lock()
        .await
        .clone()
        .context_for_user("Subject is not open!")?;

    //Save updated cache only if needed
    if current_cache.done_subjects.insert(subject) {
        let cache_path_lock = CURRENT_FILE_CACHE_PATH.lock().await;

        let cache_path = cache_path_lock
            .as_ref()
            .context("Cache Path")
            .context_for_user("Practicing file is not open!")?;

        current_cache
            .save(cache_path)
            .with_context(|| format!("path: {}", cache_path.display()))?;
    }

    Ok(())
}

#[tauri::command(async)]
async fn subject_done() -> Result<(), String> {
    match subject_done_base().await {
        Ok(o) => Ok(o),
        Err(err) => Err(err.show()),
    }
}

fn main() {
    logger::setup().expect("Setting up logger failed!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            setup,
            open_file,
            open_random_subject,
            subject_done,
            cache::cache_update_last_wait_time,
            cache::cache_get_last_wait_time,
            cache::cache_get_last_file_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
