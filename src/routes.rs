use actix_web::web;

use crate::controllers::{
    create::create, get_user_urls::get_user_urls, health::health, redirect::redirect,
};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(create);
    cfg.service(redirect);
    cfg.service(get_user_urls);
}
