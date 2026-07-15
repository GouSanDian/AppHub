//! API客户端 - 与服务端通信

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// 服务端API响应
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

/// 登录请求
#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应数据
#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// 用户信息响应数据
#[derive(Debug, Deserialize)]
pub struct UserInfoData {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
}

/// 全局HTTP客户端
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("创建HTTP客户端失败")
});

/// 全局Token存储
static ACCESS_TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static REFRESH_TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

/// 获取服务端地址
fn get_server_url() -> String {
    std::env::var("APPHUB_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string())
}

/// 设置Token
pub fn set_tokens(access: String, refresh: String) {
    let mut at = ACCESS_TOKEN.lock().unwrap();
    *at = Some(access);
    let mut rt = REFRESH_TOKEN.lock().unwrap();
    *rt = Some(refresh);
}

/// 获取访问Token
pub fn get_access_token() -> Option<String> {
    ACCESS_TOKEN.lock().unwrap().clone()
}

/// 清除Token
pub fn clear_tokens() {
    let mut at = ACCESS_TOKEN.lock().unwrap();
    *at = None;
    let mut rt = REFRESH_TOKEN.lock().unwrap();
    *rt = None;
}

/// 登录API
pub async fn login_api(username: &str, password: &str) -> Result<(LoginData, UserInfoData), String> {
    let url = format!("{}/api/v1/auth/login", get_server_url());

    let req = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let resp = HTTP_CLIENT
        .post(&url)
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("登录失败 ({}): {}", status, body));
    }

    let api_resp: ApiResponse<serde_json::Value> = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    if api_resp.code != 200 {
        return Err(api_resp.message);
    }

    let data = api_resp.data.ok_or("响应数据为空")?;

    // 解析登录数据
    let login_data: LoginData = serde_json::from_value(data.clone())
        .map_err(|e| format!("解析登录数据失败: {}", e))?;

    // 获取用户信息
    let user_info = get_user_info_api(&login_data.access_token).await?;

    Ok((login_data, user_info))
}

/// 获取用户信息API
pub async fn get_user_info_api(access_token: &str) -> Result<UserInfoData, String> {
    let url = format!("{}/api/v1/auth/user-info", get_server_url());

    let resp = HTTP_CLIENT
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("获取用户信息失败 ({})", status));
    }

    let api_resp: ApiResponse<UserInfoData> = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    if api_resp.code != 200 {
        return Err(api_resp.message);
    }

    api_resp.data.ok_or("用户信息为空".to_string())
}

/// 登出API
pub async fn logout_api(access_token: &str) -> Result<(), String> {
    let url = format!("{}/api/v1/auth/logout", get_server_url());

    let _ = HTTP_CLIENT
        .post(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;

    // 登出失败也不影响客户端操作
    Ok(())
}

/// 刷新Token API
pub async fn refresh_token_api(refresh_token: &str) -> Result<LoginData, String> {
    let url = format!("{}/api/v1/auth/refresh", get_server_url());

    let body = serde_json::json!({
        "refresh_token": refresh_token
    });

    let resp = HTTP_CLIENT
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("刷新Token失败 ({})", status));
    }

    let api_resp: ApiResponse<LoginData> = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    if api_resp.code != 200 {
        return Err(api_resp.message);
    }

    api_resp.data.ok_or("刷新数据为空".to_string())
}
