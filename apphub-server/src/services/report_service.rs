//! 上报服务

use crate::error::AppError;
use crate::models::scan_report::{ActiveModel, Entity as ScanReport, Model};
use sea_orm::DatabaseConnection;

/// 保存扫描报告
pub async fn save_scan_report(
    _db: &DatabaseConnection,
    _client_id: &str,
    _total_processes: i32,
    _blacklisted_count: i32,
) -> Result<i64, AppError> {
    // TODO: 实现扫描报告保存
    tracing::info!("保存扫描报告");
    Ok(1)
}

/// 获取统计信息
pub async fn get_statistics(_db: &DatabaseConnection) -> Result<serde_json::Value, AppError> {
    // TODO: 实现统计信息查询
    Ok(serde_json::json!({
        "total_clients": 0,
        "online_clients": 0,
        "total_softwares": 0,
        "total_downloads": 0,
    }))
}
