use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
#[derive(Debug, Serialize, Deserialize, Clone, TS, FromRow, Default)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub app_initialized: bool,
}
