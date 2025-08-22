use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Error as PgError;
use tokio_postgres::{Client, NoTls};

use crate::log;

#[derive(Clone)]
pub struct DbPool {
  client: Arc<Mutex<Client>>,
}

impl DbPool {
  // Initialize a new database connection
  pub async fn new(connection_string: &str) -> Result<DbPool, PgError> {
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

    tokio::spawn(async move {
      if let Err(e) = connection.await {
        log::error(&format!("Connection error: {}", e));
      }
    });

    log::info("Database connection established successfully!", true);

    Ok(DbPool {
      client: Arc::new(Mutex::new(client)),
    })
  }

  pub fn get_client(&self) -> Arc<Mutex<Client>> {
    self.client.clone()
  }
}
