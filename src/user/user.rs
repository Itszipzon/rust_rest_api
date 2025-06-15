use chrono::{DateTime, Utc};

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_logged_in: Option<DateTime<Utc>>,
    pub terms: bool,
    pub is_admin: bool,
    pub password: Option<String>,
}

impl User {
    pub fn new(
        id: i32,
        username: String,
        email: String,
        created_at: DateTime<Utc>,
        last_logged_in: Option<DateTime<Utc>>,
        terms: bool,
        is_admin: bool,
        password: Option<String>,
    ) -> Self {
        User {
            id,
            username,
            email,
            created_at,
            last_logged_in,
            terms,
            is_admin,
            password,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "username": self.username,
            "email": self.email,
            "created_at": self.created_at.to_rfc3339(),
            "last_logged_in": self.last_logged_in.map(|l| l.to_rfc3339()),
            "terms": self.terms,
            "is_admin": self.is_admin,
            "password": self.password
        })
    }
}
