use serde::Serialize;


#[derive(Serialize)]
pub struct Apps {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
  pub is_active: bool,
  pub image_url: String,
  pub github_url: Option<String>,
}

impl Apps {
  pub fn new(
    id: i32,
    name: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    is_active: bool,
    image_url: String,
    github_url: Option<String>,
  ) -> Self {
    Apps {
      id,
      name,
      description,
      created_at,
      updated_at,
      is_active,
      image_url,
      github_url,
    }
  }

  pub fn to_json(&self) -> serde_json::Value {
    serde_json::json!({
      "id": self.id,
      "name": self.name,
      "description": self.description,
      "created_at": self.created_at.to_rfc3339(),
      "updated_at": self.updated_at.to_rfc3339(),
      "is_active": self.is_active,
      "image_url": self.image_url,
      "github_url": self.github_url
    })
  }
}