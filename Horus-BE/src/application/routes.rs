use crate::application::controllers::github_controller;
use crate::application::controllers::proxy_controller;
use crate::constants::routes::API_V1_PREFIX;
use crate::{application::AppState, config::RunMode};
use actix_web::{guard, web, HttpResponse, Responder};

mod auth_routes;
mod collaboration_routes;
mod delivery_routes;
mod department_routes;
mod developer_routes;
mod organization_routes;
mod role_routes;
mod team_routes;
mod user_routes;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::scope("/health").route("", web::get().to(health_check)))
            .service(
                web::scope("/proxy").route(
                    "/{tail:.*}",
                    web::route()
                        .guard(
                            guard::Any(guard::Get())
                                .or(guard::Post())
                                .or(guard::Put())
                                .or(guard::Delete())
                                .or(guard::Head())
                                .or(guard::Options())
                                .or(guard::Patch()),
                        )
                        .to(proxy_controller::catch_all),
                ),
            )
            .service(
                web::scope(API_V1_PREFIX)
                    .configure(organization_routes::config)
                    .configure(developer_routes::config)
                    .configure(collaboration_routes::config)
                    .configure(delivery_routes::config)
                    .configure(department_routes::config)
                    .configure(role_routes::config)
                    .configure(team_routes::config)
                    .configure(user_routes::config)
                    // Auth routes
                    .configure(auth_routes::config)
                    .service(
                        web::scope("/github")
                            .service(github_controller::list_repos)
                            .service(github_controller::get_repo),
                    ),
            ),
    );
}

pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let config_status = match data.config.env {
        RunMode::Development => "Development",
        RunMode::Production => "Production",
        RunMode::Test => "Test",
    };

    let health_info = serde_json::json!({
        "status": "OK",
        "timestamp": chrono::Utc::now().timestamp(),
        "environment": config_status,
        "config": {
            "server": {
                "address": data.config.server.address,
                "workers": data.config.server.workers
            },
            "database_configured": !data.config.database.url.is_empty(),
            "redis_configured": !data.config.redis.url.is_empty()
        }
    });

    HttpResponse::Ok().json(health_info)
}
