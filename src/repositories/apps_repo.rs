use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use async_trait::async_trait;

use crate::dberror::DbError;
use crate::repository::Repository;

#[derive(Clone)]
pub struct AppsRepo {
    client: Arc<Mutex<Client>>,
}

impl AppsRepo {
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        Self { client }
    }

    pub async fn get_apps(&self) -> Result<String, DbError> {
        let client = self.client.lock().await;

        let rows = client
            .query("SELECT * FROM apps", &[])
            .await
            .map_err(DbError::from)?;

        if let Some(row) = rows.get(0) {
            Ok(row.get(0))
        } else {
            Err(DbError::NotFound)
        }
    }

    pub async fn add_app(
        &self,
        name: &str,
        description: &str,
        link: &str,
        image_url: &str) -> Result<(), DbError> {
        let client = self.client.lock().await;

        client
            .execute(
                "INSERT INTO apps (name, description, link, image_url) VALUES ($1, $2, $3, $4)",
                &[&name, &description, &link, &image_url],
            )
            .await
            .map_err(DbError::from)?;
        Ok(())
    }
}

#[async_trait]
impl Repository for AppsRepo {}
