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

    pub async fn get_user_id(&self, user_id: i32) -> Result<tokio_postgres::Row, String> {
        let client = self.client.lock().await;
    
        let rows = client
            .query("SELECT * FROM users WHERE id = $1", &[&user_id])
            .await
            .map_err(|e| e.to_string())?;
    
        rows.into_iter().next().ok_or("User not found".to_string())
    }

    pub async fn get_user_username(&self, username: &str) -> Result<String, String> {
        let client = self.client.lock().await;

        let rows = client
            .query("SELECT * FROM users WHERE username = $1", &[&username])
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.get(0) {
            Ok(row.get(0))
        } else {
            Err("Invalid username".to_string())
        }
    }
}

#[async_trait]
impl Repository for UserRepo {}