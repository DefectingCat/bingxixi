use std::sync::Mutex;

use anyhow::anyhow;
use serde_json::json;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::tray::MouseButton;
use tauri::tray::MouseButtonState;
use tauri::tray::TrayIconBuilder;
use tauri::tray::TrayIconEvent;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

use crate::commands::{MmsStore, MMS_STORE};

pub mod commands;

#[derive(Default)]
pub struct AppStateInner {
    pub platform: &'static str,
}

pub type AppState = Mutex<AppStateInner>;
/// 本地存储的全局状态名称
pub static MMS_STORE_NAME: &str = "mms_store";
/// mms 的本地存储 key 名称
pub static MMS_STORE_KEY: &str = "mms";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app
                .get_webview_window("main")
                .expect("main window not found");
            let _ = window
                .unminimize()
                .map_err(|e| log::error!("unminimize_window error: {:?}", e));
            let _ = window
                .show()
                .map_err(|e| log::error!("show_window error: {:?}", e));
            let _ = window
                .set_focus()
                .map_err(|e| log::error!("set_focus_window error: {:?}", e));
        }))
        .plugin(tauri_plugin_window_state::Builder::new().build())
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
            let mms_store = store.get(MMS_STORE_KEY);
            if let Some(mms_store) = mms_store {
                let mms_store_value: MmsStore = serde_json::from_value(mms_store)?;
                // 在这里发送事件，前端可能没有准备好
                // app.emit("mms_store", mms_store_value.clone())?;
                tauri::async_runtime::spawn(async move {
                    let mut mms_store = MMS_STORE.lock().await;
                    *mms_store = mms_store_value;
                });
            }

            // system tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "test" => {
                        log::debug!("test");
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        // in this example, let's show and focus the main window when the tray is clicked
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .icon(
                    app.default_window_icon()
                        .ok_or(anyhow!("default_window_icon"))?
                        .clone(),
                )
                .build(app)?;

            // 恢复窗口位置
            if let Some(window) = app.get_webview_window("main") {
                let _ = window
                    .restore_state(StateFlags::all())
                    .map_err(|e| log::error!("restore_window_state error: {:?}", e));
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                log::info!("window close requested");
                api.prevent_close();
                let _ = window
                    .hide()
                    .map_err(|e| log::error!("hide_window error: {:?}", e));

                let app = window.app_handle();
                let store = app
                    .store(MMS_STORE_NAME)
                    .map_err(|e| log::error!("store error: {:?}", e));
                let Ok(store) = store else {
                    log::error!("store error: cannot get store");
                    return;
                };
                tauri::async_runtime::spawn(async move {
                    let mms_store = MMS_STORE.lock().await;
                    store.set(MMS_STORE_KEY, json!(mms_store.clone()));
                    store.save()?;
                    anyhow::Ok(())
                });
            }
            // 当窗口被关闭时，将 mms 的状态保存到 store
            tauri::WindowEvent::Destroyed => {
                log::info!("window destroyed");
                let app = window.app_handle();
                let store = app
                    .store(MMS_STORE_NAME)
                    .map_err(|e| log::error!("store error: {:?}", e));
                let Ok(store) = store else {
                    log::error!("store error: cannot get store");
                    return;
                };
                tauri::async_runtime::spawn(async move {
                    let mms_store = MMS_STORE.lock().await;
                    store.set(MMS_STORE_KEY, json!(mms_store.clone()));
                    store.save()?;
                    anyhow::Ok(())
                });
                // 保存窗口位置
                let _ = app
                    .save_window_state(StateFlags::all())
                    .map_err(|e| log::error!("save_window_state error: {:?}", e));
            }
            _ => (),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::platform,
            commands::login_mms,
            commands::hide_mms,
            commands::destory_mms,
            commands::logged_mms,
            commands::get_mms_store,
            commands::logout_mms,
            commands::api::fetch_user_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
