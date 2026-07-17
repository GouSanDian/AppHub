use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::config::AppState;
use crate::models::blacklist::{Entity as Blacklist, Column as BlacklistColumn};
use crate::services::blacklist_service;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QuerySelect, PaginatorTrait, Condition};

use crate::api::dto::request::BlacklistQueryParams;

/// 黑名单项
#[derive(Debug, Serialize)]
pub struct BlacklistItem {
    pub id: i64,
    pub process_name: String,
    pub description: Option<String>,
    pub risk_level: i16,
    pub status: i16,
    pub version: i64,
}

/// 创建黑名单请求
#[derive(Debug, Deserialize)]
pub struct CreateBlacklistRequest {
    pub process_name: String,
    pub description: Option<String>,
    pub risk_level: i16,
}

/// 更新黑名单请求
#[derive(Debug, Deserialize)]
pub struct UpdateBlacklistRequest {
    pub description: Option<String>,
    pub risk_level: Option<i16>,
    pub status: Option<i16>,
}

/// 获取黑名单列表
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<BlacklistQueryParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut query = Blacklist::find();

    // 关键字搜索（进程名、描述）
    if let Some(keyword) = &params.keyword {
        if !keyword.trim().is_empty() {
            let like_pattern = format!("%{}%", keyword);
            let cond = Condition::any()
                .add(BlacklistColumn::ProcessName.like(&like_pattern))
                .add(BlacklistColumn::Description.like(&like_pattern));
            query = query.filter(cond);
        }
    }

    // 风险等级过滤
    if let Some(risk_level) = params.risk_level {
        query = query.filter(BlacklistColumn::RiskLevel.eq(risk_level));
    }

    // 状态过滤
    if let Some(status) = params.status {
        query = query.filter(BlacklistColumn::Status.eq(status));
    }

    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page - 1).await?;

    let list: Vec<BlacklistItem> = items
        .into_iter()
        .map(|m| BlacklistItem {
            id: m.id,
            process_name: m.process_name,
            description: m.description,
            risk_level: m.risk_level,
            status: m.status,
            version: m.version,
        })
        .collect();

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "list": list,
            "total": total,
            "page": page,
            "page_size": page_size,
            "version": 1
        }
    })))
}

/// 创建黑名单
pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateBlacklistRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 从 JWT 获取用户 ID，暂时使用默认值 1
    let user_id = 1i64;
    let id = blacklist_service::create_blacklist(
        &state.db,
        &body.process_name,
        body.description.as_deref(),
        body.risk_level,
        user_id,
    )
    .await?;

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "创建成功",
        "data": { "id": id }
    })))
}

/// 获取黑名单详情
pub async fn get(
    State(_state): State<AppState>,
    Path(_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 第四阶段实现
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "id": 1,
            "processName": "game.exe",
            "description": "游戏软件",
            "riskLevel": 2,
            "status": 1,
            "version": 1
        }
    })))
}

/// 更新黑名单
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateBlacklistRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    blacklist_service::update_blacklist(
        &state.db,
        id,
        req.description.as_deref(),
        req.risk_level,
        req.status,
    )
    .await?;

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    })))
}

/// 删除黑名单
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    blacklist_service::delete_blacklist(&state.db, id).await?;

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    })))
}

/// 获取客户端黑名单
pub async fn get_client_blacklist(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (version, blacklists) = blacklist_service::get_client_blacklist(&state.db).await?;

    let list: Vec<BlacklistItem> = blacklists
        .into_iter()
        .map(|m| BlacklistItem {
            id: m.id,
            process_name: m.process_name,
            description: m.description,
            risk_level: m.risk_level,
            status: m.status,
            version: m.version,
        })
        .collect();

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "version": version,
            "list": list
        }
    })))
}
