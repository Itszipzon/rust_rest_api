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

/*     let token = match auth.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token format"),
    }; */

    let user_row = match repo.user.get_user_id(1).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    println!("{:#?}", user_row);

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

    let created_at: String = match user_row.try_get("created_at") {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Missing created_at field"),
    };

    let last_logged_in: Option<String> = match user_row.try_get("last_login_at") {
        Ok(l) => l,
        Err(_) => None,
    };

    HttpResponse::Ok().json(serde_json::json!({
        "id": id,
        "username": username,
        "email": email,
        "created_at": created_at,
        "last_logged_in": last_logged_in,
    }))
}


pub fn scope() -> actix_web::Scope {
    web::scope("/api/users").service(get_user)
}
