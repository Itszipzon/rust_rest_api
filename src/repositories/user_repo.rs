use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::repository::Repository;

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

        if let Some(row) = rows.get(0) {
            Ok(row.get(0))
        } else {
            Err("Invalid username or password".to_string())
        }
    }
}

#[async_trait]
impl Repository for UserRepo {}