use crate::{models::url::UrlDbObject, server::AppState};
use actix_web::{get, http::header::LOCATION, web, HttpResponse, Responder};

#[get("/redirect/{short_code}")]
async fn redirect(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let short_code = path.into_inner();

    if !validate_short_code(&short_code) {
        return HttpResponse::BadRequest().json("Invalid short code format");
    }

    match sqlx::query_as!(UrlDbObject, "SELECT * FROM urls WHERE id = $1", short_code)
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

//
fn validate_short_code(short_code: &str) -> bool {
    short_code.len() == 8
        && short_code
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_' || c == '-')
}
