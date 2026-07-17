//! 认证中间件

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::services::auth_service;
use crate::config::AppState;

/// JWT认证中间件
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. 从请求头获取Token
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. 提取Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 3. 验证Token
    let claims = auth_service::verify_token(token, &state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 4. 将用户信息添加到请求扩展
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
