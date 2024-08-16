use actix_web::{http, test, web, App};
use nanoid::nanoid;
use rand::{distributions::Alphanumeric, Rng};
use url_xs::{
    application::url_dtos::{UrlRequest, UserUrlsResponse},
    server::AppState,
};

use crate::utils::{get_test_pool_db, truncate_all_tables};

#[actix_rt::test]
async fn test_get_user_urls_bad_req() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(url_xs::routes::config_routes),
    )
    .await;

    let user_id = nanoid!(8);
    for _ in 0..42 {
        let mock_url_dto = generate_mock_user_url_entry(&user_id);
        let _ = test::TestRequest::post()
            .uri("/api/v1/url")
            .set_json(&mock_url_dto)
            .send_request(&app)
            .await;
    }

    let resp = test::TestRequest::get()
        .uri(&format!("/api/v1/user/{}/urls?offset=3&limit=33", user_id))
        .send_request(&app)
        .await;

    let status = resp.status();
    assert_eq!(status, http::StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_get_user_urls() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(url_xs::routes::config_routes),
    )
    .await;

    let user_id = nanoid!(8);
    for _ in 0..101 {
        let mock_url_dto = generate_mock_user_url_entry(&user_id);
        let _ = test::TestRequest::post()
            .uri("/api/v1/url")
            .set_json(&mock_url_dto)
            .send_request(&app)
            .await;
    }

    let resp = test::TestRequest::get()
        .uri(&format!("/api/v1/user/{}/urls?offset=21&limit=5", user_id))
        .send_request(&app)
        .await;

    let status = resp.status();

    let body: UserUrlsResponse = test::read_body_json(resp).await;

    assert_eq!(status, http::StatusCode::OK);
    assert_eq!(body.pagination.total_urls, 101);
    assert_eq!(body.pagination.total_pages, 21);
    assert_eq!(body.pagination.current_page, 21);
    assert_eq!(body.pagination.page_size, 1);
}

fn generate_mock_user_url_entry(user_id: &str) -> UrlRequest {
    UrlRequest {
        long_url: generate_simple_random_url(),
        user_id: user_id.to_string(),
    }
}

fn generate_simple_random_url() -> String {
    let mut rng = rand::thread_rng();
    let path_length: usize = rng.gen_range(7..42);
    let path: String = rng
        .sample_iter(&Alphanumeric)
        .take(path_length)
        .map(char::from)
        .collect();

    format!("https://testurl.com/{}", path)
}
