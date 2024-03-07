use crate::models::url::{UrlDbObject, UrlDto, UrlResponse};
use crate::server::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use validator::Validate;

#[post("/create")]
pub async fn create(app_state: web::Data<AppState>, url_dto: web::Json<UrlDto>) -> impl Responder {
    let url_dto = url_dto.into_inner();

    if let Err(validation_error) = url_dto.validate() {
        log::error!("Validation error: {:?}", validation_error);
        return HttpResponse::BadRequest().json("error: malformed url");
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
        let res = UrlResponse::new(url.id, url.long_url, url.user_id, url.created_at);
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

    HttpResponse::Ok().json(UrlResponse::new(
        create_query.id,
        create_query.long_url,
        create_query.user_id,
        create_query.created_at,
    ))
}
