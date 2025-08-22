use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::dberror::DbError;
use crate::log;
use crate::repository::Repository;
use crate::tables::apps::Apps;
use crate::tools::table_name_from_statement;

#[derive(Clone)]
pub struct AppsRepo {
  client: Arc<Mutex<Client>>,
}

impl AppsRepo {
  pub fn new(client: Arc<Mutex<Client>>) -> Self {
    Self { client }
  }

  pub async fn get_app_by_id(&self, id: Uuid) -> Result<Apps, DbError> {
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

  pub async fn get_apps_by_user_id(&self, user_id: Uuid) -> Result<Vec<Apps>, DbError> {
    let client = self.client.lock().await;

    let rows = client
      .query(
        "
            SELECT *
                FROM apps
                JOIN users ON apps.user_id = users.id
                WHERE users.id = $1",
        &[&user_id],
      )
      .await
      .map_err(|e| {
        log::error(&format!("Failed to query apps by user_id: {}", e));
        DbError::from(e)
      })?;

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
    user_id: Uuid,
  ) -> Result<(), DbError> {
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
impl Repository for AppsRepo {
  async fn create_table(&self) -> Result<(), String> {
    let client = self.client.lock().await;

    let statement: &str = "CREATE TABLE IF NOT EXISTS apps (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    name VARCHAR(255) NOT NULL,
                    description TEXT,
                    github_url VARCHAR(255),
                    image_name VARCHAR(255),
                    user_id UUID NOT NULL REFERENCES users(id),
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    is_active BOOLEAN DEFAULT TRUE
                )";

    client
      .execute(statement, &[])
      .await
      .map_err(|e| e.to_string())?;
    log::info(
      format!("Created table {}", table_name_from_statement(&statement)).as_str(),
      true,
    );
    Ok(())
  }
}
