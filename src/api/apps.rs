use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::Data;

use crate::repository::Repositories;

#[get("/{id}")]
async fn get_app_by_id(req: HttpRequest, repo: Data<Repositories>) -> HttpResponse {
    let id: i32 = req.match_info().get("id")
        .and_then(|id| id.parse().ok())
        .unwrap_or(0);

    match repo.apps.get_app_by_id(id).await {
        Ok(app) => HttpResponse::Ok().json(app),
        Err(e) => HttpResponse::InternalServerError().body(
            format!("Failed to retrieve app by ID: {}", e)
        ),
    }
}

#[get("/user/{id}")]
async fn get_apps_by_user_id(req: HttpRequest, repo: Data<Repositories>) -> HttpResponse {
    let id: i32 = req.match_info().get("id")
        .and_then(|id| id.parse().ok())
        .unwrap_or(0);

    match repo.apps.get_apps_by_user_id(id).await {
        Ok(app) => HttpResponse::Ok().json(app),
        Err(e) => HttpResponse::InternalServerError().body(
            format!("Failed to retrieve app by ID: {}", e)
        ),
    }
}

#[post("")]
async fn create_app(repo: Data<Repositories>, ) -> HttpResponse {
    HttpResponse::NotImplemented().body("Create app endpoint not implemented yet")
}

pub fn scope() -> actix_web::Scope {
    web::scope("/api/apps")
        .service(get_app_by_id)
        .service(get_apps_by_user_id)
        .service(create_app)
}
