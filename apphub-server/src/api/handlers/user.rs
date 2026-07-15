use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::AppError;
use crate::services::user_service;
use crate::config::AppState;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

/// 用户列表响应
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub status: i16,
    pub role_id: i64,
    pub created_at: String,
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role_id: i64,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub status: Option<i16>,
}

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

/// 获取用户列表
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (users, total) = user_service::list_users(&state.db, page, page_size).await?;

    let user_list: Vec<UserListResponse> = users
        .into_iter()
        .map(|u| UserListResponse {
            id: u.id,
            username: u.username,
            nickname: u.nickname,
            email: u.email,
            status: u.status,
            role_id: u.role_id,
            created_at: u.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": {
            "list": user_list,
            "total": total,
            "page": page,
            "page_size": page_size
        }
    })))
}

/// 创建用户
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = user_service::create_user(
        &state.db,
        &req.username,
        &req.password,
        req.role_id,
        req.nickname.as_deref(),
        req.email.as_deref(),
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "创建成功",
        "data": { "id": user_id }
    })))
}

/// 获取用户详情
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = user_service::get_user(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 查询角色信息
    use crate::models::role::{Entity as Role, Column as RoleColumn};
    let role = Role::find()
        .filter(RoleColumn::Id.eq(user.role_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;

    let user_response = json!({
        "id": user.id,
        "username": user.username,
        "nickname": user.nickname,
        "email": user.email,
        "avatar": user.avatar,
        "status": user.status,
        "role_id": user.role_id,
        "role_name": role.name,
        "created_at": user.created_at.to_rfc3339(),
        "last_login_at": user.last_login_at.map(|t| t.to_rfc3339()),
    });

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": user_response
    })))
}

/// 更新用户
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    user_service::update_user(
        &state.db,
        id,
        req.nickname.as_deref(),
        req.email.as_deref(),
        req.status,
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    })))
}

/// 删除用户
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    user_service::delete_user(&state.db, id).await?;
    Ok(Json(json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    })))
}

