use crate::infrastructure::controllers::health::health;
use crate::infrastructure::controllers::url_controllers::{create, get_user_urls, redirect};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(create);
    cfg.service(redirect);
    cfg.service(get_user_urls);
}
