use actix_web::web;

use crate::controllers::{create::create, health::health};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(create);
}
