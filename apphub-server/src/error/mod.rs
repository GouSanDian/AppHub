use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// 应用错误类型
#[derive(Debug)]
pub enum AppError {
    // 认证错误
    Unauthorized(String),
    Forbidden(String),

    // 业务错误
    NotFound(String),
    ValidationError(String),
    Conflict(String),

    // 系统错误
    DatabaseError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized(msg) => write!(f, "未授权: {}", msg),
            AppError::Forbidden(msg) => write!(f, "禁止访问: {}", msg),
            AppError::NotFound(msg) => write!(f, "未找到: {}", msg),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            AppError::Conflict(msg) => write!(f, "冲突: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
            AppError::InternalError(msg) => write!(f, "内部错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "code": status.as_u16(),
            "message": message,
            "data": null
        }));

        (status, body).into_response()
    }
}

// 从其他错误类型转换
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ValidationError(err.to_string())
    }
}
