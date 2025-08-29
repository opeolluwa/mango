use serde::{Deserialize, Serialize, de::DeserializeOwned};
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

impl<T> PaginatedResponse<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(data: T, params: &PaginationParams, total_count: u64) -> Self {
        Self {
            data,
            page: params.page(),
            per_page: params.per_page(),
            total_count: total_count,
            total_pages: (total_count / params.per_page() as u64) as u32,
        }
    }
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
