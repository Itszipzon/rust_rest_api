use actix_multipart::Multipart;
use futures_util::StreamExt;
use mime_guess::get_mime_extensions_str;
use std::io::Write;
use uuid::Uuid;

pub fn table_name_from_statement(statement: &str) -> &str {
  let protected_statement_words = [
    "CREATE",
    "TABLE",
    "INDEX",
    "IF",
    "NOT",
    "EXISTS",
    "TEMPORARY",
    "TEMP",
    "UNIQUE",
    "PRIMARY",
    "KEY",
    "FOREIGN",
    "REFERENCES",
    "ON",
    "CONSTRAINT",
    "CHECK",
  ];
  let mut parts = statement.split_whitespace();
  while let Some(part) = parts.next() {
    if protected_statement_words.contains(&part.to_uppercase().as_str()) {
      continue;
    }
    if part.contains('(') {
      return part.split('(').next().unwrap_or("Unknown");
    }
    return part;
  }
  "Unknown"
}

pub fn is_valid_username(username: &str) -> bool {
  let len = username.len();

  if len < 3 || len > 20 {
    return false;
  }

  if username.contains('@') || username.contains('\\') || username.contains('/') {
    return false;
  }

  true
}

pub fn is_valid_email(email: &str) -> bool {
  let len = email.len();

  if len < 5 || len > 254 {
    return false;
  }

  if email.contains(' ') || email.contains('\\') || email.contains('/') {
    return false;
  }

  let parts: Vec<&str> = email.split('@').collect();
  if parts.len() != 2 {
    return false;
  }

  let (local, domain) = (parts[0], parts[1]);

  if local.is_empty() || domain.is_empty() {
    return false;
  }

  if !domain.contains('.') {
    return false;
  }

  true
}

pub async fn save_image(image: &mut Multipart, image_type: &str) -> Result<String, String> {
  let mut image_name = String::new();

  while let Some(item) = image.next().await {
    let mut field = match item {
      Ok(f) => f,
      Err(_) => return Err("Error reading multipart field".to_string()),
    };

    let content_disposition = field.content_disposition();
    let field_name = content_disposition.unwrap().get_name().unwrap_or("");

    if field_name == "image" {
      let original_filename = content_disposition
        .unwrap()
        .get_filename()
        .unwrap_or("file");
      let content_type = field.content_type().unwrap().to_string();

      let extension = get_mime_extensions_str(&content_type)
        .and_then(|exts| exts.first().map(|e| format!(".{}", e)))
        .unwrap_or_default();

      let new_filename = format!("{}{}", Uuid::new_v4(), extension);
      println!("Original filename: {original_filename}, New: {new_filename}, Type: {content_type}");

      // Save file
      let filepath = format!("./media/images/{}/{}", image_type, new_filename);
      let mut f = std::fs::File::create(&filepath).unwrap();
      while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f.write_all(&data).unwrap();
      }

      image_name = new_filename;
    }
  }
  Ok(image_name)
}
