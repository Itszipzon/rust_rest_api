use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub id: Uuid,
  pub exp: usize,
}
