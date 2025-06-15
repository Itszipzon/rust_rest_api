use actix_web::{
    HttpRequest, HttpResponse, get, post,
    web::{self, Data, Json},
};
use chrono::{DateTime, Utc};

use crate::{jwt::jwt::JwtManager, repository::Repositories, requests::login_request::LoginRequest};

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

    HttpResponse::Ok().json(user_row.to_json())
}

#[post("login")]
async fn user_login(repo: Data<Repositories>, payload: Json<LoginRequest>, jwt: Data<JwtManager>) -> HttpResponse {

    let user_row = match repo.user.get_user_username_authentication(&payload.username).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    let password: String = user_row.password.unwrap();

    if !(bcrypt::verify(&payload.password, &password)).unwrap() {
        return HttpResponse::Unauthorized().body("Username or password is incorrect");
    }

    let id: i32 = user_row.id;

    let token = match jwt.generate_token(&user_row.username, id) {
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
