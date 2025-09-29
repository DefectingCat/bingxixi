use serde::{Deserialize, Serialize};
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_http::reqwest;

use crate::commands::MMS_STORE;

const BASE_URL: &str = "https://mms.pinduoduo.com";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MmsUser {
    success: bool,
    error_code: i64,
    error_msg: Option<serde_json::Value>,
    result: MmsUserResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MmsUserResult {
    id: i64,
    username: String,
    mobile: String,
    nickname: String,
    mall_owner: bool,
    role_id: i64,
    is_bind_we_chat: bool,
    force_mobile_verify: bool,
    mall_verify: bool,
    mall_info_authority: bool,
    conjoined_user_id: Option<serde_json::Value>,
    #[serde(rename = "mall_id")]
    mall_id: i64,
    #[serde(rename = "password_status")]
    password_status: i64,
    #[serde(rename = "server_time")]
    server_time: i64,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "updated_at")]
    updated_at: String,
}

/// 获取用户信息
#[tauri::command]
pub async fn fetch_user_info() -> Result<MmsUser, String> {
    let client = reqwest::Client::new();
    let mms = MMS_STORE.lock().await;

    let mut header_map = HeaderMap::default();
    header_map.insert(
        "Cookie",
        HeaderValue::from_str(&format!("mms-token={}", mms.cookie)).map_err(|e| e.to_string())?,
    );
    let res = client
        .post(format!("{BASE_URL}/janus/api/new/userinfo"))
        .headers(header_map)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let res = res.json::<MmsUser>().await.map_err(|e| e.to_string())?;
    Ok(res)
}
