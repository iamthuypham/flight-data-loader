#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![allow(
    // Clippy bug: https://github.com/rust-lang/rust-clippy/issues/7422
    clippy::nonstandard_macro_braces,
)]

use tauri::Manager;
use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use std::fs::File;
use std::sync::Arc;
use arrow::json::writer::record_batches_to_json_rows;
use serde_json::to_string;
use settimeout::set_timeout;
use std::time::Duration;

#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn my_custom_command(window: tauri::Window) {
  let truth_schema = Schema::new(vec![
    Field::new("TIME_NANOSECONDS_TAI", DataType::Utf8, false),
    Field::new("truth_pos_CON_ECEF_ECEF_M[1]", DataType::Utf8, false),
    Field::new("truth_pos_CON_ECEF_ECEF_M[2]", DataType::Utf8, false),
    
    Field::new("truth_pos_CON_ECEF_ECEF_M[3]", DataType::Utf8, false),
    Field::new("truth_vel_CON_ECEF_ECEF_MpS[1]", DataType::Utf8, false),
    Field::new("truth_vel_CON_ECEF_ECEF_MpS[2]", DataType::Utf8, false),
    Field::new("truth_vel_CON_ECEF_ECEF_MpS[3]", DataType::Utf8, false),
    Field::new("truth_quat_CON2ECEF[1]", DataType::Utf8, false),
    Field::new("truth_quat_CON2ECEF[2]", DataType::Utf8, false),
    Field::new("truth_quat_CON2ECEF[3]", DataType::Utf8, false),
    Field::new("truth_quat_CON2ECEF[4]", DataType::Utf8, false)
  ]);

  let dlc_schema = Schema::new(vec![
    Field::new("TIME_NANOSECONDS_TAI", DataType::Utf8, false),
    Field::new("DATA_DELTA_VEL[1]", DataType::Utf8, false),
    Field::new("DATA_DELTA_VEL[2]", DataType::Utf8, false),
    
    Field::new("DATA_DELTA_VEL[3]", DataType::Utf8, false),
    Field::new("DATA_DELTA_ANGLE[1]", DataType::Utf8, false),
    Field::new("DATA_DELTA_ANGLE[2]", DataType::Utf8, false),
    Field::new("DATA_DELTA_ANGLE[3]", DataType::Utf8, false)
  ]);

  let lidar_schema = Schema::new(vec![
    Field::new("TIME_NANOSECONDS_TAI", DataType::Utf8, false),
    Field::new("OMPS_Range_M[1]", DataType::Utf8, false),
    Field::new("OMPS_Range_M[2]", DataType::Utf8, false),
    
    Field::new("OMPS_Range_M[3]", DataType::Utf8, false),
    Field::new("OMPS_Range_M[4]", DataType::Utf8, false),
    Field::new("OMPS_DopplerSpeed_MpS[1]", DataType::Utf8, false),
    Field::new("OMPS_DopplerSpeed_MpS[2]", DataType::Utf8, false),
    Field::new("OMPS_DopplerSpeed_MpS[3]", DataType::Utf8, false),
    Field::new("OMPS_DopplerSpeed_MpS[4]", DataType::Utf8, false),
  ]);

  // Open file
  let truth_file = File::open("data/Flight1_Catered_Dataset/Data/truth.csv").unwrap();
  let dlc_file = File::open("data/Flight1_Catered_Dataset/Data/dlc.csv").unwrap();
  let lidar_file = File::open("data/Flight1_Catered_Dataset/Data/commercial_lidar.csv").unwrap();

  // Get csv Reader using schema
  let mut truth_csv = csv::Reader::new(truth_file, Arc::new(truth_schema), true, None, 1, None, None);
  let mut dlc_csv = csv::Reader::new(dlc_file, Arc::new(dlc_schema), true, None, 1, None, None);
  let mut lidar_csv = csv::Reader::new(lidar_file, Arc::new(lidar_schema), true, None, 1, None, None);


  while let Some(m) = truth_csv.next() {
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
    .invoke_handler(tauri::generate_handler![close_splashscreen, my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}