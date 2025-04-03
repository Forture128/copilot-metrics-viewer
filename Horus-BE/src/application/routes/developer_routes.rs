use crate::application::controllers::developer_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/developer")
            .service(developer_controller::get_developer_metrics)
            .service(developer_controller::get_developer_commits)
            .service(developer_controller::get_developer_pull_requests),
    );
}
