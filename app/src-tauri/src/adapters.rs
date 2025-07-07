use crate::database::AudioBook;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const AUDIO_PROCESSING_EVENT: &str = "processing-audio";
pub const FINISHED_AUDIO_PROCESSING_EVENT: &str = "finished-processing-audio";
pub const CURRENTLY_PLAYING_EVENT: &str = "currently-playing-audio";

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export)]
pub enum Events {
    #[serde(rename = "processing-audio")]
    ProcessingAudio,
    #[serde(rename = "finished-processing-audio")]
    FinishedProcessingAudio,
    #[serde(rename = "currently-playing-audio")]
    CurrentlyPlayingAudio,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct CurrentAudioMetadata {
    pub volume: f32,
    pub speed: f32,
    pub duration: u32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioLibrary {
    audio_books: Vec<AudioBook>,
}

impl AudioLibrary {
    pub fn new(books: Vec<AudioBook>) -> Self {
        Self { audio_books: books }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioSynthesisEvent {
    pub file_name: String,
    pub audio_src: String,
}
