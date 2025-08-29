use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "paginatedResponse.d.ts")]
pub struct PaginatedResponse<T> {
    pub data: T,
    pub page: u32,
    pub per_page: u32,
    pub total_count: u64,
    pub(crate) total_pages: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    pub(crate) page: Option<u32>,
    pub(crate) per_page: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1u32),
            per_page: Some(10u32),
        }
    }
}

impl PaginationParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }
    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(10)
    }
}
