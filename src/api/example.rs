/* pub struct Example {
    pub name: String,
    pub description: String,
}

impl Example {
    pub fn new(name: String, description: String) -> Self {
        Example { name, description }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "description": self.description,
        })
    }
} */