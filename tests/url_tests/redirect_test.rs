use actix_web::{http::StatusCode, test, web, App};
use nanoid::nanoid;
use url_xs::{
    application::url_dtos::{UrlRequest, UrlResponse},
    server::AppState,
};

use crate::utils::{get_test_pool_db, truncate_all_tables};

#[actix_rt::test]
async fn test_redirect() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(url_xs::routes::config_routes),
    )
    .await;

    let mock_long_url = format!("https://actix.rs/docs/testing");
    let mock_id = nanoid!(8);

    let mock_url_dto = UrlRequest {
        long_url: mock_long_url.clone(),
        user_id: mock_id.clone(),
    };

    let created_url = test::TestRequest::post()
        .uri("/create")
        .set_json(&mock_url_dto)
        .send_request(&app)
        .await;
    let body: UrlResponse = test::read_body_json(created_url).await;
    let short_code = body.short_code;

    let resp = test::TestRequest::get()
        .uri(&format!("/{}", short_code))
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(resp.headers().get("location").unwrap(), &mock_long_url);
}
