// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::env::consts::ARCH;
use std::env::consts::OS;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn decode(app: tauri::AppHandle, name: &str, is_dir: bool) -> String {
    let binary_path = app.path_resolver()
        .resolve_resource("binary")
        .expect("failed to resolve resource dir");
    Command::new(format!("{}/{}/{}/decode_log_file", binary_path.as_path().to_str().unwrap(), OS, ARCH))
        .arg(name)
        .spawn()
        .expect("Failed to execute decode_log_file");
    if is_dir {
        format!("output: {}", name)
    } else {
        format!("output: {}", name.to_owned() + ".log")
    }
}

fn main() {
    // https://tauri.app/v1/guides/building/macos/
    // let _ = fix_path_env::fix();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![decode])
        .run(tauri::generate_context!())
        .expect("error while running xlog-decoder");
}
