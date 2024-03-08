use actix_web::{http::StatusCode, test, web, App};
use nanoid::nanoid;
use url_xs::{
    models::url::{UrlRequest, UrlResponse},
    server::AppState,
};

mod utils;
use crate::utils::{get_test_pool_db, truncate_all_tables};

#[actix_rt::test]
async fn test_create() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(url_xs::routes::config_routes),
    )
    .await;

    let mock_long_url =
        format!("https://www.google.com/aaaaaaa/adasdasd/dasdadas/aaewfwefasfewfawefwafwefwf");
    let mock_id = nanoid!(8);

    let mock_url_dto = UrlRequest {
        long_url: mock_long_url.clone(),
        user_id: mock_id.clone(),
    };

    let resp = test::TestRequest::post()
        .uri("/create")
        .set_json(&mock_url_dto)
        .send_request(&app)
        .await;

    let status = resp.status();
    let body: UrlResponse = test::read_body_json(resp).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.long_url, mock_long_url);
    assert_eq!(body.user_id, mock_id);
}
