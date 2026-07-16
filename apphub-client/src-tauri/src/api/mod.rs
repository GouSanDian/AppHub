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

/// 全局服务端地址存储（由前端同步，优先于配置文件）
static SERVER_URL: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

/// 设置服务端地址（由前端调用，确保与前端使用的地址一致）
pub fn set_server_url(url: String) {
    tracing::info!("[API] 服务端地址已更新: {}", url);
    let mut s = SERVER_URL.lock().unwrap();
    *s = Some(url);
}

/// 获取服务端基础 URL（优先使用前端同步的地址，否则从配置读取）
fn get_server_url() -> String {
    // 优先使用前端同步的地址
    if let Ok(guard) = SERVER_URL.lock() {
        if let Some(url) = guard.clone() {
            let base_url = if url.ends_with("/api/v1") {
                url.trim_end_matches("/api/v1").to_string()
            } else if url.ends_with("/api/v1/") {
                url.trim_end_matches("/api/v1/").to_string()
            } else {
                url
            };
            tracing::info!("[API] 使用前端同步的地址: {}", base_url);
            return base_url;
        }
    }

    // 降级：从配置文件读取
    if let Ok(config) = crate::commands::config::CONFIG.lock() {
        let url = config.server_url.clone();
        let base_url = if url.ends_with("/api/v1") {
            url.trim_end_matches("/api/v1").to_string()
        } else if url.ends_with("/api/v1/") {
            url.trim_end_matches("/api/v1/").to_string()
        } else {
            url
        };
        tracing::info!("[API] 使用配置文件中的地址: {}", base_url);
        return base_url;
    }

    // 最终降级：从环境变量读取
    let url = std::env::var("APPHUB_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    tracing::warn!("[API] 使用默认/环境变量地址: {}", url);
    url
}

/// 设置Token
pub fn set_tokens(access: String, refresh: String) {
    let access_preview = access.chars().take(40).collect::<String>();
    tracing::info!("[API] 设置 token: {}...", access_preview);
    
    // 使用独立的作用域块，确保锁在块结束时自动释放
    {
        let mut at = ACCESS_TOKEN.lock().unwrap();
        *at = Some(access);
    } // <-- ACCESS_TOKEN 的锁在这里释放
    
    {
        let mut rt = REFRESH_TOKEN.lock().unwrap();
        *rt = Some(refresh);
    } // <-- REFRESH_TOKEN 的锁在这里释放

    tracing::info!("[API] Token 已成功同步到 Rust 后端");
}

/// 获取访问Token
pub fn get_access_token() -> Option<String> {
    let token = ACCESS_TOKEN.lock().unwrap().clone();
    let token_preview = token.as_ref().map(|t| format!("{}...", &t[..40.min(t.len())]));
    tracing::debug!("[API] 获取 token: {:?}", token_preview);
    token
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
    let server_url = get_server_url();
    let url = format!("{}/api/v1/auth/user-info", server_url);

    tracing::info!("[API] 请求用户信息: {}", url);
    tracing::info!("[API] Token 长度: {}, 前缀: {}...", access_token.len(), &access_token[..40.min(access_token.len())]);

    let auth_header = format!("Bearer {}", access_token);
    tracing::info!("[API] Authorization header: {}", &auth_header[..60.min(auth_header.len())]);

    let resp = HTTP_CLIENT
        .get(&url)
        .header("Authorization", auth_header)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("[API] 请求失败: {}", e);
            format!("请求失败: {}", e)
        })?;

    let status = resp.status();
    tracing::info!("[API] 响应状态: {}", status);

    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        tracing::error!("[API] 获取用户信息失败 ({}): {}", status, body);
        return Err(format!("获取用户信息失败 ({}): {}", status, body));
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

/// 黑名单项
#[derive(Debug, Clone, Deserialize)]
pub struct BlacklistItem {
    pub id: i64,
    pub process_name: String,
    pub description: Option<String>,
    pub risk_level: i16,
    pub status: i16,
    pub version: i64,
}

/// 客户端黑名单响应数据
#[derive(Debug, Deserialize)]
pub struct ClientBlacklistData {
    pub version: i64,
    pub list: Vec<BlacklistItem>,
}

/// 获取客户端黑名单 API
pub async fn get_client_blacklist_api(access_token: &str) -> Result<ClientBlacklistData, String> {
    let url = format!("{}/api/v1/blacklists/client", get_server_url());

    let resp = HTTP_CLIENT
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("获取黑名单失败 ({})", status));
    }

    let api_resp: ApiResponse<ClientBlacklistData> = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    if api_resp.code != 200 {
        return Err(api_resp.message);
    }

    api_resp.data.ok_or("黑名单数据为空".to_string())
}

/// 黑名单进程（上报用）
#[derive(Debug, Clone, Serialize)]
pub struct BlacklistedProcessReport {
    pub process_name: String,
    pub pid: u32,
    pub risk_level: i16,
}

/// 扫描报告请求
#[derive(Debug, Serialize)]
pub struct ScanReportRequest {
    pub client_id: String,
    pub user_id: i64,
    pub username: String,
    pub scan_time: String,
    pub total_processes: i32,
    pub processes: Vec<String>,
    pub blacklisted_processes: Vec<BlacklistedProcessReport>,
}

/// 上报扫描结果 API
pub async fn report_scan_api(access_token: &str, report: &ScanReportRequest) -> Result<(), String> {
    let url = format!("{}/api/v1/reports/process-scans", get_server_url());

    let resp = HTTP_CLIENT
        .post(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(report)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("上报扫描结果失败 ({})", status));
    }

    let api_resp: ApiResponse<serde_json::Value> = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    if api_resp.code != 200 {
        return Err(api_resp.message);
    }

    Ok(())
}
