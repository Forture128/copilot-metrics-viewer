use crate::application::controllers::auth_controller;
use actix_files::NamedFile;
use actix_web::web;
use std::path::PathBuf;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/me", web::get().to(auth_controller::auth_me))
            .route("/login", web::post().to(auth_controller::auth_login))
            .route("/token-helper", web::get().to(token_helper)),
    );
}

async fn token_helper() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "src/application/static/token-helper.html".into();
    Ok(NamedFile::open(path)?)
}
