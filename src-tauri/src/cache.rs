use std::path::PathBuf;

use anyhow::Context;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;

lazy_static! {
    static ref CACHE: Mutex<Option<Cache>> = Default::default();
    static ref CACHE_PATH_BASE: Mutex<Option<PathBuf>> = Default::default();
    static ref CACHE_PATH: Option<PathBuf> = CACHE_PATH_BASE.blocking_lock().take();
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub last_file_open: PathBuf,
    ///In Seconds
    pub last_wait_time: u32,
}

pub async fn setup(cache_dir: PathBuf) -> anyhow::Result<Option<String>> {
    let cache_path = cache_dir.join("cache.ron");

    let result = if cache_path.exists() {
        let mut file = std::fs::File::open(&cache_path)
            .with_context(|| format!("Opening cache file | path: `{}`", cache_path.display()))?;

        let cache: Cache = ron::de::from_reader(&mut file).with_context(|| {
            format!("Parsing cache file data | path: `{}`", cache_path.display())
        })?;

        let result = cache.last_file_open.display().to_string();

        *CACHE.lock().await = Some(cache);

        Some(result)
    } else {
        None
    };

    *CACHE_PATH_BASE.lock().await = Some(cache_path);

    Ok(result)
}
