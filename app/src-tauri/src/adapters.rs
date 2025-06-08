use std::path::Path;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioBook {
    pub file_name: String,
    pub play_back_duration: u64,
    pub audio_src: String,
}

impl AudioBook {
    pub fn from_path(path: &Path) -> Option<Self> {
        let path_buf = path.canonicalize().ok()?;
        let file_name = path_buf.file_name()?.to_str()?.to_string();

        let display_name = file_name.strip_suffix(".wav").unwrap_or(&file_name);

        Some(Self {
            file_name: display_name.to_string(),
            play_back_duration: 45, // Consider computing real duration later
            audio_src: path.canonicalize().ok()?.to_str().unwrap().to_string(),
        })
    }
}