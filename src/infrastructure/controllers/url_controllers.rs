use crate::server::AppState;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use url::Url;

use crate::application::url_dtos::{PaginationMetadata, UrlRequest, UrlResponse, UserUrlsResponse};
use crate::domain::models::url::UrlModel;

#[post("/create")]
async fn create(app_state: web::Data<AppState>, url_dto: web::Json<UrlRequest>) -> impl Responder {
    let url_dto = url_dto.into_inner();

    if !is_valid_https_url(&url_dto.long_url) {
        return HttpResponse::BadRequest().json("error: invalid url");
    }

    if !validate_short_code(&url_dto.user_id) {
        return HttpResponse::BadRequest().json("error: invalid user id");
    }

    let get_query = match sqlx::query_as!(
        UrlModel,
        "Select id, long_url, created_at, user_id from urls where long_url = $1 and user_id = $2",
        url_dto.long_url,
        url_dto.user_id
    )
    .fetch_optional(&app_state.db)
    .await
    {
        Ok(url) => url,
        Err(e) => {
            log::error!("Database error: {:?}", e);
            return HttpResponse::InternalServerError().json("error: internal server error");
        }
    };

    if let Some(url) = get_query {
        let res: UrlResponse = url.into();
        return HttpResponse::Ok().json(res);
    }

    let url = UrlModel::new(url_dto.long_url, url_dto.user_id);
    let create_query = match sqlx::query_as!(
            UrlModel,
            "insert into urls (id, long_url, user_id, created_at) values ($1, $2, $3, $4) returning id, long_url, user_id, created_at",
            url.id,
            url.long_url,
            url.user_id,
            url.created_at
        ).fetch_one(&app_state.db)
        .await {
            Ok(url) => url,
            Err(e) => {
                log::error!("Database error: {:?}", e);
                return HttpResponse::InternalServerError().json("error: internal server error");
            }
        };

    HttpResponse::Ok().json(UrlResponse::from(create_query))
}

#[derive(serde::Deserialize)]
pub struct UrlQueryParams {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[get("/user/{user_id}/urls")]
async fn get_user_urls(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    params: web::Query<UrlQueryParams>,
) -> impl Responder {
    let user_id = path.into_inner();
    let limit = params.limit.unwrap_or(5);
    let offset = (params.offset.unwrap_or(1) - 1) * limit;

    if !validate_short_code(&user_id) {
        return HttpResponse::BadRequest().json("Invalid user ID format");
    }

    let total_urls = match get_total_user_urls(&app_state.db, &user_id).await {
        Ok(total) => total,
        Err(e) => {
            log::error!("Failed to fetch total user URLs from database: {:?}", e);
            return HttpResponse::InternalServerError().json("Internal server error");
        }
    };

    if total_urls == 0 {
        return HttpResponse::NotFound().json("No URLs found for this user");
    }

    let total_pages = (total_urls as f64 / limit as f64).ceil() as u64;
    let current_page = (offset / limit) + 1;

    if current_page > total_pages {
        return HttpResponse::BadRequest().json("Requested page exceeds total pages");
    }

    let user_urls = match sqlx::query_as!(
        UrlModel,
        "SELECT * FROM urls WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        user_id,
        limit as i64,
        offset as i64
    )
    .fetch_all(&app_state.db)
    .await
    {
        Ok(urls) => urls,
        Err(e) => {
            log::error!("Failed to fetch user URLs from database: {:?}", e);
            return HttpResponse::InternalServerError().json("Internal server error");
        }
    };

    let metadata = PaginationMetadata {
        total_urls,
        total_pages,
        current_page,
        page_size: user_urls.len() as u64,
    };

    let res: UserUrlsResponse = (user_urls, user_id, metadata).into();
    HttpResponse::Ok().json(res)
}

async fn get_total_user_urls(pool: &PgPool, user_id: &str) -> Result<u64, sqlx::Error> {
    let total_urls = sqlx::query!("SELECT COUNT(*) FROM urls WHERE user_id = $1", user_id)
        .fetch_one(pool)
        .await?;

    Ok(total_urls.count.unwrap_or(0) as u64)
}

#[get("/{short_code}")]
async fn redirect(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let short_code = path.into_inner();

    if !validate_short_code(&short_code) {
        return HttpResponse::BadRequest().json("Invalid short code format");
    }

    match sqlx::query_as!(UrlModel, "SELECT * FROM urls WHERE id = $1", short_code)
        .fetch_optional(&app_state.db)
        .await
    {
        Ok(Some(url)) => HttpResponse::MovedPermanently()
            .append_header((LOCATION, url.long_url))
            .finish(),
        Ok(None) => HttpResponse::NotFound().json("URL not found, or has expired"),
        Err(e) => {
            log::error!("Failed to fetch URL from database: {:?}", e);
            HttpResponse::InternalServerError().json("Internal server error")
        }
    }
}

fn is_valid_https_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(url) => url.scheme() == "https",
        Err(_) => false,
    }
}

fn validate_short_code(short_code: &str) -> bool {
    short_code.len() == 8
        && short_code
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_' || c == '-')
}
