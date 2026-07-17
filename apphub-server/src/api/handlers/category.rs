use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::category_service;

/// 分类列表项
#[derive(Debug, Serialize)]
pub struct CategoryItem {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub status: i16,
}

/// 创建分类请求
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

/// 更新分类请求
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<i16>,
}

/// 获取分类列表
pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let categories = category_service::list_categories(&state.db).await?;

    let items: Vec<CategoryItem> = categories
        .into_iter()
        .map(|c| CategoryItem {
            id: c.id,
            name: c.name,
            description: c.description,
            sort_order: c.sort_order,
            status: c.status,
        })
        .collect();

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": items
    })))
}

/// 创建分类
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let category_id = category_service::create_category(
        &state.db,
        &req.name,
        req.description.as_deref(),
        req.sort_order.unwrap_or(0),
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "创建成功",
        "data": { "id": category_id }
    })))
}

/// 获取分类详情
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let category = category_service::get_category(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("分类不存在".to_string()))?;

    let item = CategoryItem {
        id: category.id,
        name: category.name,
        description: category.description,
        sort_order: category.sort_order,
        status: category.status,
    };

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": item
    })))
}

/// 更新分类
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    category_service::update_category(
        &state.db,
        id,
        req.name.as_deref(),
        req.description.as_deref(),
        req.sort_order,
        req.status,
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    })))
}

/// 删除分类
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    category_service::delete_category(&state.db, id).await?;
    Ok(Json(json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    })))
}
