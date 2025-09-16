use std::sync::Mutex;

use tauri::Manager;

pub mod commands;

#[derive(Default)]
pub struct AppStateInner {
    pub platform: &'static str,
}

type AppState = Mutex<AppStateInner>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let platform = if cfg!(target_os = "windows") {
                "windows"
            } else if cfg!(target_os = "macos") {
                "macos"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else {
                "unknown"
            };
            app.manage(Mutex::new(AppStateInner { platform }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::platform
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
