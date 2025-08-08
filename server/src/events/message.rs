use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RedisMessage<T>
where
    T: Serialize + Debug,
{
    pub identifier: Uuid,
    pub payload: T,
}

impl<T> Display for RedisMessage<T>
where
    T: Serialize + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:#?}", self.identifier, self.payload)
    }
}

impl<T> RedisMessage<T>
where
    T: Serialize + Debug,
{
    pub fn new(message: T) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            payload: message,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ConvertDocumentMessage {
    pub user_identifier: Uuid,
    pub document_path: String,
    pub wav_output_path: String,
}

#[derive(Serialize, Debug)]
pub struct ConvertPcmMessage {
    pub user_identifier: Uuid,
    pub wav_input_file: String,
    pub mp3_input_path: String,
}
#[derive(Serialize, Debug)]
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
