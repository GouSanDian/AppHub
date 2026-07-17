//! 分类服务

use crate::error::AppError;
use crate::models::category::{ActiveModel, Entity as Category, Column as CategoryColumn, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, Set};
use chrono::Utc;

/// 创建分类
pub async fn create_category(
    db: &DatabaseConnection,
    name: &str,
    description: Option<&str>,
    sort_order: i32,
) -> Result<i64, AppError> {
    // 检查名称是否已被活跃分类使用（排除已软删除的分类）
    let existing = Category::find()
        .filter(CategoryColumn::Name.eq(name))
        .filter(CategoryColumn::Status.eq(1))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("分类名称已存在".to_string()));
    }

    let now = Utc::now();
    let category = ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.map(|s| s.to_string())),
        sort_order: Set(sort_order),
        status: Set(1),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let result = category.insert(db).await?;
    tracing::info!("创建分类成功: {} (id: {})", name, result.id);

    Ok(result.id)
}

/// 获取分类列表
pub async fn list_categories(
    db: &DatabaseConnection,
) -> Result<Vec<Model>, AppError> {
    let categories = Category::find()
        .filter(CategoryColumn::Status.eq(1))
        .order_by_asc(CategoryColumn::SortOrder)
        .all(db)
        .await?;

    Ok(categories)
}

/// 获取分类详情
pub async fn get_category(db: &DatabaseConnection, id: i64) -> Result<Option<Model>, AppError> {
    let category = Category::find()
        .filter(CategoryColumn::Id.eq(id))
        .one(db)
        .await?;
    Ok(category)
}

/// 更新分类
pub async fn update_category(
    db: &DatabaseConnection,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
    sort_order: Option<i32>,
    status: Option<i16>,
) -> Result<(), AppError> {
    let category = Category::find()
        .filter(CategoryColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("分类不存在".to_string()))?;

    let mut category_active: ActiveModel = category.into();

    if let Some(n) = name {
        // 检查新名称是否被其他活跃分类使用（排除当前分类和已删除的分类）
        let existing = Category::find()
            .filter(CategoryColumn::Name.eq(n))
            .filter(CategoryColumn::Id.ne(id))
            .filter(CategoryColumn::Status.eq(1))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("分类名称已存在".to_string()));
        }

        category_active.name = Set(n.to_string());
    }

    if let Some(d) = description {
        category_active.description = Set(Some(d.to_string()));
    }

    if let Some(s) = sort_order {
        category_active.sort_order = Set(s);
    }

    if let Some(st) = status {
        category_active.status = Set(st);
    }

    category_active.updated_at = Set(Utc::now());
    category_active.update(db).await?;

    tracing::info!("更新分类成功: id={}", id);
    Ok(())
}

/// 删除分类
pub async fn delete_category(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let category = Category::find()
        .filter(CategoryColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("分类不存在".to_string()))?;

    // 逻辑删除：将状态设为禁用
    let mut category_active: ActiveModel = category.into();
    category_active.status = Set(0);
    category_active.updated_at = Set(Utc::now());
    category_active.update(db).await?;

    tracing::info!("删除分类成功: id={}", id);
    Ok(())
}
