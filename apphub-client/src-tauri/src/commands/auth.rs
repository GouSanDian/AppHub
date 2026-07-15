use serde::{Deserialize, Serialize};
use tauri::command;

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
    // TODO: 调用后端登录API
    tracing::info!("用户登录: {}", req.username);

    Ok(LoginResponse {
        access_token: format!("mock-token-{}", req.username),
        refresh_token: format!("mock-refresh-{}", req.username),
        expires_in: 86400,
        user: UserInfo {
            id: 1,
            username: req.username.clone(),
            nickname: req.username.clone(),
            role: "user".to_string(),
        },
    })
}

/// 登出
#[command]
pub async fn logout() -> Result<(), String> {
    tracing::info!("用户登出");
    Ok(())
}

/// 获取用户信息
#[command]
pub async fn get_user_info() -> Result<UserInfo, String> {
    Ok(UserInfo {
        id: 1,
        username: "user".to_string(),
        nickname: "用户".to_string(),
        role: "user".to_string(),
    })
}
