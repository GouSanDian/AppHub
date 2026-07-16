//! 黑名单服务

use crate::error::AppError;
use crate::models::blacklist::{ActiveModel, Column as BlacklistColumn, Entity as Blacklist, Model};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set};

/// 创建黑名单
pub async fn create_blacklist(
    db: &DatabaseConnection,
    process_name: &str,
    description: Option<&str>,
    risk_level: i16,
    created_by: i64,
) -> Result<i64, AppError> {
    // 检查进程名是否已存在
    let existing = Blacklist::find()
        .filter(BlacklistColumn::ProcessName.eq(process_name))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("该进程已在黑名单中".to_string()));
    }

    let now = Utc::now();
    let blacklist = ActiveModel {
        process_name: Set(process_name.to_string()),
        description: Set(description.map(|s| s.to_string())),
        risk_level: Set(risk_level),
        status: Set(1), // 默认启用
        created_at: Set(now),
        updated_at: Set(now),
        created_by: Set(created_by),
        version: Set(1),
        ..Default::default()
    };

    let result = blacklist.insert(db).await?;
    tracing::info!("创建黑名单成功: {} (id: {})", process_name, result.id);

    Ok(result.id)
}

/// 获取黑名单列表
pub async fn list_blacklist(db: &DatabaseConnection) -> Result<Vec<Model>, AppError> {
    let blacklists = Blacklist::find()
        .filter(BlacklistColumn::Status.eq(1))
        .order_by_desc(BlacklistColumn::CreatedAt)
        .all(db)
        .await?;

    Ok(blacklists)
}

/// 获取客户端黑名单
pub async fn get_client_blacklist(db: &DatabaseConnection) -> Result<(i64, Vec<Model>), AppError> {
    // 获取最新版本号
    let latest = Blacklist::find()
        .filter(BlacklistColumn::Status.eq(1))
        .order_by_desc(BlacklistColumn::Version)
        .one(db)
        .await?;

    let version = latest.map(|m| m.version).unwrap_or(1);

    // 获取所有启用的黑名单
    let blacklists = Blacklist::find()
        .filter(BlacklistColumn::Status.eq(1))
        .all(db)
        .await?;

    Ok((version, blacklists))
}

/// 更新黑名单
pub async fn update_blacklist(
    db: &DatabaseConnection,
    id: i64,
    description: Option<&str>,
    risk_level: Option<i16>,
    status: Option<i16>,
) -> Result<(), AppError> {
    let blacklist = Blacklist::find()
        .filter(BlacklistColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("黑名单不存在".to_string()))?;

    let mut blacklist_active: ActiveModel = blacklist.into();

    if let Some(d) = description {
        blacklist_active.description = Set(Some(d.to_string()));
    }

    if let Some(r) = risk_level {
        blacklist_active.risk_level = Set(r);
    }

    if let Some(s) = status {
        blacklist_active.status = Set(s);
    }

    blacklist_active.updated_at = Set(Utc::now());
    blacklist_active.version = Set(blacklist_active.version.clone().unwrap() + 1);
    blacklist_active.update(db).await?;

    tracing::info!("更新黑名单成功: id={}", id);
    Ok(())
}

/// 删除黑名单
pub async fn delete_blacklist(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let blacklist = Blacklist::find()
        .filter(BlacklistColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("黑名单不存在".to_string()))?;

    // 逻辑删除：将状态设为禁用
    let mut blacklist_active: ActiveModel = blacklist.into();
    blacklist_active.status = Set(0);
    blacklist_active.updated_at = Set(Utc::now());
    blacklist_active.update(db).await?;

    tracing::info!("删除黑名单成功: id={}", id);
    Ok(())
}
