use std::{sync::LazyLock, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Emitter;
use tauri_plugin_store::StoreExt;
use tokio::sync::Mutex;

use crate::{AppState, MMS_STORE_KEY, MMS_STORE_NAME};

#[tauri::command]
pub fn platform(state: tauri::State<AppState>) -> &'static str {
    state.lock().map(|s| s.platform).unwrap_or("unknown")
}

/// 登录到 mms 的 webview 窗口
static MMS: LazyLock<Mutex<Option<tauri::WebviewWindow>>> = LazyLock::new(|| Mutex::new(None));

/// mms 的全局状态
/// 该状态将会保存在 store 中
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MmsStore {
    /// 是否已经登录
    pub logged: bool,
    /// cookie
    pub cookie: String,
}
pub static MMS_STORE: LazyLock<Mutex<MmsStore>> = LazyLock::new(|| Mutex::new(MmsStore::default()));

/// 初始化 mms 的窗口
/// 为 mms 的窗口添加关闭按钮
fn init_mms_window(window: &tauri::WebviewWindow) -> Result<(), String> {
    window
        .eval(
            r#"
            function createDiv() {
                const div = document.createElement('div');
                div.style.position = 'absolute';
                div.style.top = '10px';
                div.style.left = '10px';
                div.style.color = '#fff';
                div.style.borderRadius = '5px';
                document.body.appendChild(div);
                return div;
            }

            function createBtn(text) {
                const btn = document.createElement('button');
                btn.innerText = text;
                btn.style.backgroundColor = '#fff';
                btn.style.color = '#000';
                btn.style.height = '40px';
                btn.style.borderRadius = '10px';
                btn.style.marginRight = '10px';
                btn.style.border = 'none';
                return btn;
            }

            window.onload = function() { 
                console.log('onload init');
                const invoke = window.__TAURI__.core.invoke;
                console.log(invoke);
                const btn1 = createBtn('关闭窗口');
                const div = createDiv();
                div.appendChild(btn);
                div.appendChild(btn1);

                btn1.onclick = async function() {
                    await invoke('destory_mms');
                };
            }"#,
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 登录到拼多多
/// 首先检查是否已经创建了 webview 窗口，如果没有则创建
/// 如果已经创建了，再刷新
#[tauri::command]
pub async fn login_mms(app: tauri::AppHandle) -> Result<(), String> {
    {
        let mms_store = MMS_STORE.lock().await;
        if mms_store.logged {
            return Err("mms already logged".to_string());
        }
    }

    let window = MMS.lock().await.take();
    let window = if let Some(window) = window {
        window.reload().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window
    } else {
        let window = tauri::WebviewWindowBuilder::new(
            &app,
            "login_mms",
            tauri::WebviewUrl::External(
                "https://mms.pinduoduo.com/login/"
                    .parse()
                    .map_err(|_| "invalid url")?,
            ),
        )
        .title("登录")
        .build()
        .map_err(|e| e.to_string())?;

        window.set_decorations(false).map_err(|e| e.to_string())?;
        window
    };

    // 初始化 mms 的窗口，添加关闭按钮
    init_mms_window(&window)?;
    // window.open_devtools();

    // 将 mms 的窗口存储到全局状态，已供其他命令使用
    *MMS.lock().await = Some(window);

    // 监听 mms 的窗口，当登录成功后，关闭窗口
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let window = MMS.lock().await;
            let window = window.as_ref();
            let Some(window) = window else {
                break;
            };
            let url = window.url()?;
            // window 会先打开一个 about:blank 的窗口，然后再打开 mms 的登录页面
            if !url.path().contains("login") && !url.path().contains("blank") {
                let mut mms_store = MMS_STORE.lock().await;
                mms_store.logged = true;
                let cookie_vec: Vec<String> = window
                    .cookies()?
                    .iter()
                    .map(|c| format!("{}={}", c.name(), c.value()))
                    .collect();
                let cookie_str = cookie_vec.join("; ");
                mms_store.cookie = cookie_str;
                app.emit("mms_store", mms_store.clone())?;
                // store
                let store = app.store(MMS_STORE_NAME)?;
                store.set("mms", json!(mms_store.clone()));
                store.save()?;
                window.destroy()?;
                break;
            }
        }
        anyhow::Ok(())
    });
    Ok(())
}

/// 隐藏 mms 的窗口
/// 并不会关闭 mms 的窗口，只是隐藏
#[tauri::command]
pub async fn hide_mms() -> Result<(), String> {
    let window = MMS.lock().await.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    window.hide().map_err(|e| e.to_string())?;
    // window.close_devtools();

    *MMS.lock().await = Some(window);
    Ok(())
}

/// 销毁 mms 的窗口
#[tauri::command]
pub async fn destory_mms() -> Result<(), String> {
    let window = MMS.lock().await.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    window.destroy().map_err(|e| e.to_string())?;
    // window.close_devtools();

    *MMS.lock().await = None;
    Ok(())
}

/// 获取 mms 的窗口的 cookie
/// 并销毁 mms 的窗口
#[tauri::command]
pub async fn logged_mms() -> Result<(), String> {
    let window = MMS.lock().await.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    let cookies = window.cookies().map_err(|e| e.to_string())?;
    for cookie in cookies {
        println!("{:?}", cookie);
    }

    window.destroy().map_err(|e| e.to_string())?;
    // window.close_devtools();

    *MMS.lock().await = None;
    Ok(())
}

/// 获取 mms 的本地存储
#[tauri::command]
pub async fn get_mms_store() -> Result<MmsStore, String> {
    let mms_store = MMS_STORE.lock().await;
    Ok(mms_store.clone())
}

/// 注销 mms 的登录
#[tauri::command]
pub async fn logout_mms(app: tauri::AppHandle) -> Result<(), String> {
    let mut mms_store = MMS_STORE.lock().await;
    *mms_store = MmsStore::default();

    let store = app.store(MMS_STORE_NAME).map_err(|e| e.to_string())?;
    let ok = store.delete(MMS_STORE_KEY);
    if !ok {
        return Err("delete mms store failed".to_string());
    }
    app.emit("mms_store", MmsStore::default())
        .map_err(|e| e.to_string())?;
    Ok(())
}
