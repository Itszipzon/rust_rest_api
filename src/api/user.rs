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

    let user_row = match repo.user.get_user_id(1).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    println!("User {:#?}", user_row);

    let username: String = match user_row.try_get("username") {
        Ok(u) => u,
        Err(_) => return HttpResponse::InternalServerError().body("Missing username field"),
    };

    HttpResponse::Ok().json(serde_json::json!({ "username": username }))
}


pub fn scope() -> actix_web::Scope {
    web::scope("/api/user").service(get_user)
}
