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
// use arrow::json::writer::record_batches_to_json_rows;
// use serde_json::to_string;
use arrow::array::{StringArray, ArrayRef};

#[tauri::command]
async fn my_custom_command(window: tauri::Window) {
  let schema = Schema::new(vec![
    Field::new("exp_time", DataType::Utf8, false),
    // Field::new("utc_time", DataType::Utf8, false),
    // Field::new("lat_long_1", DataType::Utf8, false),
  ]);

  // Open file
  let file = File::open("src/data.csv").unwrap();

  // Get csv Reader using schema
  let mut csv = csv::Reader::new(file, Arc::new(schema), true, None, 1, None, None);

    // Loop through each row
    while let Some(m) = csv.next() {
      let n = m.unwrap();
      // Get reference of array of a column
      let col: &ArrayRef = n.column(0);
      // Cast the reference of array to array of string
      let col = col.as_any().downcast_ref::<StringArray>().unwrap();
      // Get value from the array using index
      let v = col.value(0);
      println!("{}", col.value(0));
      
      // Send each value through an event
      window
        .emit("rust-event", v)
        .expect("failed to emit");
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}