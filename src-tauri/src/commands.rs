use std::sync::{LazyLock, Mutex};

use crate::AppState;

#[tauri::command]
pub fn platform(state: tauri::State<AppState>) -> &'static str {
    state.lock().map(|s| s.platform).unwrap_or("unknown")
}

static MMS: LazyLock<Mutex<Option<tauri::WebviewWindow>>> = LazyLock::new(|| Mutex::new(None));

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
                const btn = createBtn('完成登录');
                const btn1 = createBtn('关闭窗口');
                const div = createDiv();
                div.appendChild(btn);
                div.appendChild(btn1);

                btn.onclick = async function() {
                    await invoke('logged_mms');
                };
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
    let window = MMS.lock().map_err(|e| e.to_string())?.take();
    let window = if let Some(window) = window {
        window.reload().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window
    } else {
        let window = tauri::WebviewWindowBuilder::new(
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
        .map_err(|e| e.to_string())?;

        window.set_decorations(false).map_err(|e| e.to_string())?;
        window
    };

    init_mms_window(&window)?;
    window.open_devtools();

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
    window.close_devtools();

    *MMS.lock().map_err(|e| e.to_string())? = Some(window);
    Ok(())
}

#[tauri::command]
pub async fn destory_mms() -> Result<(), String> {
    let window = MMS.lock().map_err(|e| e.to_string())?.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    window.destroy().map_err(|e| e.to_string())?;
    window.close_devtools();

    *MMS.lock().map_err(|e| e.to_string())? = None;
    Ok(())
}

#[tauri::command]
pub async fn logged_mms() -> Result<(), String> {
    let window = MMS.lock().map_err(|e| e.to_string())?.take();
    let Some(window) = window else {
        return Err("mms window not found".to_string());
    };

    let cookies = window.cookies().map_err(|e| e.to_string())?;
    for cookie in cookies {
        println!("{:?}", cookie);
    }

    window.destroy().map_err(|e| e.to_string())?;
    window.close_devtools();

    *MMS.lock().map_err(|e| e.to_string())? = None;
    Ok(())
}
