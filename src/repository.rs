use std::sync::Arc;

use async_trait::async_trait;

use crate::repositories::{apps_repo::AppsRepo, user_repo::UserRepo};

#[async_trait]
#[allow(dead_code)]
pub trait Repository: Send + Sync {}

#[derive(Clone)]
pub struct Repositories {
    pub apps: Arc<AppsRepo>,
    pub user: Arc<UserRepo>,
}