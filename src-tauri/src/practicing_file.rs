use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use serde::{Deserialize, Serialize};

use crate::command_error::{CommandError, CommandResult, ForUserAnyError, ForUserAnyError2};

lazy_static! {
    static ref PRACTICING_REGEX:Regex=Regex::new(r"(X. )?(?<subject>.*) Wstęp\n+(?<wstep>(\s{3,}.*\n)*)\s*(X. )?\2 Teza\n+(?<teza>(\s{3,}.*\n)*)\s*(X. )?\2 Odniesienie\n+(?<odniesienie>(\s{3,}.*\n)*)\s*((X. )?\2 Kontekst\n+(?<kontekst>(\s{3,}.*\n)*)\s*)?(X. )?\2 Podsumowanie\n+(?<podsumowanie>(\s{3,}.*\n)*)").unwrap();
}

pub struct PracticingSubject {
    ///key - name of the capture
    ///
    ///data - value of the capture
    ///
    ///Contains every named regex capture other than the `subject` capture
    pub captures: HashMap<String, String>,
}

pub struct PracticingFileData {
    ///key - subject name
    pub subjects: HashMap<String, PracticingSubject>,
}

impl PracticingFileData {
    fn new(path: impl AsRef<Path>) -> Result<Self, CommandError> {
        let mut text_file = std::fs::File::open(&path).context_for_user("Opening file failed!")?;

        let mut text_data = Vec::new();

        text_file
            .read_to_end(&mut text_data)
            .context_for_user("Reading file failed!")?;

        let mut subjects = HashMap::new();

        for captures in PRACTICING_REGEX.captures_iter(&text_data) {
            //Get Subject Name
            let subject = if let Some(s) = captures.name("subject") {
                String::from_utf8_lossy(s.as_bytes()).to_string()
            } else {
                //No subject in the current capture, skip

                continue;
            };

            let mut captures_mapped = HashMap::new();
            //Save all named captures, skip the one named `subject`
            for capture_name in PRACTICING_REGEX.capture_names().flatten() {
                if capture_name == "subject" {
                    continue;
                }
                if let Some(capture) = captures.name(capture_name) {
                    let capture_data = String::from_utf8_lossy(capture.as_bytes()).to_string();

                    captures_mapped.insert(capture_name.to_owned(), capture_data);
                }
            }
            //Save current subject
            subjects.insert(
                subject,
                PracticingSubject {
                    captures: captures_mapped,
                },
            );
        }

        Ok(PracticingFileData { subjects })
    }
}

///.practicing file data
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PracticingFileCache {
    pub done_subjects: HashSet<String>,
}

impl PracticingFileCache {
    fn new(path: impl AsRef<Path>) -> Result<Self, CommandError> {
        let text_file = std::fs::File::open(&path).context_for_user("Opening file failed!")?;

        let this = ron::de::from_reader(text_file)
            .context_for_user("Reading practicing file cache failed!")?;

        Ok(this)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), CommandError> {
        let text_file = std::fs::File::create(&path)
            .context_for_user("Creating practicing file cache failed!")?;

        ron::ser::to_writer_pretty(text_file, self, ron::ser::PrettyConfig::new())
            .with_context(|| format!("Cache: {:?}", self))
            .context_for_user("Updating practicing file cache failed!")?;

        Ok(())
    }
}
///Returned `PathBuf` leads to file cache
pub fn open_practicing_file(
    path: impl AsRef<Path>,
) -> Result<(PracticingFileData, PathBuf, PracticingFileCache), CommandError> {
    let path = path.as_ref();
    let (text_file_path, cache_file_path) = if path.extension() == Some(OsStr::new("practicing")) {
        (path.with_extension(""), path.to_owned())
    } else {
        (
            path.to_owned(),
            path.with_extension(format!(
                "{}.practicing",
                path.extension().unwrap_or_default().to_string_lossy()
            )),
        )
    };

    let practicing_file_data = PracticingFileData::new(&text_file_path).with_context(|| {
        format!(
            "Creating Practicing File Data | file_path: {}",
            text_file_path.display()
        )
    })?;

    let practicing_file_cache = if cache_file_path.exists() {
        PracticingFileCache::new(&cache_file_path).with_context(|| {
            format!(
                "Creating Practicing File Cache | file_path: {}",
                cache_file_path.display()
            )
        })?
    } else {
        Default::default()
    };

    Ok((practicing_file_data, cache_file_path, practicing_file_cache))
}
