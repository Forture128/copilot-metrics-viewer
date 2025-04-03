use crate::application::controllers::user_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // User management routes
    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(user_controller::user_login))
            .route("", web::post().to(user_controller::register_user))
            .route("/{id}", web::get().to(user_controller::get_user))
            .route(
                "/{id}/with-roles",
                web::get().to(user_controller::get_user_with_roles),
            )
            .route("/{id}", web::put().to(user_controller::update_user))
            .route(
                "/{id}/change-password",
                web::put().to(user_controller::change_password),
            )
            .route(
                "/{user_id}/roles/{role_id}",
                web::delete().to(user_controller::remove_role),
            ),
    );

    // User roles route
    cfg.route("/user-roles", web::post().to(user_controller::assign_role));

    // Organization users route
    cfg.route(
        "/users-orgs/{organization_id}/users",
        web::get().to(user_controller::list_users),
    );
}
