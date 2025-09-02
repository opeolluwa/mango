// #[derive(uniffi::Enum)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Languages {
    #[default]
    English,
}

