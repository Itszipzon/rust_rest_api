use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{jwt::jwt::JwtManager, repository::Repository};

#[derive(Clone)]
pub struct UserRepo {
    client: Arc<Mutex<Client>>,
}

impl UserRepo {
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        Self { client }
    }

    pub async fn get_user(&self, user_id: i32) -> Result<String, String> {
        let client = self.client.lock().await;

        let rows = client
            .query("SELECT name FROM users WHERE id = $1", &[&user_id])
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.get(0) {
            Ok(row.get(0))
        } else {
            Err("User not found".to_string())
        }
    }

    pub async fn user_login(&self, username: &str, password: &str) -> Result<String, String> {
        let client = self.client.lock().await;

        let rows = client
            .query("SELECT * FROM users WHERE username = $1", &[&username])
            .await
            .map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Err("Invalid username".to_string());
        }

        let row = &rows[0];

        let stored_password: String = row.get("password");

        let verify = bcrypt::verify(password, &stored_password);

        if verify.is_err() || !verify.unwrap() {
            return Err("Invalid password".to_string());
        }

        Ok("Login successful".to_string())
    }
}

#[async_trait]
impl Repository for UserRepo {}