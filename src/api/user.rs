use actix_web::{
    HttpRequest, HttpResponse, get, post,
    web::{self, Data, Json},
};
use chrono::{DateTime, Utc};

use crate::{jwt::jwt::JwtManager, repository::Repositories, user::login_request::LoginRequest};

#[get("")]
async fn get_user(
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
        None => return HttpResponse::Unauthorized().body("Missing or invalid token format"),
    };

    let claims = match jwt.validate_token(token) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let user_row = match repo.user.get_user_id(claims.id).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    let id: i32 = match user_row.try_get("id") {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().body("Missing id field"),
    };

    let username: String = match user_row.try_get("username") {
        Ok(u) => u,
        Err(_) => return HttpResponse::InternalServerError().body("Missing username field"),
    };

    let email = match user_row.try_get::<&str, String>("email") {
        Ok(e) => e,
        Err(_) => return HttpResponse::InternalServerError().body("Missing email field"),
    };

    let created_at: DateTime<Utc> = match user_row.try_get("created_at") {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Missing created_at field for user {}", e)),
    };

    let terms: bool = match user_row.try_get("accepted_terms") {
        Ok(t) => t,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Missing terms field for user {}", e)),
    };

    let is_admin: bool = match user_row.try_get("is_admin") {
        Ok(a) => a,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Missing is_admin field for user {}", e)),
    };

    let last_logged_in: Option<DateTime<Utc>> = match user_row.try_get("last_login_at") {
        Ok(l) => l,
        Err(_) => None,
    };

    let last_logged_in = last_logged_in.map_or_else(
        || None,
        |l| Some(l.to_rfc3339()),
    );

    HttpResponse::Ok().json(serde_json::json!({
        "id": id,
        "username": username,
        "email": email,
        "created_at": created_at.to_rfc3339(),
        "last_logged_in": last_logged_in,
        "terms": terms,
        "is_admin": is_admin
    }))
}

#[post("login")]
async fn user_login(repo: Data<Repositories>, payload: Json<LoginRequest>, jwt: Data<JwtManager>) -> HttpResponse {

    let user_row = match repo.user.get_user_username(&payload.username).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    let password: String = match user_row.try_get("password") {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().body("Missing password field"),
    };

    if !(bcrypt::verify(&payload.password, &password)).unwrap() {
        return HttpResponse::Unauthorized().body("Username or password is incorrect");
    }

    let id: i32 = match user_row.try_get("id") {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().body("User not found"),
    };

    let token = match jwt.generate_token(&payload.username, id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to generate token"),
    };

    HttpResponse::Ok().body(token)
}

pub fn scope() -> actix_web::Scope {
    web::scope("/api/users")
        .service(get_user)
        .service(user_login)
}
