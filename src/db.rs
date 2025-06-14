use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;
use tokio_postgres::Error as PgError;
use tokio_postgres::{Client, NoTls};

use crate::dberror::DbError;
use crate::tools;

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


        for (_, statement) in statements.enumerate() {
            println!("Trying to create table: {}", tools::table_name_from_statement(statement));
            client.execute(statement, &[]).await?;
        }

        let user_insert = "INSERT INTO users (username, email, password) VALUES ($1, $2, $3);";

        let password = bcrypt::hash("12345678", bcrypt::DEFAULT_COST)
            .map_err(|e| DbError::HashingError(e.to_string()))?;

        println!("Inserting default user...");
        client
            .execute(user_insert, &[&"admin", &"rune.molander@hotmail.com", &password])
            .await?;

        Ok(())
    }
}
