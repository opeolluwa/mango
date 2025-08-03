use std::fmt::Display;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    Light,
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Theme::Light => "light",
                Theme::Dark => "dark",
            }
        )
    }
}
