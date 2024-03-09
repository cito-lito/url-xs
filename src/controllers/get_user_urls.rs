use crate::models::url::{PaginationMetadata, UrlDbObject, UserUrlsResponse};
use crate::server::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::postgres::PgPool;

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
    let limit = params.limit.unwrap_or(10);
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
        UrlDbObject,
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

    let res = UserUrlsResponse::from_db_obj(user_urls, user_id, metadata);
    HttpResponse::Ok().json(res)
}

fn validate_short_code(short_code: &str) -> bool {
    short_code.len() == 8
        && short_code
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_' || c == '-')
}

async fn get_total_user_urls(pool: &PgPool, user_id: &str) -> Result<u64, sqlx::Error> {
    let total_urls = sqlx::query!("SELECT COUNT(*) FROM urls WHERE user_id = $1", user_id)
        .fetch_one(pool)
        .await?;

    Ok(total_urls.count.unwrap_or(0) as u64)
}
