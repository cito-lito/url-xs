use crate::application::url_dtos::UrlRequest;
use crate::application::url_use_cases::UrlUseCases;
use crate::infrastructure::db::url_repositories::PostgresUrlRepository;
use crate::server::AppState;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse, Responder};
use url::Url;

#[post("/url")]
async fn shorten(app_state: web::Data<AppState>, url_dto: web::Json<UrlRequest>) -> impl Responder {
    let url_dto = url_dto.into_inner();

    if !is_valid_https_url(&url_dto.long_url) {
        return HttpResponse::BadRequest().json("error: invalid url");
    }

    if !validate_short_code(&url_dto.user_id) {
        return HttpResponse::BadRequest().json("error: invalid user id");
    }

    let url_repo = PostgresUrlRepository::new(app_state.db.clone());

    let use_cases = UrlUseCases { url_repo };
    match use_cases.create_url(url_dto).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            log::error!("Failed to create URL: {:?}", e);
            HttpResponse::InternalServerError().json("Internal server error")
        }
    }
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

    let url_repo = PostgresUrlRepository::new(app_state.db.clone());
    let use_cases = UrlUseCases { url_repo };

    match use_cases
        .get_user_urls(&user_id, limit as i64, offset as i64)
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            log::error!("Failed to get user URLs: {:?}", e);
            if e.contains("No urls found") {
                return HttpResponse::NotFound().json("No URLs found");
            }
            if e.contains("Invalid page") {
                return HttpResponse::BadRequest().json("Invalid page");
            }
            HttpResponse::InternalServerError().json("Internal server error")
        }
    }
}

#[get("/{short_code}")]
async fn redirect(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let short_code = path.into_inner();

    if !validate_short_code(&short_code) {
        return HttpResponse::BadRequest().json("Invalid short code format");
    }

    let url_repo = PostgresUrlRepository::new(app_state.db.clone());
    let use_cases = UrlUseCases { url_repo };

    match use_cases.find_by_short_code(&short_code).await {
        Ok(long_url) => HttpResponse::MovedPermanently()
            .append_header((LOCATION, long_url))
            .finish(),
        Err(e) => {
            log::error!("Failed to redirect: {:?}", e);
            HttpResponse::NotFound().json("URL not found")
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
