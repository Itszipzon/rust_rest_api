use actix_multipart::Multipart;
use actix_web::web::{Data, Query};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use uuid::Uuid;

use crate::jwt::jwt::JwtManager;
use crate::repository::Repositories;
use crate::requests::create_app_request::CreateAppRequest;
use crate::{log, tools};

#[get("")]
async fn get_own_apps(
  req: HttpRequest,
  repo: Data<Repositories>,
  jwt: Data<JwtManager>,
) -> HttpResponse {
  let auth = req
    .headers()
    .get("Authorization")
    .and_then(|h| h.to_str().ok());

  let token = match auth.and_then(|h| h.strip_prefix("Bearer ")) {
    Some(t) => t,
    None => {
      return {
        log::debug("Missing or invalid token format");
        HttpResponse::Unauthorized().body("Missing or invalid token format")
      };
    }
  };

  let claims = match jwt.validate_token(token) {
    Ok(c) => c,
    Err(_) => {
      return {
        log::debug("Invalid token");
        HttpResponse::Unauthorized().body("Invalid token")
      };
    }
  };

  let user_id = claims.id;
  match repo.apps.get_apps_by_user_id(user_id).await {
    Ok(apps) => HttpResponse::Ok().json(apps),
    Err(e) => {
      log::debug(&format!("Failed to retrieve apps: {}", e));
      HttpResponse::InternalServerError().body(format!("Failed to retrieve apps: {}", e))
    },
  }
}

#[get("/{id}")]
async fn get_app_by_id(req: HttpRequest, repo: Data<Repositories>) -> HttpResponse {
  let id: Uuid = req
    .match_info()
    .get("id")
    .and_then(|id| id.parse().ok())
    .unwrap_or(Uuid::new_v4());

  match repo.apps.get_app_by_id(id).await {
    Ok(app) => HttpResponse::Ok().json(app.to_json()),
    Err(e) => {
      HttpResponse::InternalServerError().body(format!("Failed to retrieve app by ID: {}", e))
    }
  }
}

#[get("/user/{id}")]
async fn get_apps_by_user_id(req: HttpRequest, repo: Data<Repositories>) -> HttpResponse {
  let id: Uuid = req
    .match_info()
    .get("id")
    .and_then(|id| id.parse().ok())
    .unwrap_or(Uuid::new_v4());

  match repo.apps.get_apps_by_user_id(id).await {
    Ok(app) => HttpResponse::Ok().json(app),
    Err(e) => {
      log::debug(&format!("Failed to retrieve apps by user_id: {}", e));
      HttpResponse::InternalServerError().body(format!("Failed to retrieve apps by user_id: {}", e))
    }
  }
}

#[post("")]
async fn create_app(
  req: HttpRequest,
  repo: Data<Repositories>,
  jwt: Data<JwtManager>,
  mut payload: Multipart,
  query: Query<CreateAppRequest>,
) -> HttpResponse {
  let auth = req
    .headers()
    .get("Authorization")
    .and_then(|h| h.to_str().ok());

  let token = match auth.and_then(|h| h.strip_prefix("Bearer ")) {
    Some(t) => t,
    None => return HttpResponse::Unauthorized().body("Missing or invalid token format"),
  };

  let claims = match jwt.validate_token(token) {
    Ok(c) => c,
    Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
  };

  let user_id = claims.id;
  let name = query.name.clone();
  let description = query.description.clone();
  let github_url = query.github_url.clone();
  let image_name = tools::save_image(&mut payload, "app")
    .await
    .map_err(|e| HttpResponse::InternalServerError().body(format!("Failed to save image: {}", e)));

  let _ = repo
    .apps
    .add_app(
      &name,
      &description,
      Some(&github_url),
      &image_name.unwrap(),
      user_id,
    )
    .await
    .map_err(|e| HttpResponse::InternalServerError().body(format!("Failed to add app: {}", e)));

  HttpResponse::Ok().json(format!(
    "'{}' created successfully for user {}",
    name, claims.sub
  ))
}

pub fn scope() -> actix_web::Scope {
  web::scope("/api/apps")
    .service(get_app_by_id)
    .service(get_apps_by_user_id)
    .service(create_app)
    .service(get_own_apps)
}
