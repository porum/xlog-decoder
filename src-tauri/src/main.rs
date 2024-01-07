// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::process::{Command};
use std::env::consts::ARCH;
use std::env::consts::OS;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn decode(app: tauri::AppHandle, name: &str, private_key: &str, dist: &str) -> i32 {
  let binary = Path::new("binary")
    .join(OS)
    .join(ARCH)
    .join(if OS == "windows" { "decode_log_file.exe" } else { "decode_log_file" });
  let path = app.path_resolver()
    .resolve_resource(binary)
    .expect("failed to resolve resource dir");
  let mut command = Command::new(path);
  command.args([private_key, name]);
  if dist != "" {
    command.arg(dist);
  }
  return command.status().unwrap().code().unwrap();
}

#[cfg(target_os = "linux")]
use std::{fs::metadata, path::PathBuf};
#[cfg(target_os = "linux")]
use fork::{daemon, Fork}; // dep: fork = "0.1"

#[tauri::command]
fn show_in_folder(path: String, opening: bool) {
  #[cfg(target_os = "windows")]
  {
    if opening {
      Command::new("explorer.exe")
        .arg(&path)
        .spawn()
        .unwrap();
    } else {
      Command::new("explorer.exe")
        .args(["/select,", &path]) // The comma after select is not a typo
        .spawn()
        .unwrap();
    }
  }

  #[cfg(target_os = "linux")]
  {
    if path.contains(",") {
      // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
      let new_path = match metadata(&path).unwrap().is_dir() {
        true => path,
        false => {
          let mut path2 = PathBuf::from(path);
          path2.pop();
          path2.into_os_string().into_string().unwrap()
        }
      };
      Command::new("xdg-open")
        .arg(&new_path)
        .spawn()
        .unwrap();
    } else {
      if let Ok(Fork::Child) = daemon(false, false) {
        Command::new("dbus-send")
          .args(["--session", "--dest=org.freedesktop.FileManager1", "--type=method_call",
            "/org/freedesktop/FileManager1", "org.freedesktop.FileManager1.ShowItems",
            format!("array:string:\"file://{path}\"").as_str(), "string:\"\""])
          .spawn()
          .unwrap();
      }
    }
  }

  #[cfg(target_os = "macos")]
  {
    if opening {
      Command::new("open")
        .arg(&path)
        .spawn()
        .unwrap();
    } else {
      Command::new("open")
        .args(["-R", &path])
        .spawn()
        .unwrap();
    }
  }
}

fn main() {
  // https://tauri.app/v1/guides/building/macos/
  // let _ = fix_path_env::fix();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![decode, show_in_folder])
    .run(tauri::generate_context!())
    .expect("error while running xlog-decoder");
}
