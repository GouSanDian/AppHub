use axum::{
    extract::State,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::error::AppError;
use crate::services::auth_service;
use crate::config::AppState;

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 刷新Token请求
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// 登录
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (access_token, refresh_token, expires_in) = auth_service::login(
        &state.db,
        &req.username,
        &req.password,
        &state.jwt_secret,
        state.jwt_expiration,
        state.jwt_refresh_expiration,
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "登录成功",
        "data": {
            "access_token": access_token,
            "refresh_token": refresh_token,
            "expires_in": expires_in
        }
    })))
}

/// 登出
pub async fn logout() -> Result<Json<serde_json::Value>, AppError> {
    // JWT是无状态的，登出主要由客户端清除Token完成
    Ok(Json(json!({
        "code": 200,
        "message": "登出成功",
        "data": null
    })))
}

/// 刷新Token
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (access_token, refresh_token, expires_in) = auth_service::refresh_token(
        &state.db,
        &req.refresh_token,
        &state.jwt_secret,
        state.jwt_expiration,
        state.jwt_refresh_expiration,
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "刷新成功",
        "data": {
            "access_token": access_token,
            "refresh_token": refresh_token,
            "expires_in": expires_in
        }
    })))
}

/// 获取用户信息
pub async fn get_user_info(
    State(state): State<AppState>,
    req: axum::extract::Request,
) -> Result<Json<serde_json::Value>, AppError> {
    // 从请求扩展中获取用户信息（由auth_middleware注入）
    let claims = req.extensions().get::<crate::utils::jwt::Claims>()
        .ok_or_else(|| AppError::Unauthorized("未授权".to_string()))?;

    let user_id: i64 = claims.sub.parse()
        .map_err(|_| AppError::Unauthorized("无效的Token".to_string()))?;

    // 查询用户信息
    use crate::models::user::{Entity as User, Column as UserColumn};
    use crate::models::role::{Entity as Role, Column as RoleColumn};
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    let user = User::find()
        .filter(UserColumn::Id.eq(user_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 查询角色信息
    let role = Role::find()
        .filter(RoleColumn::Id.eq(user.role_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": {
            "id": user.id,
            "username": user.username,
            "nickname": user.nickname,
            "email": user.email,
            "avatar": user.avatar,
            "role": role.name
        }
    })))
}

