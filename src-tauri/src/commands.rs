use std::{sync::LazyLock, time::Duration};

use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tokio::sync::Mutex;

use crate::AppState;

#[tauri::command]
pub fn platform(state: tauri::State<AppState>) -> &'static str {
    state.lock().map(|s| s.platform).unwrap_or("unknown")
}

/// 登录到 mms 的 webview 窗口
static MMS: LazyLock<Mutex<Option<tauri::WebviewWindow>>> = LazyLock::new(|| Mutex::new(None));

/// mms 的全局状态
#[derive(Serialize, Deserialize, Clone)]
pub struct MmsStore {
    /// 是否已经登录
    pub logged: bool,
    /// cookie
    pub cookie: String,
}
static MMS_STORE: LazyLock<Mutex<MmsStore>> = LazyLock::new(|| {
    Mutex::new(MmsStore {
        logged: false,
        cookie: String::new(),
    })
});

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

#[tauri::command]
/// 登录到拼多多
/// 首先检查是否已经创建了 webview 窗口，如果没有则创建
/// 如果已经创建了，再刷新
pub async fn login_mms(app: tauri::AppHandle) -> Result<(), String> {
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
    window.open_devtools();

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
            if !url.path().contains("login") {
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
                window.destroy()?;
                break;
            }
        }
        anyhow::Ok(())
    });
    Ok(())
}

#[tauri::command]
pub async fn hide_mms() -> Result<(), String> {
    let window = MMS.lock().await.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    window.hide().map_err(|e| e.to_string())?;
    window.close_devtools();

    *MMS.lock().await = Some(window);
    Ok(())
}

#[tauri::command]
pub async fn destory_mms() -> Result<(), String> {
    let window = MMS.lock().await.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    window.destroy().map_err(|e| e.to_string())?;
    window.close_devtools();

    *MMS.lock().await = None;
    Ok(())
}

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
    window.close_devtools();

    *MMS.lock().await = None;
    Ok(())
}
