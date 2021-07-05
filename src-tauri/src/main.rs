
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};
use std::vec;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    first: String,
    last: String
}

#[tauri::command]
fn my_custom_command() -> Vec<String> {
  let mut rdr = csv::Reader::from_path("src/data.csv").unwrap();
  let mut v = Vec::new();

  for result in rdr.deserialize() {
    let record: Record = result.unwrap();
    let serialized = serde_json::to_string(&record).unwrap();

    v.push(serialized);
  }
  v
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
