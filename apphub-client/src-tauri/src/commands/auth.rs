use serde::{Deserialize, Serialize};
use tauri::command;

use crate::api;

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

/// 用户信息
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub role: String,
}

/// 登录
#[command]
pub async fn login(req: LoginRequest) -> Result<LoginResponse, String> {
    tracing::info!("用户登录: {}", req.username);

    // 调用真实后端登录 API
    let (login_data, user_info) = api::login_api(&req.username, &req.password).await?;

    // 同步 token 到 Rust 后端，供扫描服务等使用
    api::set_tokens(login_data.access_token.clone(), login_data.refresh_token.clone());

    Ok(LoginResponse {
        access_token: login_data.access_token,
        refresh_token: login_data.refresh_token,
        expires_in: login_data.expires_in,
        user: UserInfo {
            id: user_info.id,
            username: user_info.username,
            nickname: user_info.nickname.unwrap_or_default(),
            role: user_info.role,
        },
    })
}

/// 从前端同步 token 到 Rust 后端（前端直接调 HTTP 登录后调用此命令）
#[command]
pub fn sync_token(access_token: String, refresh_token: String) -> Result<(), String> {
    tracing::info!("[auth] 前端同步 token 到 Rust 后端");
    tracing::info!("[auth] 收到 access_token: {}...", &access_token[..60.min(access_token.len())]);
    api::set_tokens(access_token, refresh_token);
    Ok(())
}

/// 从前端同步服务端地址到 Rust 后端
#[command]
pub fn sync_server_url(server_url: String) -> Result<(), String> {
    tracing::info!("[auth] 前端同步服务端地址: {}", server_url);
    api::set_server_url(server_url);
    Ok(())
}

/// 登出
#[command]
pub async fn logout() -> Result<(), String> {
    tracing::info!("用户登出");
    api::clear_tokens();
    Ok(())
}

/// 获取用户信息
#[command]
pub async fn get_user_info() -> Result<UserInfo, String> {
    let access_token = api::get_access_token()
        .ok_or_else(|| "未登录".to_string())?;
    let u = api::get_user_info_api(&access_token).await?;
    Ok(UserInfo {
        id: u.id,
        username: u.username,
        nickname: u.nickname.unwrap_or_default(),
        role: u.role,
    })
}
