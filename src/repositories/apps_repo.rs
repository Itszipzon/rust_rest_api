use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use async_trait::async_trait;

use crate::dberror::DbError;
use crate::repository::Repository;
use crate::tables::apps::Apps;

#[derive(Clone)]
pub struct AppsRepo {
    client: Arc<Mutex<Client>>,
}

impl AppsRepo {
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        Self { client }
    }
    
    pub async fn get_app_by_id(&self, id: i32) -> Result<Apps, DbError> {
        let client = self.client.lock().await;

        let rows = client
            .query("SELECT * FROM apps WHERE id = $1", &[&id])
            .await
            .map_err(DbError::from)?;

        if let Some(row) = rows.first() {
            let app = Apps::new(
                row.get("id"),
                row.get("name"),
                row.get("description"),
                row.get("created_at"),
                row.get("updated_at"),
                row.get("is_active"),
                row.get("image_name"),
                row.get("github_url"),
            );
            Ok(app)
        } else {
            Err(DbError::NotFound)
        }
    }

    pub async fn get_apps_by_user_id(&self, user_id: i32) -> Result<Vec<Apps>, DbError> {
        let client = self.client.lock().await;

        let rows = client
            .query("
            SELECT *
                FROM apps
                JOIN user ON apps.user_id = user.id
                WHERE user.id = $1",
            &[&user_id])
            .await
            .map_err(DbError::from)?;

        let mut apps = Vec::new();
        for row in rows {
            let app = Apps {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                github_url: row.get("github_url"),
                image_name: row.get("image_name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                is_active: row.get("is_active"),
            };
            apps.push(app);
        }
        Ok(apps)

        
    }

    pub async fn add_app(
        &self,
        name: &str,
        description: &str,
        github_url: Option<&str>,
        image_url: &str,
        user_id: i32) -> Result<(), DbError> {
        let client = self.client.lock().await;

        client
            .execute(
                "INSERT INTO apps (name, description, github_url, image_url, user_id) VALUES ($1, $2, $3, $4, $5)",
                &[&name, &description, &github_url, &image_url, &user_id],
            )
            .await
            .map_err(DbError::from)?;
        Ok(())
    }
}

#[async_trait]
impl Repository for AppsRepo {}
