use actix_web::{get, post, web, HttpResponse};
use actix_web::web::Data;

use crate::repository::Repositories;

#[get("")]
async fn get_apps(repo: Data<Repositories>) -> HttpResponse {
  match repo.apps.get_apps().await {
      Ok(vessel_name) => HttpResponse::Ok().json(serde_json::json!({ "name": vessel_name })),
      Err(e) => HttpResponse::InternalServerError().body(
          format!("Failed to retrieve vessel: {}", e)
      ),
  }
}

#[post("")]
async fn create_app(repo: Data<Repositories>, app_data: web::Json<serde_json::Value>) -> HttpResponse {
    let name = app_data.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let description = app_data.get("description").and_then(|v| v.as_str()).unwrap_or("");
    let link = app_data.get("link").and_then(|v| v.as_str()).unwrap_or("");
    let image_url = app_data.get("image_url").and_then(|v| v.as_str()).unwrap_or("");

    match repo.apps.add_app(name, description, link, image_url).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create app: {}", e)),
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/api/apps")
        .service(get_apps)
        /* .service(create_vessel) */
}
