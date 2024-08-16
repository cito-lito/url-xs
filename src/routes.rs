use crate::infrastructure::controllers::health::health;
use crate::infrastructure::controllers::url_controllers::{get_user_urls, redirect, shorten};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(redirect);
    cfg.service(
        web::scope("/api/v1")
            .service(shorten)
            .service(get_user_urls),
    );
}
