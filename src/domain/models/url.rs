use chrono::NaiveDateTime;
use nanoid::nanoid;

#[derive(Debug)]
pub struct UrlModel {
    pub id: String,
    pub long_url: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
}

impl UrlModel {
    pub fn new(long_url: String, user_id: String) -> Self {
        Self {
            id: nanoid!(8),
            long_url,
            user_id,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
