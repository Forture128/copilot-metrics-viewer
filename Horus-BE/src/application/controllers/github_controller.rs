use crate::application::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json;
use tracing::{error, info};

#[get("/repos")]
pub async fn list_repos(data: web::Data<AppState>) -> impl Responder {
    info!("Fetching user repositories");
    match data.github.list_user_repos().await {
        Ok(repos) => {
            info!("Successfully fetched {} repositories", repos.len());
            HttpResponse::Ok().json(repos)
        }
        Err(e) => {
            error!("GitHub API error: {:?}", e);
            error!("Error details: {:#?}", e);
            let error_message = format!("Failed to fetch repositories: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error_message,
                "details": e.to_string()
            }))
        }
    }
}

#[get("/{owner}/{repo}")]
pub async fn get_repo(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    info!("Fetching repository {}/{}", owner, repo);

    match data.github.get_repo(&owner, &repo).await {
        Ok(repo) => {
            info!("Successfully fetched repository");
            HttpResponse::Ok().json(repo)
        }
        Err(e) => {
            error!("GitHub API error: {:?}", e);
            error!("Error details: {:#?}", e);
            let error_message = format!("Failed to fetch repository: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error_message,
                "details": e.to_string()
            }))
        }
    }
}
