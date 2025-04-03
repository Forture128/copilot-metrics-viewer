use crate::application::controllers::collaboration_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/collaboration")
            .service(collaboration_controller::get_collaboration_metrics)
            .service(collaboration_controller::get_code_review_stats),
    );
}
