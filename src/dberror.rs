use std::fmt;
use tokio_postgres::Error as PgError;


#[derive(Debug)]
pub enum DbError {
    NotFound,
    DatabaseError(PgError),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DbError::NotFound => write!(f, "Not found"),
            DbError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
        }
    }
}

impl From<PgError> for DbError {
    fn from(err: PgError) -> Self {
        DbError::DatabaseError(err)
    }
}