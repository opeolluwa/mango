use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Event<T>
where
    T: Serialize + Debug + DeserializeOwned,
{
    pub identifier: Uuid,
    pub payload: T,
}

impl<T> Display for Event<T>
where
    T: Serialize + Debug + DeserializeOwned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:#?}", self.identifier, self.payload)
    }
}

impl<T> Event<T>
where
    T: Serialize + Debug + DeserializeOwned,
{
    pub fn new(message: T) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            payload: message,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct EmailMessage {}

#[derive(Serialize, Debug, Deserialize)]
pub struct ConvertDocumentMessage {
    pub user_identifier: Uuid,
    pub document_path: String,
    pub wav_output_path: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ConvertWavToMp3Message {
    pub user_identifier: Uuid,
    pub wav_input_file: String,
    pub mp3_input_path: String,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct WavConversionStarted {
    pub user_identifier: Uuid,
    pub input_file: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct WavConversionEnded {
    pub user_identifier: Uuid,
    pub input_file: String,
    pub output_file: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ConvertDocument {
    pub playlist_identifier: Option<Uuid>,
    pub file_name: String,
    pub user_identifier: Uuid,
    pub file_path: PathBuf,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all="snake_case")]
pub struct DocumentConverted {
    pub playlist_identifier: Option<Uuid>,
    pub file_name: String,
    pub user_identifier: Uuid,
    pub url: String,
}
