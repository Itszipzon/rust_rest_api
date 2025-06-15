use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{repository::Repository, tables::user::User};

#[derive(Clone)]
pub struct UserRepo {
    client: Arc<Mutex<Client>>,
}

impl UserRepo {
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        Self { client }
    }

    pub async fn get_user_id(&self, user_id: i32) -> Result<User, String> {
        let client = self.client.lock().await;
    
        let rows = client
            .query("SELECT * FROM users WHERE id = $1", &[&user_id])
            .await
            .map_err(|e| e.to_string())?;
    
        if let Some(row) = rows.into_iter().next() {
            let id: i32 = row.get("id");
            let username: String = row.get("username");
            let email: String = row.get("email");
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            let last_logged_in: Option<chrono::DateTime<chrono::Utc>> = row.get("last_login_at");
            let terms: bool = row.get("accepted_terms");
            let is_admin: bool = row.get("is_admin");
            let password: Option<String> = None;

            Ok(User::new(
                id,
                username,
                email,
                created_at,
                last_logged_in,
                terms,
                is_admin,
                password,
            ))
        } else {
            Err("User not found".to_string())
        }
    }

    pub async fn get_user_username_authentication(&self, username: &str) -> Result<User, String> {
        let client = self.client.lock().await;

        let rows = client
            .query("SELECT * FROM users WHERE username = $1 OR email = $1", &[&username])
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.into_iter().next() {
            let id: i32 = row.get("id");
            let username: String = row.get("username");
            let email: String = row.get("email");
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            let last_logged_in: Option<chrono::DateTime<chrono::Utc>> = row.get("last_login_at");
            let terms: bool = row.get("accepted_terms");
            let is_admin: bool = row.get("is_admin");
            let password: Option<String> = row.get("password");

            Ok(User::new(
                id,
                username,
                email,
                created_at,
                last_logged_in,
                terms,
                is_admin,
                password,
            ))
        } else {
            Err("User not found".to_string())
        }
    }
}

#[async_trait]
impl Repository for UserRepo {}