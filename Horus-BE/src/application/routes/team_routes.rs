use crate::application::controllers::team_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Global team routes
    cfg.service(
        web::scope("/teams")
            .route("", web::post().to(team_controller::create_team))
            .route("/{id}", web::get().to(team_controller::get_team))
            .route("/{id}", web::put().to(team_controller::update_team))
            .route("/{id}", web::delete().to(team_controller::delete_team))
            .route(
                "/{team_id}/members",
                web::post().to(team_controller::add_team_member),
            )
            .route(
                "/{team_id}/members/{user_id}",
                web::delete().to(team_controller::remove_team_member),
            )
            .route(
                "/{team_id}/members",
                web::get().to(team_controller::get_team_with_members),
            ),
    );

    // Organization-scoped team routes
    cfg.service(
        web::scope("/teams-org/{organization_id}")
            .route("/teams", web::get().to(team_controller::list_teams))
            .route(
                "/teams/by-department/{department_id}",
                web::get().to(team_controller::list_teams_by_department),
            ),
    );
}
