use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct PracticingSubject {
    introduction: String,
    thesis: String,
    reference: String,
    context: String,
    summary: String,
}

pub struct PracticingFileData {
    subjects: HashMap<String, PracticingSubject>,
}

///.practicing file data
#[derive(Deserialize, Serialize)]
pub struct PracticingFileCache {
    done_subjects: Vec<String>,
}
