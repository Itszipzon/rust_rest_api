use std::sync::Arc;

use async_trait::async_trait;

use crate::repositories::{apps_repo::AppsRepo, user_repo::UserRepo};

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
        self.apps.create_table().await?;
        self.user.create_table().await?;
        Ok(())
    }
}