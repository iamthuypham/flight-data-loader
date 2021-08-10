#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![allow(
    // Clippy bug: https://github.com/rust-lang/rust-clippy/issues/7422
    clippy::nonstandard_macro_braces,
)]

use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use std::fs::File;
use std::sync::Arc;
use arrow::json::writer::record_batches_to_json_rows;
use serde_json::to_string;
use tauri::async_runtime::{channel, spawn};

#[tauri::command]
async fn my_custom_command(window: tauri::Window) {
  let schema = Schema::new(vec![
    Field::new("exp_time", DataType::Utf8, false),
    Field::new("utc_time", DataType::Utf8, false),
    Field::new("lat_long_1", DataType::Utf8, false),
  ]);

  let file = File::open("src/data.csv").unwrap();

  let mut csv = csv::Reader::new(file, Arc::new(schema), true, None, 1, None, None);

  let (tx, mut rx) = channel(1);
  spawn(async move {
    while let Some(m) = csv.next() {
      let n = m.unwrap();
      if let Err(_) = tx.send(n).await {
        println!("receiver dropped");
        return;
      }
    }
  });

  while let Some(i) = rx.recv().await {
    let json_rows = record_batches_to_json_rows(&[i]);
    let serialized = to_string(&json_rows).unwrap();
    
    window
      .emit("rust-event", serialized)
      .expect("failed to emit");
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}