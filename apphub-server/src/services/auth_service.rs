//! 认证服务

use crate::error::AppError;
use crate::models::user::{Entity as User, Column as UserColumn};
use crate::models::role::{Entity as Role, Column as RoleColumn};
use crate::utils::{jwt, password};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use chrono::Utc;

/// 登录
pub async fn login(
    db: &DatabaseConnection,
    username: &str,
    password_str: &str,
    jwt_secret: &str,
    jwt_expiration: i64,
    jwt_refresh_expiration: i64,
) -> Result<(String, String, i64), AppError> {
    tracing::info!("用户登录: {}", username);

    // 1. 查询用户
    let user = User::find()
        .filter(UserColumn::Username.eq(username))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("用户名或密码错误".to_string()))?;

    // 2. 检查用户状态
    if user.status != 1 {
        return Err(AppError::Forbidden("用户已被禁用".to_string()));
    }

    // 3. 验证密码
    let valid = password::verify_password(password_str, &user.password_hash)
        .map_err(|_| AppError::Unauthorized("用户名或密码错误".to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized("用户名或密码错误".to_string()));
    }

    // 4. 查询角色信息
    let role = Role::find()
        .filter(RoleColumn::Id.eq(user.role_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;

    // 5. 生成Token
    let user_id_str = user.id.to_string();
    let access_token = jwt::generate_token(
        &user_id_str,
        &user.username,
        &role.name,
        jwt_secret,
        jwt_expiration,
    )
    .map_err(|e| AppError::InternalError(format!("生成Token失败: {}", e)))?;

    let refresh_token = jwt::generate_token(
        &user_id_str,
        &user.username,
        &role.name,
        jwt_secret,
        jwt_refresh_expiration,
    )
    .map_err(|e| AppError::InternalError(format!("生成刷新Token失败: {}", e)))?;

    // 6. 更新最后登录时间
    let mut user_active: crate::models::user::ActiveModel = user.into();
    user_active.last_login_at = sea_orm::ActiveValue::Set(Some(Utc::now()));
    user_active.update(db).await?;

    Ok((access_token, refresh_token, jwt_expiration))
}

/// 验证Token
pub fn verify_token(token: &str, jwt_secret: &str) -> Result<jwt::Claims, AppError> {
    jwt::verify_token(token, jwt_secret)
        .map_err(|e| AppError::Unauthorized(format!("Token验证失败: {}", e)))
}

/// 刷新Token
pub async fn refresh_token(
    db: &DatabaseConnection,
    refresh_token_str: &str,
    jwt_secret: &str,
    jwt_expiration: i64,
    jwt_refresh_expiration: i64,
) -> Result<(String, String, i64), AppError> {
    // 1. 验证刷新Token
    let claims = verify_token(refresh_token_str, jwt_secret)?;

    // 2. 查询用户
    let user_id: i64 = claims.sub.parse()
        .map_err(|_| AppError::Unauthorized("无效的Token".to_string()))?;

    let user = User::find()
        .filter(UserColumn::Id.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("用户不存在".to_string()))?;

    // 3. 检查用户状态
    if user.status != 1 {
        return Err(AppError::Forbidden("用户已被禁用".to_string()));
    }

    // 4. 查询角色
    let role = Role::find()
        .filter(RoleColumn::Id.eq(user.role_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;

    // 5. 生成新的Token
    let new_access_token = jwt::generate_token(
        &user_id.to_string(),
        &user.username,
        &role.name,
        jwt_secret,
        jwt_expiration,
    )
    .map_err(|e| AppError::InternalError(format!("生成Token失败: {}", e)))?;

    let new_refresh_token = jwt::generate_token(
        &user_id.to_string(),
        &user.username,
        &role.name,
        jwt_secret,
        jwt_refresh_expiration,
    )
    .map_err(|e| AppError::InternalError(format!("生成刷新Token失败: {}", e)))?;

    Ok((new_access_token, new_refresh_token, jwt_expiration))
}
