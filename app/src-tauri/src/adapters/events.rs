use serde::{Deserialize, Serialize};
use ts_rs::TS;

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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct AudioSynthesisEvent {
    pub file_name: String,
    pub audio_src: String,
}
