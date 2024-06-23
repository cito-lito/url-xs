use crate::domain::models::url::UrlModel;
use std::future::Future;

pub trait UrlRepository {
    fn find_by_long_url_and_user_id(
        &self,
        long_url: &str,
        user_id: &str,
    ) -> impl Future<Output = Result<Option<UrlModel>, String>> + Send;

    fn save(&self, url: &UrlModel) -> impl Future<Output = Result<(), String>> + Send;

    fn find_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> impl Future<Output = Result<Vec<UrlModel>, String>> + Send;

    fn count_by_user_id(&self, user_id: &str) -> impl Future<Output = Result<u64, String>> + Send;

    fn find_by_id(&self, id: &str)
        -> impl Future<Output = Result<Option<UrlModel>, String>> + Send;
}
