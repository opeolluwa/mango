use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export)]
pub enum Events {
    #[serde(rename = "processing-audio")]
    Processing,
    #[serde(rename = "finished-processing-audio")]
    FinishedProcessing,
    #[serde(rename = "currently-playing-audio")]
    CurrentlyPlaying,
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
