use actix_web::web;

use crate::application::controllers::department_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/departments")
            .service(department_controller::create_department)
            .service(department_controller::get_department)
            .service(department_controller::update_department)
            .service(department_controller::delete_department)
            .service(department_controller::get_department_users)
            .service(department_controller::batch_add_users_to_department)
            .service(department_controller::remove_user_from_department)
            .service(department_controller::list_departments)
            .service(department_controller::add_user_to_department)
            .service(department_controller::remove_user_from_department)
            .service(department_controller::list_all_departments),
    );

    // Organization-scoped department routes
    cfg.service(
        web::scope("/organizations")
            .service(department_controller::list_departments)
            .service(department_controller::add_user_to_department),
    );
}
