use axum::{
    extract::State,
    Json,
};
use serde::Deserialize;

use crate::error::AppError;
use crate::config::AppState;

/// 扫描报告请求
#[derive(Debug, Deserialize)]
pub struct ScanReportRequest {
    pub client_id: String,
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

/// 上报扫描结果
pub async fn report_scan(
    State(_state): State<AppState>,
    Json(_req): Json<ScanReportRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 第四阶段实现
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "上报成功",
        "data": null
    })))
}

/// 获取统计信息
pub async fn get_statistics(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: 第五阶段实现
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "totalClients": 0,
            "onlineClients": 0,
            "totalSoftwares": 0,
            "totalDownloads": 0,
            "todayScans": 0,
            "todayAlerts": 0
        }
    })))
}
