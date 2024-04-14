use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use fancy_regex::{Regex, RegexBuilder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::command_error::{CommandError, CommandResult, ForUserAnyError, ForUserAnyError2};

lazy_static! {
    static ref PRACTICING_REGEX: Regex = RegexBuilder::new(include_str!("regex"))
        .backtrack_limit(1_000_000_00)
        .build()
        .unwrap();
}

#[test]
fn regex_test() {
    println!(">====");
    let file = include_str!("example-file.txt");

    let data = match PracticingFileData::from_text(file) {
        Ok(o) => o,
        Err(err) => {
            panic!("{} ||| Debug: {:?}", err.for_user_message, err.error);
        }
    };

    if data.subjects.is_empty() {
        println!("No Subjects were found in the test file!")
    } else {
        println!("Parsed Data:");
        println!("===================================");
        for (title, subject) in data.subjects.into_iter() {
            println!("title: `{title}`");
            println!("=================");
            println!("data: `{subject:#?}`");
            println!("=================");
        }
        println!("===================================");
    }
    println!("Regex Test Complete!");
    println!(">====");
}
#[derive(Debug)]
pub struct PracticingSubject {
    ///key - name of the capture
    ///
    ///data - value of the capture
    ///
    ///Contains every named regex capture other than the `subject` capture
    pub captures: HashMap<String, String>,
}

impl PracticingSubject {
    pub fn get_captures<'a>(&'a self) -> Vec<(&'static str, &'a String)> {
        let mut result = Vec::new();

        for capture_name in PRACTICING_REGEX.capture_names().flatten() {
            if let Some(data) = self.captures.get(capture_name) {
                result.push((capture_name, data))
            }
        }

        result
    }
}

pub struct PracticingFileData {
    ///key - subject name
    pub subjects: HashMap<String, PracticingSubject>,
}

impl PracticingFileData {
    fn from_text(text: &str) -> Result<Self, CommandError> {
        let mut subjects = HashMap::new();

        for captures in PRACTICING_REGEX.captures_iter(text) {
            let captures = captures.context_for_user("Getting text capture group failed!")?;

            //Get Subject Name
            let subject = if let Some(s) = captures.name("subject") {
                s.as_str().to_string()
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
                    let capture_data = capture.as_str().to_string();

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

    fn new(path: impl AsRef<Path>) -> Result<Self, CommandError> {
        let mut text_file = std::fs::File::open(&path).context_for_user("Opening file failed!")?;

        let mut text_data = String::new();

        text_file
            .read_to_string(&mut text_data)
            .context_for_user("Reading file failed!")?;

        Self::from_text(&text_data)
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
