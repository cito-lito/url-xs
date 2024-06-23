use crate::{application::url_repositories::UrlRepository, domain::models::url::UrlModel};
use sqlx::PgPool;

pub struct PostgresUrlRepository {
    pool: PgPool,
}
impl PostgresUrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UrlRepository for PostgresUrlRepository {
    async fn find_by_long_url_and_user_id(
        &self,
        long_url: &str,
        user_id: &str,
    ) -> Result<Option<UrlModel>, String> {
        find_by_long_url_and_user_id(&self.pool, long_url, user_id).await
    }
    async fn save(&self, url: &UrlModel) -> Result<(), String> {
        save(&self.pool, url).await
    }
    async fn find_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UrlModel>, String> {
        find_by_user_id(&self.pool, user_id, limit, offset).await
    }
    async fn count_by_user_id(&self, user_id: &str) -> Result<u64, String> {
        count_by_user_id(&self.pool, user_id).await
    }
    async fn find_by_id(&self, id: &str) -> Result<Option<UrlModel>, String> {
        find_by_id(&self.pool, id).await
    }
}

async fn find_by_long_url_and_user_id(
    pool: &PgPool,
    long_url: &str,
    user_id: &str,
) -> Result<Option<UrlModel>, String> {
    sqlx::query_as!(
        UrlModel,
        "SELECT id, long_url, created_at, user_id FROM urls WHERE long_url = $1 AND user_id = $2",
        long_url,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {:?}", e))
}

async fn save(pool: &PgPool, url: &UrlModel) -> Result<(), String> {
    sqlx::query!(
        "INSERT INTO urls (id, long_url, user_id, created_at) VALUES ($1, $2, $3, $4)",
        url.id,
        url.long_url,
        url.user_id,
        url.created_at
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {:?}", e))?;
    Ok(())
}

async fn find_by_user_id(
    pool: &PgPool,
    user_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<UrlModel>, String> {
    sqlx::query_as!(
        UrlModel,
        "SELECT id, long_url, created_at, user_id FROM urls WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {:?}", e))
}

async fn count_by_user_id(pool: &PgPool, user_id: &str) -> Result<u64, String> {
    sqlx::query!("SELECT COUNT(*) FROM urls WHERE user_id = $1", user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {:?}", e))
        .map(|row| row.count.unwrap_or(0) as u64)
}

async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<UrlModel>, String> {
    sqlx::query_as!(UrlModel, "SELECT * FROM urls WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {:?}", e))
}
