use actix_web::{
    HttpRequest, HttpResponse, get, post,
    web::{self, Data, Json},
};

use crate::{
    jwt::jwt::JwtManager,
    repository::Repositories,
    requests::{login_request::LoginRequest, register_request::RegisterRequest},
    tools::{is_valid_email, is_valid_username},
};

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
async fn user_login(
    repo: Data<Repositories>,
    payload: Json<LoginRequest>,
    jwt: Data<JwtManager>,
) -> HttpResponse {
    let user_row = match repo
        .user
        .get_user_username_authentication(&payload.username)
        .await
    {
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

#[post("register")]
async fn user_register(repo: Data<Repositories>, payload: Json<RegisterRequest>) -> HttpResponse {
    if !is_valid_username(&payload.username) {
        return HttpResponse::BadRequest().body("Invalid username");
    }

    if !is_valid_email(&payload.email) {
        return HttpResponse::BadRequest().body("Invalid email");
    }

    if payload.password.len() < 8 {
        return HttpResponse::BadRequest().body("Password must be at least 8 characters long");
    }

    if !payload.terms {
        return HttpResponse::BadRequest().body("You must accept the terms and conditions");
    }

    let hashed_password = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    if repo
        .user
        .user_exists_by_username(&payload.username)
        .await
        .unwrap_or(false)
    {
        return HttpResponse::Conflict().body("Username already exists");
    }

    if repo
        .user
        .user_exists_by_email(&payload.email)
        .await
        .unwrap_or(false)
    {
        return HttpResponse::Conflict().body("Email already exists");
    }

    match repo
        .user
        .register_user(
            payload.username.clone(),
            payload.email.clone(),
            hashed_password,
            payload.terms,
        )
        .await
    {
        Ok(_) => HttpResponse::Created().body("User registered successfully"),
        Err(e) => {
            if e.contains("duplicate key value violates unique constraint") {
                HttpResponse::Conflict().body("Username or email already exists")
            } else {
                HttpResponse::InternalServerError().body("Failed to register user")
            }
        }
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/api/users")
        .service(get_user)
        .service(user_login)
        .service(user_register)
}
