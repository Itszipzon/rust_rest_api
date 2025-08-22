use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAppRequest {
  pub name: String,
  pub description: String,
  pub github_url: String,
}
