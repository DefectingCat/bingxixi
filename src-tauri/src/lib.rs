use std::sync::Mutex;

use serde_json::json;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

use crate::commands::{MmsStore, MMS_STORE};

pub mod commands;

#[derive(Default)]
pub struct AppStateInner {
    pub platform: &'static str,
}

pub type AppState = Mutex<AppStateInner>;
pub static MMS_STORE_NAME: &str = "mms_store";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
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

            // 初始化 store
            let store = app.store(MMS_STORE_NAME)?;
            let mms_store = store.get("mms");
            if let Some(mms_store) = mms_store {
                let mms_store_value: MmsStore = serde_json::from_value(mms_store)?;
                // 在这里发送事件，前端可能没有准备好
                // app.emit("mms_store", mms_store_value.clone())?;
                tauri::async_runtime::spawn(async move {
                    let mut mms_store = MMS_STORE.lock().await;
                    *mms_store = mms_store_value;
                });
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            // 当窗口被关闭时，将 mms 的状态保存到 store
            tauri::WindowEvent::CloseRequested { .. } => {
                let app = window.app_handle();
                let store = app
                    .store(MMS_STORE_NAME)
                    .map_err(|e| eprintln!("store error: {:?}", e));
                let Ok(store) = store else {
                    eprintln!("store error: cannot get store");
                    return;
                };
                tauri::async_runtime::spawn(async move {
                    let mms_store = MMS_STORE.lock().await;
                    store.set("mms", json!(mms_store.clone()));
                    store.save()?;
                    anyhow::Ok(())
                });
            }
            tauri::WindowEvent::Destroyed => (),
            _ => (),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::platform,
            commands::login_mms,
            commands::hide_mms,
            commands::destory_mms,
            commands::logged_mms,
            commands::get_mms_store
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
