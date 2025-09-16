use crate::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn platform(state: tauri::State<AppState>) -> &'static str {
    state.lock().map(|s| s.platform).unwrap_or("unknown")
}
