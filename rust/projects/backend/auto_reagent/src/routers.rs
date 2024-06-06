use super::handlers::*;
use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello", web::get().to(greet));
}


