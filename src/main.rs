use actix_web::{App, HttpServer, web};
use db::DbPool;
use dotenv::dotenv;
use std::{env, sync::Arc};

use crate::{jwt::jwt::JwtManager, repositories::apps_repo::AppsRepo, repository::Repositories};

mod api;
mod db;
mod repositories;
mod dberror;
mod repository;
mod jwt;
mod tools;
mod user;
mod requests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME not set in .env");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD not set in .env");
    let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST not set in .env");
    let database_port = env::var("DATABASE_PORT").expect("DATABASE_PORT not set in .env");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set in .env");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_username, database_password, database_host, database_port, database_name
    );

    let db_pool_data;
    println!("Connecting to database at {}", database_url);
    match DbPool::new(&database_url).await {
        Ok(pool) => {
            println!("Connected to database successfully!");
            db_pool_data = web::Data::new(pool);
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    }

    let repos = web::Data::new(Repositories {
        apps: Arc::new(AppsRepo::new(db_pool_data.get_client())),
        user: Arc::new(repositories::user_repo::UserRepo::new(db_pool_data.get_client())),
    });

    let jwt_manager = web::Data::new(JwtManager::new());

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .app_data(repos.clone())
            .service(api::apps::scope())
            .service(api::user::scope())
            .app_data(jwt_manager.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
