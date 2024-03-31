use std::path::{Path, PathBuf};

use anyhow::Context;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;

use crate::command_error::{CommandError, CommandResult, ForUserAnyError2, ForUserAnyError};

lazy_static! {
    static ref CACHE: Mutex<Option<Cache>> = Default::default();
    static ref CACHE_PATH_BASE: Mutex<Option<PathBuf>> = Default::default();
    static ref CACHE_PATH: Option<PathBuf> = CACHE_PATH_BASE.blocking_lock().take();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub last_file_open: PathBuf,
    ///In Seconds
    pub last_wait_time: Option<u32>,
}

impl Cache {
    fn save(&self, path: impl AsRef<Path>) -> Result<(), CommandError> {
        let text_file =
            std::fs::File::create(&path).context_for_user("Creating global cache file failed!")?;

        ron::ser::to_writer_pretty(text_file, self, ron::ser::PrettyConfig::new())
            .with_context(|| format!("Global Cache: {:?}", self))
            .context_for_user("Updating global cache file failed!")?;

        Ok(())
    }
}

///Returns last file open path
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

pub async fn update_last_open(new_last_open: PathBuf) -> Result<(), CommandError> {
    let mut cache_lock = CACHE.lock().await;
    let cache = cache_lock.as_mut();

    match cache {
        Some(s) => {
            s.last_file_open = new_last_open;
        }
        None => {
            *cache_lock = Some(Cache {
                last_file_open: new_last_open,
                last_wait_time: None,
            })
        }
    }

    if let Some(cache_path) = &*CACHE_PATH {
        cache_lock
            .as_ref()
            .unwrap()
            .save(cache_path)
            .context("Saving global cache")?;
    }

    Ok(())
}

///`new_wait_time` In Seconds
async fn cache_update_last_wait_time_base(new_wait_time: u32) -> Result<(), CommandError> {
    let mut cache_lock = CACHE.lock().await;

    let cache = cache_lock
        .as_mut()
        .context("Getting Cache")
        .context_for_user("Practicing File is not open!")?;

    cache.last_wait_time = Some(new_wait_time);

    if let Some(cache_path) = &*CACHE_PATH {
        cache.save(cache_path).context("Saving global cache")?;
    }

    Ok(())
}
#[tauri::command(async)]
pub async fn cache_update_last_wait_time(new_wait_time: u32) -> Result<(), String> {
    match cache_update_last_wait_time_base(new_wait_time).await {
        Ok(o) => Ok(o),
        Err(err) => Err(err.show()),
    }
}
#[tauri::command(async)]
pub async fn cache_get_last_wait_time() -> Option<u32> {
    CACHE.lock().await.as_ref().and_then(|el| el.last_wait_time)
}

#[tauri::command(async)]
pub async fn cache_get_last_file_path() -> Option<String> {
    CACHE
        .lock()
        .await
        .as_ref()
        .map(|el| el.last_file_open.display().to_string())
}
