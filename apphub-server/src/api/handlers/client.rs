use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::config::AppState;

/// 客户端注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterClientRequest {
    pub client_id: String,
    pub device_name: String,
    pub os_type: String,
    pub os_version: String,
    pub mac_address: String,
}

/// 心跳请求
#[derive(Debug, Deserialize)]
pub struct HeartbeatRequest {
    pub client_id: String,
    pub ip_address: Option<String>,
}

/// 客户端信息
#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub client_id: String,
    pub device_name: String,
    pub os_type: String,
    pub status: i16,
    pub last_heartbeat_at: Option<String>,
}

/// 注册客户端
pub async fn register(
    State(_state): State<AppState>,
    Json(req): Json<RegisterClientRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 第四阶段实现
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "注册成功",
        "data": {
            "clientId": req.client_id,
            "blacklistVersion": 1
        }
    })))
}

/// 心跳
pub async fn heartbeat(
    State(_state): State<AppState>,
    Json(req): Json<HeartbeatRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 第四阶段实现
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "blacklistVersion": 1
        }
    })))
}

/// 获取客户端列表
pub async fn list(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 第四阶段实现
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "list": [],
            "total": 0,
            "onlineCount": 0
        }
    })))
}
