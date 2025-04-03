use actix_web::web;

use crate::application::controllers::organization_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/organizations")
            // First register specific routes like "search" or "batch"
            // to avoid conflicts with parameterized routes
            .service(organization_controller::find_organizations_by_name) // "/search" route
            .service(organization_controller::batch_create_organizations) // "/batch" route
            .service(organization_controller::batch_update_organizations) // "/batch" route
            .service(organization_controller::batch_delete_organizations) // "/batch" route
            .service(organization_controller::get_organizations_by_ids) // "/get-by-ids" route
            // Then register routes with path parameters
            .service(organization_controller::create_organization) // "" route
            .service(organization_controller::update_organization) // "/{id}" route
            .service(organization_controller::delete_organization) // "/{id}" route
            .service(organization_controller::get_organization) // "/{id}" route
            .service(organization_controller::list_organizations) // "" route (listing)
            .service(organization_controller::organization_exists) // "/{id}/exists" route
            .service(
                web::scope("/{org_id}/configs")
                    // For nested routes, register specific paths before parameter paths
                    // This ensures "/search" doesn't get mistaken for a config_id
                    .service(organization_controller::find_configs_by_key) // "/search" route
                    .service(organization_controller::batch_create_configs) // "/batch" route
                    .service(organization_controller::batch_update_configs) // "/batch" route
                    .service(organization_controller::batch_delete_configs) // "/batch" route
                    .service(organization_controller::get_configs_by_ids) // "/get-by-ids" route
                    // Then the parameterized routes
                    .service(organization_controller::create_config) // "" route
                    .service(organization_controller::update_config) // "/{config_id}" route
                    .service(organization_controller::delete_config) // "/{config_id}" route
                    .service(organization_controller::get_config) // "/{config_id}" route
                    .service(organization_controller::list_configs), // "" route (listing)
            ),
    );
}
