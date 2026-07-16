use axum::{extract::State, Json};
use serde_json::json;

use crate::config::AppState;
use crate::error::AppError;
use crate::models::role::Entity as Role;
use sea_orm::EntityTrait;

/// 角色列表响应
#[derive(Debug, serde::Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

/// 获取角色列表
pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let roles = Role::find().all(&state.db).await?;

    let role_list: Vec<RoleResponse> = roles
        .into_iter()
        .map(|r| RoleResponse {
            id: r.id,
            name: r.name,
            description: r.description,
        })
        .collect();

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": role_list
    })))
}
