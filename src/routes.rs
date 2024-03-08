use actix_web::web;

use crate::controllers::{create::create, health::health, redirect::redirect};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(create);
    cfg.service(redirect);
}
