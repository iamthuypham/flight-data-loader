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
use settimeout::set_timeout;
use std::time::Duration;

#[tauri::command]
async fn my_custom_command(window: tauri::Window) {
  let schema = Schema::new(vec![
    Field::new("exp_time", DataType::Utf8, false),
    Field::new("utc_time", DataType::Utf8, false),
    Field::new("lat_long_1", DataType::Utf8, false),
    
    Field::new("lat_long_2", DataType::Utf8, false),
    Field::new("pos_1", DataType::Utf8, false),
    Field::new("pos_2", DataType::Utf8, false),
    Field::new("pos_3", DataType::Utf8, false),
    Field::new("gps_alt", DataType::Utf8, false),
    Field::new("vel_1", DataType::Utf8, false),
    Field::new("vel_2", DataType::Utf8, false),
    Field::new("vel_3", DataType::Utf8, false),
    Field::new("accel_1", DataType::Utf8, false),
    Field::new("accel_2", DataType::Utf8, false),
    Field::new("accel_3", DataType::Utf8, false),
    Field::new("mag_accel", DataType::Utf8, false),
    Field::new("att_1", DataType::Utf8, false),
    Field::new("att_2", DataType::Utf8, false),
    Field::new("att_3", DataType::Utf8, false),
    Field::new("ang_vel_1", DataType::Utf8, false),
    Field::new("ang_vel_2", DataType::Utf8, false),
    Field::new("ang_vel_3", DataType::Utf8, false),
    Field::new("warnings_liftoff_warn", DataType::Utf8, false),
    Field::new("warnings_rcs_warn", DataType::Utf8, false),
    Field::new("warnings_drogue_chute_warn", DataType::Utf8, false),
    Field::new("warnings_landing_warn", DataType::Utf8, false),
    Field::new("warnings_chute_fault_warn", DataType::Utf8, false)
  ]);

  let file = File::open("src/data.csv").unwrap();

  let mut csv = csv::Reader::new(file, Arc::new(schema), true, None, 1, None, None);

    while let Some(m) = csv.next() {
      let n = m.unwrap();
    let json_rows = record_batches_to_json_rows(&[n]);
    let serialized = to_string(&json_rows).unwrap();
    
    // Delay process so frontend not freaking out
    set_timeout(Duration::from_micros(1_000)).await;

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