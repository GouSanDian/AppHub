//! 软件服务

use crate::error::AppError;
use crate::models::software::{ActiveModel, Column as SoftwareColumn, Entity as Software, Model};
use crate::services::file_service;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use chrono::Utc;
use std::path::Path;

/// 创建软件
pub async fn create_software(
    db: &DatabaseConnection,
    name: &str,
    version: &str,
    description: Option<&str>,
    category_id: i64,
    platform: &str,
    file_data: &[u8],
    filename: &str,
    upload_dir: &Path,
    created_by: i64,
) -> Result<i64, AppError> {
    // 保存文件
    let file_info = file_service::save_file(file_data, filename, upload_dir).await?;

    let now = Utc::now();
    let software = ActiveModel {
        name: Set(name.to_string()),
        version: Set(version.to_string()),
        description: Set(description.map(|s| s.to_string())),
        category_id: Set(category_id),
        platform: Set(platform.to_string()),
        file_name: Set(file_info.file_name),
        file_size: Set(file_info.file_size),
        file_path: Set(file_info.file_path),
        file_hash: Set(Some(file_info.file_hash)),
        status: Set(1),
        download_count: Set(0),
        created_at: Set(now),
        updated_at: Set(now),
        created_by: Set(created_by),
        ..Default::default()
    };

    let result = software.insert(db).await?;
    tracing::info!("创建软件成功: {} v{} (id: {})", name, version, result.id);

    Ok(result.id)
}

/// 获取软件列表（分页）
pub async fn list_software(
    db: &DatabaseConnection,
    category_id: Option<i64>,
    status: Option<i16>,
    page: u64,
    page_size: u64,
) -> Result<(Vec<Model>, u64), AppError> {
    use sea_orm::{PaginatorTrait, QuerySelect};

    let mut query = Software::find();

    if let Some(cat_id) = category_id {
        query = query.filter(SoftwareColumn::CategoryId.eq(cat_id));
    }

    if let Some(st) = status {
        query = query.filter(SoftwareColumn::Status.eq(st));
    }

    let paginator = query.paginate(db, page_size);
    let total = paginator.num_items().await?;
    let softwares = paginator.fetch_page(page - 1).await?;

    Ok((softwares, total))
}

/// 获取软件列表（带搜索过滤）
pub async fn list_software_with_filters(
    db: &DatabaseConnection,
    category_id: Option<i64>,
    status: Option<i16>,
    keyword: Option<&str>,
    platform: Option<&str>,
    page: u64,
    page_size: u64,
) -> Result<(Vec<Model>, u64), AppError> {
    use sea_orm::{Condition, PaginatorTrait, QuerySelect};

    let mut query = Software::find();

    if let Some(cat_id) = category_id {
        query = query.filter(SoftwareColumn::CategoryId.eq(cat_id));
    }

    if let Some(st) = status {
        query = query.filter(SoftwareColumn::Status.eq(st));
    }

    if let Some(kw) = keyword {
        if !kw.is_empty() {
            let keyword_condition = Condition::any()
                .add(SoftwareColumn::Name.contains(kw))
                .add(SoftwareColumn::Description.contains(kw));
            query = query.filter(keyword_condition);
        }
    }

    if let Some(plat) = platform {
        if !plat.is_empty() {
            query = query.filter(SoftwareColumn::Platform.contains(plat));
        }
    }

    let paginator = query.paginate(db, page_size);
    let total = paginator.num_items().await?;
    let softwares = paginator.fetch_page(page - 1).await?;

    Ok((softwares, total))
}

/// 获取软件详情
pub async fn get_software(db: &DatabaseConnection, id: i64) -> Result<Option<Model>, AppError> {
    let software = Software::find()
        .filter(SoftwareColumn::Id.eq(id))
        .one(db)
        .await?;
    Ok(software)
}

/// 更新软件信息
pub async fn update_software(
    db: &DatabaseConnection,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
    status: Option<i16>,
) -> Result<(), AppError> {
    let software = Software::find()
        .filter(SoftwareColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;

    let mut software_active: ActiveModel = software.into();

    if let Some(n) = name {
        software_active.name = Set(n.to_string());
    }

    if let Some(d) = description {
        software_active.description = Set(Some(d.to_string()));
    }

    if let Some(s) = status {
        software_active.status = Set(s);
    }

    software_active.updated_at = Set(Utc::now());
    software_active.update(db).await?;

    tracing::info!("更新软件成功: id={}", id);
    Ok(())
}

/// 删除软件
pub async fn delete_software(
    db: &DatabaseConnection,
    id: i64,
    upload_dir: &Path,
) -> Result<(), AppError> {
    let software = Software::find()
        .filter(SoftwareColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;

    // 删除文件
    let file_path = file_service::get_full_path(upload_dir, &software.file_path);
    file_service::delete_file(&file_path).await?;

    // 删除数据库记录
    Software::delete_by_id(id).exec(db).await?;

    tracing::info!("删除软件成功: id={}", id);
    Ok(())
}

/// 增加下载次数
pub async fn increment_download_count(
    db: &DatabaseConnection,
    id: i64,
) -> Result<(), AppError> {
    let software = Software::find()
        .filter(SoftwareColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;

    let mut software_active: ActiveModel = software.into();
    software_active.download_count = Set(software_active.download_count.unwrap() + 1);
    software_active.updated_at = Set(Utc::now());
    software_active.update(db).await?;

    Ok(())
}
