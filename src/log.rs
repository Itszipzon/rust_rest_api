use std::io::Write;
use std::{fs::File, path::PathBuf};

/// Logs error messages to a log file and prints them to the console
pub fn error(message: &str) {
  add_log_to_file(message, "ERROR");
}

/// Logs informational messages to a log file if log is true and prints them to the console
pub fn info(message: &str, log: bool) {
  if log {
    add_log_to_file(message, "INFO");
  } else {
    let stdout = std::io::stdout();
    let mut handle = std::io::BufWriter::new(stdout.lock());
    let date_time = chrono::Local::now().format("%d-%b-%Y %H:%M:%S").to_string();
    writeln!(handle, "[{}] [{}]: {}", date_time, "INFO", message).unwrap();
  }
}

pub fn debug(message: &str) {
  add_log_to_file(message, "DEBUG");
}

pub fn warn(message: &str) {
  add_log_to_file(message, "WARNING");
}

fn add_log_to_file(message: &str, level: &str) {
  let dir: PathBuf = std::env::current_dir()
    .expect("Failed to get current directory")
    .parent()
    .expect("Failed to get parent directory")
    .join("logs");

  let stdout = std::io::stdout();
  let mut handle = std::io::BufWriter::new(stdout.lock());

  std::fs::create_dir_all(&dir).expect("Directory creation failed");

  let date = chrono::Local::now().format("%d-%b-%Y").to_string();
  let file_path: PathBuf = dir.join(format!("{}.log", date));
  let mut file: File = std::fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_path)
    .expect("Failed to open log file");

  let date_time = chrono::Local::now().format("%d-%b-%Y %H:%M:%S").to_string();
  writeln!(handle, "[{}] [{}]: {}", date_time, level, message).unwrap();

  writeln!(file, "[{}] [{}]: {}", date_time, level, message).expect("Failed to write to log file");
}
