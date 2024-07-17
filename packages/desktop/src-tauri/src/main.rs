// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Listener};
use tauri::Manager;
use window_vibrancy::*;


fn main() {
  tauri::Builder::default()
  .setup(|app| {
      let window = app.get_webview_window("main").unwrap();

      #[cfg(target_os = "windows")]
      apply_mica(&window, Some(true))
          .expect("Unsupported platform! 'apply_mica' is only supported on Windows");

      #[cfg(target_os = "macos")]
      {
          apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
              .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
      }

      Ok(())
  })
  .run(tauri::generate_context!())  .expect("error while running tauri application!");
}
