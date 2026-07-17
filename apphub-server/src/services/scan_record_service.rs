//! 扫描记录服务

use crate::error::AppError;
use crate::models::scan_record::{ActiveModel, Column as ScanRecordColumn, Entity as ScanRecord};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set};

/// 保存扫描记录（每个命中的黑名单进程一条记录）
pub async fn save_scan_record(
    db: &DatabaseConnection,
    client_id: &str,
    user_id: i64,
    username: &str,
    process_name: &str,
    risk_level: i16,
    scan_time: chrono::DateTime<Utc>,
) -> Result<i64, AppError> {
    let record = ActiveModel {
        client_id: Set(client_id.to_string()),
        user_id: Set(user_id),
        username: Set(username.to_string()),
        process_name: Set(process_name.to_string()),
        risk_level: Set(risk_level),
        scan_time: Set(scan_time),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let result = record.insert(db).await?;
    tracing::info!(
        "保存扫描记录: user={}, process={}, id={}",
        username,
        process_name,
        result.id
    );
    Ok(result.id)
}
