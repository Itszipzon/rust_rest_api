use actix_web::{
    HttpRequest, HttpResponse, get,
    web::{self, Data},
};

use crate::{jwt::jwt::JwtManager, repository::Repositories};

#[get("")]
async fn get_user(req: HttpRequest, repo: Data<Repositories>, jwt: Data<JwtManager>) -> HttpResponse {
    let auth = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token format"),
    };

    match jwt.validate_token(token) {
        Ok(claims) => HttpResponse::Ok().body(format!("Welcome, {}!", claims.sub)),
        Err(_) => HttpResponse::Unauthorized().body("Token invalid or expired"),
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/api/apps").service(get_user)
}
