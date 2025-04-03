use crate::application::controllers::delivery_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/delivery").service(delivery_controller::get_delivery_metrics));
}
