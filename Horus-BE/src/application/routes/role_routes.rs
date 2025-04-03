use crate::application::controllers::role_controller;
use actix_web::{web, HttpResponse};

// Simple debug handler
async fn debug_handler() -> HttpResponse {
    HttpResponse::Ok().json("Debug endpoint is working!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // Global role routes
    cfg.service(
        web::scope("/roles")
            .route("", web::post().to(role_controller::create_role))
            .route("/{id}", web::get().to(role_controller::get_role))
            .route("/{id}", web::put().to(role_controller::update_role))
            .route("/{id}", web::delete().to(role_controller::delete_role)),
    );

    // Organization-scoped role routes
    cfg.service(
        web::scope("/roles-org/{organization_id}")
            .route("/roles", web::get().to(role_controller::list_roles))
            .route(
                "/department-roles",
                web::post().to(role_controller::assign_department_role),
            )
            // Add a debug endpoint
            .route("/debug", web::get().to(debug_handler)),
    );
}
