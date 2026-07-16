use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::error::AppError;
use crate::config::AppState;
use crate::services::scan_record_service;

/// 扫描报告请求
#[derive(Debug, Deserialize)]
pub struct ScanReportRequest {
    pub client_id: String,
    pub user_id: i64,
    pub username: String,
    pub scan_time: String,
    pub total_processes: i32,
    pub processes: Vec<String>,
    pub blacklisted_processes: Vec<BlacklistedProcess>,
}

/// 黑名单进程
#[derive(Debug, Deserialize)]
pub struct BlacklistedProcess {
    pub process_name: String,
    pub pid: u32,
    pub risk_level: i16,
}

/// 扫描记录查询参数
#[derive(Debug, Deserialize)]
pub struct ScanRecordQueryParams {
    pub keyword: Option<String>,
    pub username: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

/// 上报扫描结果
pub async fn report_scan(
    State(state): State<AppState>,
    Json(req): Json<ScanReportRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 解析扫描时间
    let scan_time = chrono::DateTime::parse_from_rfc3339(&req.scan_time)
        .map_err(|e| AppError::ValidationError(format!("无效的扫描时间格式: {}", e)))?
        .with_timezone(&chrono::Utc);

    // 为每个命中的黑名单进程保存一条记录
    for process in &req.blacklisted_processes {
        scan_record_service::save_scan_record(
            &state.db,
            &req.client_id,
            req.user_id,
            &req.username,
            &process.process_name,
            process.risk_level,
            scan_time,
        )
        .await?;
    }

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "上报成功",
        "data": null
    })))
}

/// 获取扫描记录列表（管理员）
pub async fn get_scan_records(
    State(state): State<AppState>,
    Query(params): Query<ScanRecordQueryParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    use crate::models::scan_record::{Entity as ScanRecord, Column as ScanRecordColumn};
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait, Condition};

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    let mut query = ScanRecord::find();

    // 关键字搜索（用户名、进程名）
    if let Some(keyword) = &params.keyword {
        if !keyword.trim().is_empty() {
            let like_pattern = format!("%{}%", keyword);
            let cond = Condition::any()
                .add(ScanRecordColumn::Username.like(&like_pattern))
                .add(ScanRecordColumn::ProcessName.like(&like_pattern));
            query = query.filter(cond);
        }
    }

    // 用户名过滤
    if let Some(username) = &params.username {
        if !username.trim().is_empty() {
            query = query.filter(ScanRecordColumn::Username.eq(username));
        }
    }

    let paginator = query
        .order_by_desc(ScanRecordColumn::ScanTime)
        .paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page - 1).await?;

    let list: Vec<serde_json::Value> = items
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "id": m.id,
                "client_id": m.client_id,
                "user_id": m.user_id,
                "username": m.username,
                "process_name": m.process_name,
                "risk_level": m.risk_level,
                "scan_time": m.scan_time.to_rfc3339(),
                "created_at": m.created_at.to_rfc3339()
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "list": list,
            "total": total,
            "page": page,
            "page_size": page_size
        }
    })))
}

/// 获取统计信息
pub async fn get_statistics(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    use crate::models::{user, software, blacklist};
    use sea_orm::EntityTrait;

    // 获取用户总数
    let user_count = user::Entity::find()
        .all(&state.db)
        .await?
        .len() as i64;

    // 获取软件总数
    let software_count = software::Entity::find()
        .all(&state.db)
        .await?
        .len() as i64;

    // 获取黑名单总数
    let blacklist_count = blacklist::Entity::find()
        .all(&state.db)
        .await?
        .len() as i64;

    // 获取总下载次数
    let softwares = software::Entity::find()
        .all(&state.db)
        .await?;
    let total_downloads: i64 = softwares.iter()
        .map(|s| s.download_count)
        .sum();

    // 获取最近登录的用户（最近10个）
    let recent_users = user::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .filter(|u| u.last_login_at.is_some())
        .collect::<Vec<_>>();

    let mut recent_logins: Vec<serde_json::Value> = recent_users
        .into_iter()
        .map(|u| {
            serde_json::json!({
                "username": u.username,
                "login_time": u.last_login_at.map(|t| t.to_rfc3339()).unwrap_or_default()
            })
        })
        .collect();

    // 按登录时间倒序排序
    recent_logins.sort_by(|a, b| {
        let time_a = a["login_time"].as_str().unwrap_or("");
        let time_b = b["login_time"].as_str().unwrap_or("");
        time_b.cmp(time_a)
    });
    recent_logins.truncate(10);

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "userCount": user_count,
            "softwareCount": software_count,
            "blacklistCount": blacklist_count,
            "downloadCount": total_downloads,
            "recentLogins": recent_logins
        }
    })))
}
