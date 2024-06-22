use crate::domain::models::url::UrlModel;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlRequest {
    pub long_url: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlResponse {
    pub short_url: String,
    pub long_url: String,
    pub user_id: String,
    pub short_code: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUrls {
    pub short_url: String,
    pub long_url: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUrlsResponse {
    pub user_id: String,
    pub urls: Vec<UserUrls>,
    pub pagination: PaginationMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMetadata {
    pub total_urls: u64,
    pub total_pages: u64,
    pub current_page: u64,
    pub page_size: u64,
}

impl From<UrlModel> for UserUrls {
    fn from(data: UrlModel) -> Self {
        let base_url =
            std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:3003/".to_string());
        let short_url = url::Url::parse(&base_url)
            .unwrap()
            .join(&data.id)
            .unwrap()
            .to_string();
        Self {
            short_url,
            long_url: data.long_url,
            created_at: data.created_at.to_string(),
        }
    }
}

impl From<(Vec<UrlModel>, String, PaginationMetadata)> for UserUrlsResponse {
    fn from(params: (Vec<UrlModel>, String, PaginationMetadata)) -> Self {
        let (data, user_id, pagination) = params;
        let urls = data.into_iter().map(UserUrls::from).collect();

        Self {
            user_id,
            urls,
            pagination,
        }
    }
}

impl From<UrlModel> for UrlResponse {
    fn from(data: UrlModel) -> Self {
        let base_url =
            std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:3003/".to_string());
        let short_url = url::Url::parse(&base_url)
            .unwrap()
            .join(&data.id)
            .unwrap()
            .to_string();
        Self {
            short_url,
            long_url: data.long_url,
            user_id: data.user_id,
            short_code: data.id,
            created_at: data.created_at,
        }
    }
}
