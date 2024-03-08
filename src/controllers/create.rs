use crate::models::url::{UrlDbObject, UrlRequest, UrlResponse};
use crate::server::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use url::Url;

#[post("/create")]
async fn create(app_state: web::Data<AppState>, url_dto: web::Json<UrlRequest>) -> impl Responder {
    let url_dto = url_dto.into_inner();

    if !is_valid_https_url(&url_dto.long_url) {
        return HttpResponse::BadRequest().json("error: invalid url");
    }

    let get_query = match sqlx::query_as!(
        UrlDbObject,
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
        let res = UrlResponse::from_db_obj(url);
        return HttpResponse::Ok().json(res);
    }

    let url = UrlDbObject::new(url_dto.long_url, url_dto.user_id);
    let create_query = match sqlx::query_as!(
            UrlDbObject,
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

    HttpResponse::Ok().json(UrlResponse::from_db_obj(create_query))
}

fn is_valid_https_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(url) => url.scheme() == "https",
        Err(_) => false,
    }
}
