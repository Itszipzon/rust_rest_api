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