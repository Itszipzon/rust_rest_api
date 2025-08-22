use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
  pub username: String,
  pub email: String,
  pub password: String,
  pub terms: bool,
}
