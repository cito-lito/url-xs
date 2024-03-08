use nanoid::nanoid;
use serde::{Deserialize, Serialize};

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
pub struct UrlRequest {
    pub long_url: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlResponse {
    pub short_url: String,
    pub long_url: String,
    pub user_id: String,
    pub short_code: String,
    pub created_at: chrono::NaiveDateTime,
}

impl UrlResponse {
    pub fn _new(
        short_code: String,
        long_url: String,
        user_id: String,
        created_at: chrono::NaiveDateTime,
    ) -> Self {
        const BASE_URL: &str = "http://localhost:3003/";
        let short_url = format!("{}{}", BASE_URL, short_code);

        Self {
            short_url,
            long_url,
            user_id,
            short_code,
            created_at,
        }
    }

    pub fn from_db_obj(data: UrlDbObject) -> Self {
        const BASE_URL: &str = "http://localhost:3003/";
        let short_url = format!("{}{}", BASE_URL, data.id);

        Self {
            short_url,
            long_url: data.long_url,
            user_id: data.user_id,
            short_code: data.id,
            created_at: data.created_at,
        }
    }
}
