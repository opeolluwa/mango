use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    pub identifier: Uuid,
    pub payload: T,
}

impl<T> Display for RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:#?}", self.identifier, self.payload)
    }
}

impl<T> RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    pub fn new(message: T) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            payload: message,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PdfConversionStarted {
    pub user_identifier: Uuid,
    pub file_name: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PdfConversionEnded {
    pub user_identifier: Uuid,
    pub file_name: String,
    pub file_path: String,
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
