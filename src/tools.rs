pub fn table_name_from_statement(statement: &str) -> String {
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
            return part.split('(').next().unwrap_or("Unknown").to_string();
        }
        return part.to_string();
    }
    "Unknown".to_string()
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
