use crate::AppState;

#[tauri::command]
pub fn platform(state: tauri::State<AppState>) -> &'static str {
    state.lock().map(|s| s.platform).unwrap_or("unknown")
}

#[tauri::command]
pub async fn login_mms(app: tauri::AppHandle) -> Result<(), String> {
    tauri::WebviewWindowBuilder::new(
        &app,
        "login_mms",
        tauri::WebviewUrl::External(
            "https://mms.pinduoduo.com/"
                .parse()
                .map_err(|_| "invalid url")?,
        ),
    )
    .title("登录微信")
    .build()
    .map_err(|e| e.to_string())?;
    Ok(())
}
