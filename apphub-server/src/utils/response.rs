//! 响应工具

use axum::Json;
use serde::Serialize;

/// 统一响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

/// 成功响应
pub fn success<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        code: 200,
        message: "success".to_string(),
        data: Some(data),
    })
}

/// 成功响应（无数据）
pub fn success_empty() -> Json<ApiResponse<()>> {
    Json(ApiResponse {
        code: 200,
        message: "success".to_string(),
        data: None,
    })
}

/// 错误响应
pub fn error<T: Serialize>(code: i32, message: &str) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        code,
        message: message.to_string(),
        data: None,
    })
}
