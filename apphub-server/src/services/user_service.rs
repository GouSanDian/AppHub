//! 用户服务

use crate::error::AppError;
use crate::models::user::{ActiveModel, Entity as User, Column as UserColumn, Model};
use crate::utils::password;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use chrono::Utc;

/// 创建用户
pub async fn create_user(
    db: &DatabaseConnection,
    username: &str,
    password_str: &str,
    role_id: i64,
    nickname: Option<&str>,
    email: Option<&str>,
) -> Result<i64, AppError> {
    // 1. 检查用户名是否已存在
    let existing = User::find()
        .filter(UserColumn::Username.eq(username))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("用户名已存在".to_string()));
    }

    // 2. 密码加密
    let password_hash = password::hash_password(password_str)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?;

    // 3. 创建用户记录
    let now = Utc::now();
    let user = ActiveModel {
        username: Set(username.to_string()),
        password_hash: Set(password_hash),
        role_id: Set(role_id),
        nickname: Set(nickname.map(|s| s.to_string())),
        email: Set(email.map(|s| s.to_string())),
        status: Set(1),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let result = user.insert(db).await?;
    tracing::info!("创建用户成功: {} (id: {})", username, result.id);

    Ok(result.id)
}

/// 获取用户列表
pub async fn list_users(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<(Vec<Model>, u64), AppError> {
    use sea_orm::{PaginatorTrait, QuerySelect};

    let paginator = User::find()
        .paginate(db, page_size);

    let total = paginator.num_items().await?;
    let users = paginator.fetch_page(page - 1).await?;

    Ok((users, total))
}

/// 获取用户详情
pub async fn get_user(db: &DatabaseConnection, id: i64) -> Result<Option<Model>, AppError> {
    let user = User::find()
        .filter(UserColumn::Id.eq(id))
        .one(db)
        .await?;
    Ok(user)
}

/// 更新用户
pub async fn update_user(
    db: &DatabaseConnection,
    id: i64,
    nickname: Option<&str>,
    email: Option<&str>,
    status: Option<i16>,
) -> Result<(), AppError> {
    let user = User::find()
        .filter(UserColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    let mut user_active: ActiveModel = user.into();

    if let Some(nick) = nickname {
        user_active.nickname = Set(Some(nick.to_string()));
    }

    if let Some(mail) = email {
        user_active.email = Set(Some(mail.to_string()));
    }

    if let Some(s) = status {
        user_active.status = Set(s);
    }

    user_active.updated_at = Set(Utc::now());
    user_active.update(db).await?;

    tracing::info!("更新用户成功: id={}", id);
    Ok(())
}

/// 删除用户（逻辑删除）
pub async fn delete_user(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let user = User::find()
        .filter(UserColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 逻辑删除：将状态设为禁用
    let mut user_active: ActiveModel = user.into();
    user_active.status = Set(0);
    user_active.updated_at = Set(Utc::now());
    user_active.update(db).await?;

    tracing::info!("删除用户成功: id={}", id);
    Ok(())
}
