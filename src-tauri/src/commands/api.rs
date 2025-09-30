use serde::{Deserialize, Serialize};
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_http::reqwest;

use crate::commands::MMS_STORE;

const BASE_URL: &str = "https://mms.pinduoduo.com";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MmsBaseResponse<T> {
    success: bool,
    error_code: u32,
    error_msg: Option<String>,
    result: T,
}

/// 出现错误时的响应
/// error_code 为 43001 时为 cookie 失效
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MmsErrResponse {
    error_code: u32,
    error_msg: String,
    success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MmsResponse<T> {
    Success(MmsBaseResponse<T>),
    Err(MmsErrResponse),
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
pub async fn fetch_user_info() -> Result<MmsResponse<MmsUserResult>, String> {
    let client = reqwest::Client::new();
    let mms = MMS_STORE.lock().await;

    let mut header_map = HeaderMap::default();
    header_map.insert(
        "Cookie",
        HeaderValue::from_str(&mms.cookie).map_err(|e| e.to_string())?,
    );
    let res = client
        .post(format!("{BASE_URL}/janus/api/new/userinfo"))
        .headers(header_map)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let res = res
        .json::<MmsResponse<MmsUserResult>>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}
