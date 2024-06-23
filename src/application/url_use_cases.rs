use crate::domain::models::url::UrlModel;

use super::{
    url_dtos::{PaginationMetadata, UrlRequest, UrlResponse, UserUrlsResponse},
    url_repositories::UrlRepository,
};

pub struct UrlUseCases<R: UrlRepository> {
    pub url_repo: R,
}

impl<R: UrlRepository> UrlUseCases<R> {
    pub async fn create_url(&self, url_request: UrlRequest) -> Result<UrlResponse, String> {
        let existing_url = self
            .url_repo
            .find_by_long_url_and_user_id(&url_request.long_url, &url_request.user_id)
            .await?;

        if let Some(short_url) = existing_url {
            return Ok(short_url.into());
        }

        let new_short_url = UrlModel::new(url_request.long_url, url_request.user_id);
        self.url_repo.save(&new_short_url).await?;
        Ok(new_short_url.into())
    }

    pub async fn get_user_urls(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<UserUrlsResponse, String> {
        let total_urls = self.url_repo.count_by_user_id(user_id).await?;
        if total_urls == 0 {
            return Err("No urls found".to_string());
        }

        let total_pages = (total_urls as f64 / limit as f64).ceil() as u64;
        let current_page = (offset / limit + 1) as u64;

        if current_page > total_pages {
            return Err("Invalid page".to_string());
        }

        let user_urls = self
            .url_repo
            .find_by_user_id(user_id, limit, offset)
            .await?;

        let metadata = PaginationMetadata {
            total_urls,
            total_pages,
            current_page,
            page_size: user_urls.len() as u64,
        };

        Ok(UserUrlsResponse::from((
            user_urls,
            user_id.to_string(),
            metadata,
        )))
    }

    pub async fn find_by_short_code(&self, short_code: &str) -> Result<String, String> {
        let url = self.url_repo.find_by_id(short_code).await?;
        match url {
            Some(url) => Ok(url.long_url),
            None => Err("URL not found".to_string()),
        }
    }
}
