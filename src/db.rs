use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;
use tokio_postgres::Error as PgError;
use tokio_postgres::{Client, NoTls};

use crate::dberror::DbError;

#[derive(Clone)]
pub struct DbPool {
    client: Arc<Mutex<Client>>,
}

impl DbPool {
    // Initialize a new database connection
    pub async fn new(connection_string: &str) -> Result<DbPool, PgError> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        println!("Connected to the database at {}", connection_string);
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        // THEN run queries
        let _ = DbPool::initiate_tables(&client).await;

        println!("Database connection established successfully!");

        Ok(DbPool {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn get_client(&self) -> Arc<Mutex<Client>> {
        self.client.clone()
    }

    async fn initiate_tables(client: &Client) -> Result<(), DbError> {

        let path = Path::new("database/tables.sql");

        let tables = fs::read_to_string(path).await;

        println!("Reading SQL file...");

        let statements = tables
            .as_ref()
            .unwrap()
            .split(';')
            .map(str::trim)
            .filter(|stmt| !stmt.is_empty());

        for (i, statement) in statements.enumerate() {
            println!("Executing SQL statement {}:\n{}", i + 1, statement);
            client.execute(statement, &[]).await?;
        }

        Ok(())
    }
}
