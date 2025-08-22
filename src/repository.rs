use std::sync::Arc;

use async_trait::async_trait;

use crate::{log, repositories::{apps_repo::AppsRepo, user_repo::UserRepo}};

#[async_trait]
#[allow(dead_code)]
pub trait Repository: Send + Sync {
  async fn create_table(&self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct Repositories {
  pub apps: Arc<AppsRepo>,
  pub user: Arc<UserRepo>,
}

impl Repositories {
  pub async fn create_tables(&self) -> Result<(), String> {
    let _ = self.apps.create_table().await.map_err(|e| {
      log::warn(&format!("Failed to create apps table: {}", e));
    });
    let _ = self.user.create_table().await.map_err(|e| {
      log::warn(&format!("Failed to create user table: {}", e));
    });
    Ok(())
  }
}
