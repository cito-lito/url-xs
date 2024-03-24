use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
pub struct UrlDbObject {
    pub id: String,
    pub long_url: String,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl UrlDbObject {
    pub fn new(long_url: String, user_id: String) -> Self {
        Self {
            id: nanoid!(8),
            long_url,
            user_id,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

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
    pub created_at: chrono::NaiveDateTime,
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

impl UserUrls {
    pub fn from_db_obj(data: UrlDbObject) -> Self {
        let base_url = std::env::var("SERVER_URL").unwrap_or("http://localhost:3003".to_string());
        let short_url = Url::parse(&base_url)
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

impl UserUrlsResponse {
    pub fn from_db_obj(
        data: Vec<UrlDbObject>,
        user_id: String,
        pagination: PaginationMetadata,
    ) -> Self {
        let mut urls = Vec::new();
        for url in data {
            urls.push(UserUrls::from_db_obj(url));
        }
        Self {
            user_id,
            urls,
            pagination,
        }
    }
}

impl UrlResponse {
    pub fn from_db_obj(data: UrlDbObject) -> Self {
        let base_url = std::env::var("SERVER_URL").unwrap_or("http://localhost:3003".to_string());
        let short_url = Url::parse(&base_url)
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
