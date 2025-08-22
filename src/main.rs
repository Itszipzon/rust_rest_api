use actix_web::{App, HttpServer, web};
use db::DbPool;
use dotenv::dotenv;
use std::{env, sync::Arc};

use crate::{jwt::jwt::JwtManager, repositories::apps_repo::AppsRepo, repository::Repositories};

mod api;
mod db;
mod dberror;
mod jwt;
mod log;
mod repositories;
mod repository;
mod requests;
mod tables;
mod tools;

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
  log::info(&format!("Connecting to database {}", database_name), true);
  match DbPool::new(&database_url).await {
    Ok(pool) => {
      log::info("Connected to database successfully!", true);
      db_pool_data = web::Data::new(pool);
    }
    Err(e) => {
      log::error(&format!("Failed to connect to database: {}", e));
      std::process::exit(1);
    }
  }

  let repos: web::Data<Repositories> = web::Data::new(Repositories {
    apps: Arc::new(AppsRepo::new(db_pool_data.get_client())),
    user: Arc::new(repositories::user_repo::UserRepo::new(
      db_pool_data.get_client(),
    )),
  });

  let _ = repos.create_tables().await;

  let jwt_manager = web::Data::new(JwtManager::new());

  let server = match HttpServer::new(move || {
    App::new()
      .app_data(db_pool_data.clone())
      .app_data(repos.clone())
      .app_data(jwt_manager.clone())
      .service(api::apps::scope())
      .service(api::user::scope())
  })
  .bind(("127.0.0.1", 8080))
  {
    Ok(srv) => {
      log::info("Server running at port 8080", true);
      srv
    }
    Err(e) => {
      log::error(&format!("Failed to bind to port 8080: {}", e));
      std::process::exit(1);
    }
  };

  server.run().await
}
