use std::sync::{LazyLock, Mutex};

use crate::AppState;

#[tauri::command]
pub fn platform(state: tauri::State<AppState>) -> &'static str {
    state.lock().map(|s| s.platform).unwrap_or("unknown")
}

static MMS: LazyLock<Mutex<Option<tauri::WebviewWindow>>> = LazyLock::new(|| Mutex::new(None));

#[tauri::command]
/// 登录到拼多多
/// 首先检查是否已经创建了 webview 窗口，如果没有则创建
/// 如果已经创建了，再刷新
pub async fn login_mms(app: tauri::AppHandle) -> Result<(), String> {
    let window = MMS.lock().map_err(|e| e.to_string())?.take();
    let window = if let Some(window) = window {
        window.reload().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window
    } else {
        tauri::WebviewWindowBuilder::new(
            &app,
            "login_mms",
            tauri::WebviewUrl::External(
                "https://mms.pinduoduo.com/"
                    .parse()
                    .map_err(|_| "invalid url")?,
            ),
        )
        .title("登录")
        .build()
        .map_err(|e| e.to_string())?
    };

    *MMS.lock().map_err(|e| e.to_string())? = Some(window);
    Ok(())
}

#[tauri::command]
pub async fn hide_mms() -> Result<(), String> {
    let window = MMS.lock().map_err(|e| e.to_string())?.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    window.hide().map_err(|e| e.to_string())?;

    *MMS.lock().map_err(|e| e.to_string())? = Some(window);
    Ok(())
}
