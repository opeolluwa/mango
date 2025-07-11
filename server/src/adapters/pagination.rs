use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub(crate) data: Vec<T>,
    pub(crate) page: u32,
    pub(crate) per_page: u32,
    pub(crate) total_count: u64,
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
