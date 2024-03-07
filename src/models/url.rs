use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UrlDto {
    #[validate(url)]
    pub long_url: String,
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct UrlResponse {
    pub short_url: String,
    pub long_url: String,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl UrlResponse {
    pub fn new(
        short_code: String,
        long_url: String,
        user_id: String,
        created_at: chrono::NaiveDateTime,
    ) -> Self {
        const BASE_URL: &str = "http://localhost:3030/";
        Self {
            short_url: format!("{}{}", BASE_URL, short_code),
            long_url,
            user_id,
            created_at,
        }
    }
}
