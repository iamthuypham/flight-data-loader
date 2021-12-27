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
use arrow::datatypes::{DataType, Field, Schema, TimeUnit, Time64NanosecondType};
use arrow::array::{StringArray};
use std::fs::File;
use std::sync::Arc;
use arrow::json::writer::record_batches_to_json_rows;
use serde_json::to_string;
use settimeout::set_timeout;
use std::time::Duration;
use arrow::record_batch::RecordBatch;

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
    Field::new("truth_pos_CON_ECEF_ECEF_M[1]", DataType::Float64, false),
    Field::new("truth_pos_CON_ECEF_ECEF_M[2]", DataType::Float64, false),
    
    Field::new("truth_pos_CON_ECEF_ECEF_M[3]", DataType::Float64, false),
    Field::new("truth_vel_CON_ECEF_ECEF_MpS[1]", DataType::Float64, false),
    Field::new("truth_vel_CON_ECEF_ECEF_MpS[2]", DataType::Float64, false),
    Field::new("truth_vel_CON_ECEF_ECEF_MpS[3]", DataType::Float64, false),
    Field::new("truth_quat_CON2ECEF[1]", DataType::Float64, false),
    Field::new("truth_quat_CON2ECEF[2]", DataType::Float64, false),
    Field::new("truth_quat_CON2ECEF[3]", DataType::Float64, false),
    Field::new("truth_quat_CON2ECEF[4]", DataType::Float64, false)
  ]);

  let dlc_schema = Schema::new(vec![
    Field::new("TIME_NANOSECONDS_TAI", DataType::Utf8, false),
    Field::new("DATA_DELTA_VEL[1]", DataType::Float64, false),
    Field::new("DATA_DELTA_VEL[2]", DataType::Float64, false),
    
    Field::new("DATA_DELTA_VEL[3]", DataType::Float64, false),
    Field::new("DATA_DELTA_ANGLE[1]", DataType::Float64, false),
    Field::new("DATA_DELTA_ANGLE[2]", DataType::Float64, false),
    Field::new("DATA_DELTA_ANGLE[3]", DataType::Float64, false)
  ]);

  let lidar_schema = Schema::new(vec![
    Field::new("TIME_NANOSECONDS_TAI", DataType::Utf8, false),
    Field::new("OMPS_Range_M[1]", DataType::Float64, false),
    Field::new("OMPS_Range_M[2]", DataType::Float64, false),
    
    Field::new("OMPS_Range_M[3]", DataType::Float64, false),
    Field::new("OMPS_Range_M[4]", DataType::Float64, false),
    Field::new("OMPS_DopplerSpeed_MpS[1]", DataType::Float64, false),
    Field::new("OMPS_DopplerSpeed_MpS[2]", DataType::Float64, false),
    Field::new("OMPS_DopplerSpeed_MpS[3]", DataType::Float64, false),
    Field::new("OMPS_DopplerSpeed_MpS[4]", DataType::Float64, false),
  ]);

  // Open file
  let truth_file = File::open("data/Flight1_Catered_Dataset/Data/truth.csv").unwrap();
  let dlc_file = File::open("data/Flight1_Catered_Dataset/Data/dlc.csv").unwrap();
  let lidar_file = File::open("data/Flight1_Catered_Dataset/Data/commercial_lidar.csv").unwrap();

  // Get csv Reader using schema
  let mut truth_csv = csv::Reader::new(truth_file, Arc::new(truth_schema.clone()), true, None, 1, None, None);
  let mut dlc_csv = csv::Reader::new(dlc_file, Arc::new(dlc_schema.clone()), true, None, 1, None, None);
  let mut lidar_csv = csv::Reader::new(lidar_file, Arc::new(lidar_schema.clone()), true, None, 1, None, None);

  let mut start: i64 = 1602596010219040000;
  let end: i64 = 1602596810212950000;

  let mut truth_record_batch: RecordBatch = RecordBatch::new_empty(Arc::new(truth_schema));
  // let mut lidar_record_batch: RecordBatch = RecordBatch::new_empty(Arc::new(lidar_schema));

  // let mut lidar_should_be_next:bool = true ;
  // let mut lidar_time:i64 = 0;
  
  while start <= end {
    println!("{}", &start);
    // set_timeout(Duration::from_micros(10_000)).await;
    // window
    //     .emit("tai-event", &start)
    //     .expect("failed to emit");

    truth_record_batch = truth_csv.nth(0).unwrap().unwrap();
    let truth_time = get_time(&truth_record_batch);
    
    // if lidar_should_be_next {
    //   lidar_record_batch = lidar_csv.nth(0).unwrap().unwrap();
    //   lidar_time = get_time(&lidar_record_batch);
    // }

    if truth_time - start <= 1000 {
      let json_rows1 = record_batches_to_json_rows(&[truth_record_batch.clone()]);
      let serialized1 = to_string(&json_rows1).unwrap();

        // Delay process so frontend not freaking out
        set_timeout(Duration::from_micros(10000)).await;
      window
        .emit("truth-event", serialized1)
        .expect("failed to emit");
    } 

    // if lidar_time - start <= 10000 {
    //   let json_rows2 = record_batches_to_json_rows(&[lidar_record_batch.clone()]);
    //   let serialized2 = to_string(&json_rows2).unwrap();

    //     // Delay process so frontend not freaking out
    //     // set_timeout(Duration::from_micros(10_000)).await;
    //   window
    //     .emit("lidar-event", serialized2)
    //     .expect("failed to emit");
    // } else {
    //   // keep current record batch
    //   lidar_should_be_next = false;
    // } 

    start = start + 10000000; // 10_000_000
  }
    
}

fn get_time(record_batch: &RecordBatch) -> i64 {
  let base: f64 = 10.0;
  let t = base.powf(18.0);

  let v = record_batch
    .column(0)
    .as_any().downcast_ref::<StringArray>().unwrap()
    .value(0)
    .replace("e+18", "")
    .parse::<f64>().unwrap();

    let res = (t * v).round() as i64;
    return res;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![close_splashscreen, my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

// truth: s-1.60259601021904e+18 e-1.60259681021295e+18
// dlc  : s-1.6025960102293e+18  e-1.602596810229e+18
// lidar: s-1.60259621009107e+18 e-1.60259665919107e+18

// t-0  :   1.60259621021e+18
// let x, y, z = 0;
// loop i from 1.60259601021904e+18 to 1.60259681021295e+18
// if i === truth[x].tai: emit  and x++
// if i === dlc[x].tai: emit  and y++
// if i === lidar[x].tai: emit  and z++

// 1.60 25 96 01 02 39 04 00 00
