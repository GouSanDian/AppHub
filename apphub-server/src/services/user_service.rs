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

    // 2. 检查邮箱是否已存在（空字符串视为 None）
    let email_normalized = email
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if let Some(ref mail) = email_normalized {
        let existing_email = User::find()
            .filter(UserColumn::Email.eq(mail))
            .one(db)
            .await?;
        if existing_email.is_some() {
            return Err(AppError::Conflict("邮箱已被使用".to_string()));
        }
    }

    // 3. 密码加密
    let password_hash = password::hash_password(password_str)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?;

    // 4. 创建用户记录
    let now = Utc::now();
    let user = ActiveModel {
        username: Set(username.to_string()),
        password_hash: Set(password_hash),
        role_id: Set(role_id),
        nickname: Set(nickname.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())),
        email: Set(email_normalized),
        status: Set(1),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let result = user.insert(db).await?;
    tracing::info!("创建用户成功: {} (id: {})", username, result.id);

    Ok(result.id)
}

/// 用户列表查询参数
pub struct ListUsersParams {
    pub page: u64,
    pub page_size: u64,
    pub keyword: Option<String>,
    pub role: Option<String>,
    pub status: Option<i16>,
}

/// 获取用户列表
pub async fn list_users(
    db: &DatabaseConnection,
    params: ListUsersParams,
) -> Result<(Vec<Model>, u64), AppError> {
    use sea_orm::{PaginatorTrait, QuerySelect, QueryFilter, Condition};

    let mut query = User::find();

    // 关键字搜索（用户名、昵称、邮箱）
    if let Some(keyword) = &params.keyword {
        if !keyword.is_empty() {
            let like_pattern = format!("%{}%", keyword);
            let cond = Condition::any()
                .add(UserColumn::Username.like(&like_pattern))
                .add(UserColumn::Nickname.like(&like_pattern))
                .add(UserColumn::Email.like(&like_pattern));
            query = query.filter(cond);
        }
    }

    // 状态筛选
    if let Some(status) = params.status {
        query = query.filter(UserColumn::Status.eq(status));
    }

    // 角色筛选（需要联表查询）
    if let Some(role_name) = &params.role {
        if !role_name.is_empty() {
            use crate::models::role::{Entity as Role, Column as RoleColumn};
            let role_ids: Vec<i64> = Role::find()
                .filter(RoleColumn::Name.eq(role_name))
                .all(db)
                .await?
                .into_iter()
                .map(|r| r.id)
                .collect();
            if !role_ids.is_empty() {
                query = query.filter(UserColumn::RoleId.is_in(role_ids));
            } else {
                // 角色不存在，返回空结果
                return Ok((vec![], 0));
            }
        }
    }

    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let users = paginator.fetch_page(params.page - 1).await?;

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
    role_id: Option<i64>,
) -> Result<(), AppError> {
    let user = User::find()
        .filter(UserColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    let mut user_active: ActiveModel = user.into();

    if let Some(nick) = nickname {
        user_active.nickname = Set(Some(nick.trim().to_string()).filter(|s| !s.is_empty()));
    }

    if let Some(mail) = email {
        let mail_normalized = Some(mail.trim().to_string()).filter(|s| !s.is_empty());
        // 如果邮箱非空，检查是否被其他用户使用
        if let Some(ref m) = mail_normalized {
            let existing = User::find()
                .filter(UserColumn::Email.eq(m))
                .one(db)
                .await?;
            if let Some(existing_user) = existing {
                if existing_user.id != id {
                    return Err(AppError::Conflict("邮箱已被其他用户使用".to_string()));
                }
            }
        }
        user_active.email = Set(mail_normalized);
    }

    if let Some(s) = status {
        user_active.status = Set(s);
    }

    if let Some(rid) = role_id {
        user_active.role_id = Set(rid);
    }

    user_active.updated_at = Set(Utc::now());
    user_active.update(db).await?;

    tracing::info!("更新用户成功: id={}", id);
    Ok(())
}

/// 删除用户（物理删除）
pub async fn delete_user(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let user = User::find()
        .filter(UserColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 不允许删除超级管理员
    if user.role_id == 1 {
        return Err(AppError::Forbidden("不能删除超级管理员".to_string()));
    }

    use sea_orm::DeleteMany;
    User::delete_many()
        .filter(UserColumn::Id.eq(id))
        .exec(db)
        .await?;

    tracing::info!("删除用户成功: id={}", id);
    Ok(())
}

/// 重置用户密码
pub async fn reset_password(
    db: &DatabaseConnection,
    id: i64,
    new_password: &str,
) -> Result<(), AppError> {
    let user = User::find()
        .filter(UserColumn::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    // 密码加密
    let password_hash = password::hash_password(new_password)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?;

    let mut user_active: ActiveModel = user.into();
    user_active.password_hash = Set(password_hash);
    user_active.updated_at = Set(Utc::now());
    user_active.update(db).await?;

    tracing::info!("重置用户密码成功: id={}", id);
    Ok(())
}
