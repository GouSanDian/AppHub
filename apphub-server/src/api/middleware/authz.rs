//! 授权中间件

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::config::AppState;
use crate::utils::jwt::Claims;

/// 管理员授权中间件
/// 要求用户角色为 admin 或 super_admin
pub async fn admin_required(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 检查角色是否为管理员
    if claims.role != "admin" && claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

/// 超级管理员授权中间件
/// 要求用户角色为 super_admin
pub async fn super_admin_required(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 检查角色是否为超级管理员
    if claims.role != "super_admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}
