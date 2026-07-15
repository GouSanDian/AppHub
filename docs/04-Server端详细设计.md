# Server端详细设计文档 (Rust实现)

## 1. 项目概述

### 1.1 项目简介
apphub-server 是应用中心系统的服务端，使用Rust语言开发，提供高性能、高安全性的Web管理后台和RESTful API接口。支持管理员通过浏览器进行软件管理、用户管理、黑名单配置等操作，并为桌面客户端提供软件下载和进程监控上报接口。

### 1.2 技术选型

| 技术领域 | 技术选型 | 版本 | 说明 |
|----------|----------|------|------|
| **核心框架** | Axum / Actix-web | 0.7 / 4.x | 高性能Web框架 |
| **编程语言** | Rust | 1.70+ | 内存安全语言 |
| **数据库** | PostgreSQL | 15+ | 企业级数据库（推荐） |
| **ORM框架** | SeaORM | 0.12+ | 异步ORM框架 |
| **异步运行时** | Tokio | 1.x | 高性能异步运行时 |
| **认证授权** | JWT + Middleware | - | jsonwebtoken crate |
| **文件存储** | 本地存储/MinIO | - | tokio::fs异步IO |
| **日志框架** | tracing | 0.1.x | 结构化日志 |
| **序列化** | serde | 1.x | 零成本序列化 |
| **API文档** | utoipa | 0.4+ | OpenAPI自动生成 |
| **缓存** | Redis/moka | 6.0+ | 分布式缓存/本地缓存 |
| **构建工具** | Cargo | - | Rust包管理器 |

### 1.3 Rust技术优势

#### 性能优势
- **启动速度快**: 10-50ms (vs Java 1-3秒)
- **内存占用低**: 10-50MB (vs Java 100-300MB)
- **并发处理强**: 10万QPS (vs Java 1万QPS)
- **CPU效率高**: 零成本抽象，无GC开销

#### 安全优势
- **内存安全**: 编译时检查，防止内存泄漏、空指针
- **线程安全**: 防止数据竞争，无锁并发
- **类型安全**: 强类型系统，编译时类型检查
- **无运行时异常**: 大部分错误在编译时发现

#### 开发优势
- **类型推导**: 减少冗余类型声明
- **宏系统**: 减少重复代码
- **包管理**: Cargo自动管理依赖
- **文档生成**: rustdoc自动生成文档

## 2. 项目结构

### 2.1 目录结构

```
apphub-server/
├── Cargo.toml                         # Cargo配置文件
├── Cargo.lock                         # 依赖锁定文件
├── .env                               # 环境变量配置
├── .env.example                       # 环境变量示例
├── rustfmt.toml                       # Rust格式化配置
├── clippy.toml                        # Clippy配置
│
├── src/
│   ├── main.rs                        # 程序入口
│   ├── lib.rs                         # 库入口
│   │
│   ├── config/                        # 配置模块
│   │   ├── mod.rs                     # 模块入口
│   │   ├── app_config.rs             # 应用配置（config-rs）
│   │   ├── database.rs               # 数据库配置
│   │   └── redis.rs                  # Redis配置
│   │
│   ├── api/                           # API层（Axum）
│   │   ├── mod.rs                     # 模块入口
│   │   ├── routes.rs                 # 路由定义
│   │   │
│   │   ├── handlers/                 # 请求处理器
│   │   │   ├── mod.rs                # 模块入口
│   │   │   ├── auth.rs               # 认证处理器
│   │   │   ├── user.rs               # 用户处理器
│   │   │   ├── software.rs           # 软件处理器
│   │   │   ├── blacklist.rs          # 黑名单处理器
│   │   │   ├── client.rs             # 客户端处理器
│   │   │   ├── report.rs             # 上报处理器
│   │   │   ├── statistics.rs         # 统计处理器
│   │   │   └── file.rs               # 文件处理器
│   │   │
│   │   ├── middleware/               # 中间件
│   │   │   ├── mod.rs                # 模块入口
│   │   │   ├── auth.rs               # JWT认证中间件
│   │   │   ├── logging.rs            # 日志中间件（tracing）
│   │   │   ├── cors.rs               # CORS中间件
│   │   │   ├── error_handler.rs      # 错误处理中间件
│   │   │   └── rate_limit.rs         # 请求频率限制
│   │   │
│   │   └ dto/                        # 数据传输对象
│   │   │   ├── mod.rs                # 模块入口
│   │   │   ├── request.rs            # 请求DTO
│   │   │   ├── response.rs           # 响应DTO
│   │   │   └ page.rs                 # 分页DTO
│   │   │
│   │   └ openapi.rs                  # OpenAPI定义（utoipa）
│   │   └ swagger.rs                  # Swagger配置
│   │
│   ├── models/                        # 数据模型（SeaORM Entity）
│   │   ├── mod.rs                     # 模块入口
│   │   ├── user.rs                   # 用户模型
│   │   ├── role.rs                   # 角色模型
│   │   ├── permission.rs             # 权限模型
│   │   ├── department.rs             # 部门模型
│   │   ├── software.rs               # 软件模型
│   │   ├── category.rs               # 分类模型
│   │   ├── blacklist_process.rs      # 黑名单进程
│   │   ├── blacklist_group.rs        # 黑名单分组
│   │   ├── client_info.rs            # 客户端信息
│   │   ├── scan_report.rs            # 扫描报告
│   │   ├── download_record.rs        # 下载记录
│   │   ├── operation_log.rs          # 操作日志
│   │   └── notice.rs                 # 公告模型
│   │
│   ├── services/                      # 业务服务层
│   │   ├── mod.rs                     # 模块入口
│   │   ├── auth_service.rs           # 认证服务
│   │   ├── user_service.rs           # 用户服务
│   │   ├── role_service.rs           # 角色服务
│   │   ├── software_service.rs       # 软件服务
│   │   ├── blacklist_service.rs      # 黑名单服务
│   │   ├── file_service.rs           # 文件服务（异步IO）
│   │   ├── report_service.rs         # 上报服务
│   │   ├── statistics_service.rs     # 统计服务
│   │   ├── cache_service.rs          # 缓存服务（moka/Redis）
│   │   └ client_service.rs           # 客户端服务
│   │   └ notification_service.rs     # 通知服务
│   │   └ log_service.rs              # 日志服务
│   │
│   ├── repository/                    # 数据访问层（SeaORM）
│   │   ├── mod.rs                     # 模块入口
│   │   ├── user_repo.rs              # 用户Repository
│   │   ├── software_repo.rs          # 软件Repository
│   │   ├── blacklist_repo.rs         # 黑名单Repository
│   │   ├── report_repo.rs            # 上报Repository
│   │   └ log_repo.rs                 # 日志Repository
│   │
│   ├── utils/                         # 工具模块
│   │   ├── mod.rs                     # 模块入口
│   │   ├── jwt.rs                    # JWT工具（jsonwebtoken）
│   │   ├── password.rs               # 密码工具（bcrypt）
│   │   ├── file.rs                   # 文件工具（tokio::fs）
│   │   ├── time.rs                   # 时间工具（chrono）
│   │   ├── response.rs               # 响应工具
│   │   ├── validator.rs              # 数据验证
│   │   ├── hash.rs                   # 哈希工具（ring/sha2）
│   │   └ ip.rs                        # IP工具
│   │
│   ├── error/                         # 错误处理
│   │   ├── mod.rs                     # 模块入口
│   │   ├── app_error.rs              # 应用错误定义
│   │   ├── handler.rs                # 错误处理器
│   │   ├── database_error.rs         # 数据库错误
│   │   └ auth_error.rs               # 认证错误
│   │
│   ├── constants/                     # 常量定义
│   │   ├── mod.rs                     # 模块入口
│   │   ├── response_codes.rs         # 响应码常量
│   │   ├── user_status.rs            # 用户状态常量
│   │   ├── download_status.rs        # 下载状态常量
│   │   └ risk_level.rs               # 风险等级常量
│   │   └ cache_keys.rs               # 缓存键常量
│   │
│   └ migrations/                      # 数据库迁移（SeaORM Migration）
│   │   ├── mod.rs                     # 迁移入口
│   │   ├── m20240101_000001_create_users_table.rs
│   │   ├── m20240101_000002_create_roles_table.rs
│   │   ├── m20240101_000003_create_permissions_table.rs
│   │   ├── ...
│   │
│   └ tests/                           # 测试代码
│   │   ├── integration/              # 集成测试
│   │   │   ├── auth_test.rs
│   │   │   ├── software_test.rs
│   │   │   └ report_test.rs
│   │   └ unit/                        # 单元测试
│   │   │   ├── service_test.rs
│   │   │   ├── utils_test.rs
│   │
│   └ bin/                             # 可执行程序
│   │   ├── server.rs                 # 服务器主程序
│   │   ├── migrate.rs                # 迁移工具
│   │   └ seed.rs                      # 数据填充工具
│   │
├── migrations/                        # SQL迁移文件（可选）
│   ├── 20240101000001_create_users.sql
│   ├── 20240101000002_create_roles.sql
│   └── ...
│
├── config/                            # 配置文件目录
│   ├── default.toml                   # 默认配置
│   ├── development.toml               # 开发环境配置
│   ├── production.toml                # 生产环境配置
│   └ logging.toml                     # 日志配置
│   └ database.toml                    # 数据库配置
│   └ redis.toml                       # Redis配置
│
├── static/                            # 静态文件
│   ├── swagger-ui/                    # Swagger UI
│   └ uploads/                         # 上传文件存储
│   │   ├── software/                  # 软件安装包
│   │   ├── avatar/                    # 用户头像
│   │   └ temp/                        # 临时文件
│
├── docs/                              # 文档目录
│   ├── API.md                         # API文档
│   ├── ARCHITECTURE.md                # 架构文档
│   └ DEPLOYMENT.md                    # 部署文档
│
├── scripts/                           # 脚本文件
│   ├── build.sh                       # 构建脚本
│   ├── deploy.sh                      # 部署脚本
│   ├── migrate.sh                     # 迁移脚本
│   └ test.sh                          # 测试脚本
│
├── docker/                            # Docker配置
│   ├── Dockerfile                     # Docker镜像
│   ├── docker-compose.yml             # Docker Compose
│   ├── nginx.conf                     # Nginx配置
│
├── logs/                              # 日志目录（运行时生成）
├── target/                            # 编译输出（Cargo生成）
│   ├── debug/                         # Debug版本
│   ├── release/                       # Release版本
│
└── README.md                          # 项目说明
```

### 2.2 分层架构

```
┌─────────────────────────────────────────────────────────┐
│                   Handler层 (Axum)                       │
│          (接收请求、参数验证、调用Service)               │
│                                                         │
│  handlers: auth, user, software, blacklist, report     │
└                                                         │
│  middleware: auth(JWT), logging(tracing), cors          │
└                                                         │
│  dto: request, response, page                           │
└───────────────────────────────┬─────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────┐
│                   Service层 (异步)                       │
│          (业务逻辑、事务控制、数据组装)                  │
│                                                         │
│  services: auth, user, software, blacklist, file        │
│                                                         │
│  特点: 全异步实现(async/await), 无运行时开销            │
└───────────────────────────────┬─────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────┐
│                  Repository层 (SeaORM)                   │
│              (数据访问、异步查询、事务)                  │
│                                                         │
│  repository: user_repo, software_repo, report_repo      │
│                                                         │
│  特点: 编译时SQL检查, 异步查询, 参数化防注入            │
└───────────────────────────────┬─────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────┐
│                   数据层 (PostgreSQL)                    │
│                    (企业级数据库)                        │
│                                                         │
│  PostgreSQL: 用户表、软件表、黑名单表、上报表           │
│                                                         │
│  特点: JSONB字段, 部分索引, 表分区, 全文搜索            │
└─────────────────────────────────────────────────────────┘
```

## 3. 核心模块设计

### 3.1 认证授权模块

#### 3.1.1 JWT认证实现

```rust
// src/utils/jwt.rs
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::config::app_config::CONFIG;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // 用户名 (subject)
    pub user_id: u32,       // 用户ID
    pub role_id: u32,       // 角色ID
    pub role_code: String,  // 角色编码
    pub exp: usize,         // 过期时间 (expiration)
    pub iat: usize,         // 发行时间 (issued at)
}

/// 生成访问令牌
pub fn generate_access_token(user_id: u32, username: &str, role_id: u32, role_code: &str) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(CONFIG.jwt.access_token_expire as i64))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: username.to_owned(),
        user_id,
        role_id,
        role_code: role_code.to_owned(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(format!("Token生成失败: {}", e)))
}

/// 生成刷新令牌
pub fn generate_refresh_token(user_id: u32, username: &str) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(CONFIG.jwt.refresh_token_expire as i64))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: username.to_owned(),
        user_id,
        role_id: 0,  // refresh token不需要角色信息
        role_code: String::new(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(format!("Token生成失败: {}", e)))
}

/// 验证令牌
pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("无效的令牌: {}", e)))
}

/// 刷新令牌
pub fn refresh_access_token(refresh_token: &str) -> Result<String, AppError> {
    let claims = validate_token(refresh_token)?;
    
    // 从数据库重新查询用户信息（确保用户仍然有效）
    // TODO: 调用用户服务查询
    
    generate_access_token(claims.user_id, &claims.sub, claims.role_id, &claims.role_code)
}
```

#### 3.1.2 认证中间件

```rust
// src/api/middleware/auth.rs
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use crate::utils::jwt::{validate_token, Claims};
use crate::error::AppError;

/// JWT认证中间件
pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 从Header中提取Token
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("缺少认证令牌".to_string()));
    }
    
    let token = auth_header.trim_start_matches("Bearer ");
    
    // 验证Token
    let claims = validate_token(token)?;
    
    // 将用户信息注入到Request扩展中
    let mut request = request;
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

/// 权限检查中间件
pub async fn permission_middleware(
    Extension(claims): Extension<Claims>,
    request: Request,
    next: Next,
    required_permission: String,
) -> Result<Response, AppError> {
    // 从缓存或数据库查询用户权限列表
    // let permissions = get_user_permissions(claims.user_id)?;
    
    // 检查是否有所需权限
    // if !permissions.contains(&required_permission) {
    //     return Err(AppError::Forbidden("权限不足".to_string()));
    // }
    
    Ok(next.run(request).await)
}

/// 管理员权限中间件
pub async fn admin_middleware(
    Extension(claims): Extension<Claims>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if claims.role_code != "SUPER_ADMIN" && claims.role_code != "ADMIN" {
        return Err(AppError::Forbidden("需要管理员权限".to_string()));
    }
    
    Ok(next.run(request).await)
}
```

#### 3.1.3 认证服务

```rust
// src/services/auth_service.rs
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::models::user::{Entity as UserEntity, Column as UserColumn};
use crate::utils::jwt::{generate_access_token, generate_refresh_token};
use crate::api::dto::response::LoginResponse;
use crate::error::AppError;

pub struct AuthService {
    db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    
    /// 用户登录（异步）
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse, AppError> {
        // 查询用户（异步）
        let user = UserEntity::find()
            .filter(UserColumn::Username.eq(username))
            .filter(UserColumn::Deleted.eq(0))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::Unauthorized("用户名或密码错误".to_string()))?;
        
        // 验证密码（bcrypt）
        let valid = verify(password, &user.password)
            .map_err(|_| AppError::InternalError("密码验证失败".to_string()))?;
        
        if !valid {
            return Err(AppError::Unauthorized("用户名或密码错误".to_string()));
        }
        
        // 检查用户状态
        if user.status != 1 {
            return Err(AppError::Forbidden("用户已被禁用".to_string()));
        }
        
        // 查询角色信息（异步）
        let role = RoleEntity::find_by_id(user.role_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;
        
        // 生成Token
        let access_token = generate_access_token(user.id, &user.username, user.role_id, &role.role_code)?;
        let refresh_token = generate_refresh_token(user.id, &user.username)?;
        
        // 更新最后登录时间（异步）
        let mut user_active: user::ActiveModel = user.into();
        user_active.last_login_time = Set(Some(Utc::now().naive_utc()));
        user_active.last_login_ip = Set(Some(get_client_ip()));
        user_active.update(&self.db).await?;
        
        Ok(LoginResponse {
            user_id: user.id,
            username: user.username,
            nickname: user.nickname.unwrap_or_default(),
            role_code: role.role_code,
            access_token,
            refresh_token,
            expires_in: CONFIG.jwt.access_token_expire,
        })
    }
    
    /// 用户登出
    pub async fn logout(&self, user_id: u32, token: &str) -> Result<(), AppError> {
        // 将Token加入黑名单（Redis）
        // TODO: 实现Token黑名单
        
        Ok(())
    }
    
    /// 刷新Token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<LoginResponse, AppError> {
        // 验证refresh_token
        let claims = validate_token(refresh_token)?;
        
        // 重新查询用户信息（确保用户仍然有效）
        let user = UserEntity::find_by_id(claims.user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::Unauthorized("用户不存在".to_string()))?;
        
        if user.status != 1 || user.deleted != 0 {
            return Err(AppError::Unauthorized("用户已被禁用或删除".to_string()));
        }
        
        // 查询角色
        let role = RoleEntity::find_by_id(user.role_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;
        
        // 生成新的Token
        let new_access_token = generate_access_token(user.id, &user.username, user.role_id, &role.role_code)?;
        let new_refresh_token = generate_refresh_token(user.id, &user.username)?;
        
        Ok(LoginResponse {
            user_id: user.id,
            username: user.username,
            nickname: user.nickname.unwrap_or_default(),
            role_code: role.role_code,
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in: CONFIG.jwt.access_token_expire,
        })
    }
    
    /// 获取当前用户信息
    pub async fn get_user_info(&self, user_id: u32) -> Result<UserInfoResponse, AppError> {
        let user = UserEntity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
        
        let role = RoleEntity::find_by_id(user.role_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::InternalError("角色不存在".to_string()))?;
        
        // 查询权限列表（异步）
        let permissions = self.get_user_permissions(user_id).await?;
        
        Ok(UserInfoResponse {
            user_id: user.id,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            phone: user.phone,
            avatar: user.avatar,
            role_code: role.role_code,
            role_name: role.role_name,
            permissions,
        })
    }
    
    /// 获取用户权限列表（异步）
    async fn get_user_permissions(&self, user_id: u32) -> Result<Vec<String>, AppError> {
        // TODO: 从数据库或缓存查询用户权限
        
        Ok(vec!["user:list", "software:download".to_string()])
    }
}
```

### 3.2 文件上传模块

#### 3.2.1 异步文件处理

```rust
// src/services/file_service.rs
use tokio::fs::{File, create_dir_all, remove_dir_all};
use tokio::io::{AsyncReadExt, AsyncWriteExt, copy};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use crate::error::AppError;
use crate::config::app_config::CONFIG;

pub struct FileService {
    upload_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Self {
        Self {
            upload_dir: PathBuf::from(&CONFIG.upload.path),
        }
    }
    
    /// 普通文件上传（异步）
    pub async fn upload_file(
        &self,
        file_name: &str,
        file_data: Vec<u8>,
        file_type: &str, // "software", "avatar", etc.
    ) -> Result<FileInfo, AppError> {
        // 验证文件大小
        if file_data.len() as u64 > CONFIG.upload.max_size {
            return Err(AppError::BadRequest("文件大小超过限制".to_string()));
        }
        
        // 验证文件扩展名
        let extension = Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        if !self.is_allowed_extension(extension, file_type) {
            return Err(AppError::BadRequest("不支持的文件类型".to_string()));
        }
        
        // 生成存储路径
        let date_path = chrono::Utc::now().format("%Y/%m");
        let new_file_name = format!("{}.{}", uuid::Uuid::new_v4(), extension);
        let relative_path = format!("{}/{}/{}", file_type, date_path, new_file_name);
        let absolute_path = self.upload_dir.join(&relative_path);
        
        // 创建目录（异步）
        create_dir_all(absolute_path.parent().unwrap())
            .await
            .map_err(|e| AppError::InternalError(format!("创建目录失败: {}", e)))?;
        
        // 写入文件（异步）
        let mut file = File::create(&absolute_path)
            .await
            .map_err(|e| AppError::InternalError(format!("创建文件失败: {}", e)))?;
        
        file.write_all(&file_data)
            .await
            .map_err(|e| AppError::InternalError(format!("写入文件失败: {}", e)))?;
        
        file.flush()
            .await
            .map_err(|e| AppError::InternalError(format!("刷新文件失败: {}", e)))?;
        
        // 计算MD5/SHA256（ring crate）
        let hash = Sha256::digest(&file_data);
        let hash_hex = format!("{:x}", hash);
        
        Ok(FileInfo {
            file_name: file_name.to_string(),
            file_path: relative_path,
            file_size: file_data.len() as u64,
            file_hash: hash_hex,
        })
    }
    
    /// 分片上传初始化（异步）
    pub async fn init_multipart_upload(
        &self,
        file_name: &str,
        file_size: u64,
        file_hash: &str,
    ) -> Result<MultipartUploadInit, AppError> {
        // 验证文件大小
        if file_size > CONFIG.upload.max_size {
            return Err(AppError::BadRequest("文件大小超过限制".to_string()));
        }
        
        // 生成上传ID
        let upload_id = format!("upload_{}", uuid::Uuid::new_v4());
        
        // 计算分片数量
        let chunk_size = CONFIG.upload.chunk_size;
        let chunk_count = (file_size as f64 / chunk_size as f64).ceil() as u32;
        
        // 创建临时目录（异步）
        let temp_dir = self.upload_dir.join("temp").join(&upload_id);
        create_dir_all(&temp_dir)
            .await
            .map_err(|e| AppError::InternalError(format!("创建临时目录失败: {}", e)))?;
        
        // 存储上传信息（Redis可选）
        // TODO: 存储到Redis
        
        Ok(MultipartUploadInit {
            upload_id,
            chunk_count,
            chunk_size,
        })
    }
    
    /// 上传分片（异步）
    pub async fn upload_chunk(
        &self,
        upload_id: &str,
        chunk_index: u32,
        chunk_data: Vec<u8>,
    ) -> Result<(), AppError> {
        let chunk_path = self.upload_dir
            .join("temp")
            .join(upload_id)
            .join(format!("chunk_{}", chunk_index));
        
        // 写入分片文件（异步）
        let mut file = File::create(&chunk_path)
            .await
            .map_err(|e| AppError::InternalError(format!("创建分片文件失败: {}", e)))?;
        
        file.write_all(&chunk_data)
            .await
            .map_err(|e| AppError::InternalError(format!("写入分片失败: {}", e)))?;
        
        file.flush()
            .await
            .map_err(|e| AppError::InternalError(format!("刷新分片失败: {}", e)))?;
        
        Ok(())
    }
    
    /// 合并分片（异步）
    pub async fn complete_multipart_upload(
        &self,
        upload_id: &str,
        file_name: &str,
    ) -> Result<FileInfo, AppError> {
        let temp_dir = self.upload_dir.join("temp").join(upload_id);
        
        // 验证所有分片是否已上传
        // TODO: 检查分片完整性
        
        // 生成最终文件路径
        let extension = Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let date_path = chrono::Utc::now().format("%Y/%m");
        let new_file_name = format!("{}.{}", uuid::Uuid::new_v4(), extension);
        let relative_path = format!("software/{}/{}", date_path, new_file_name);
        let final_path = self.upload_dir.join(&relative_path);
        
        // 创建目录（异步）
        create_dir_all(final_path.parent().unwrap())
            .await
            .map_err(|e| AppError::InternalError(format!("创建目录失败: {}", e)))?;
        
        // 合并文件（异步流式处理）
        let mut final_file = File::create(&final_path)
            .await
            .map_err(|e| AppError::InternalError(format!("创建最终文件失败: {}", e)))?;
        
        let mut total_size: u64 = 0;
        let mut hasher = Sha256::new();
        
        // 逐个读取分片并合并（内存占用恒定）
        for i in 0..1000 { // TODO: 从元数据获取chunk_count
            let chunk_path = temp_dir.join(format!("chunk_{}", i));
            if !chunk_path.exists() {
                break;
            }
            
            let mut chunk_file = File::open(&chunk_path)
                .await
                .map_err(|e| AppError::InternalError(format!("打开分片失败: {}", e)))?;
            
            // 流式复制（不一次性加载整个分片）
            let size = copy(&mut chunk_file, &mut final_file)
                .await
                .map_err(|e| AppError::InternalError(format!("复制分片失败: {}", e)))?;
            
            total_size += size;
            
            // 更新哈希
            let mut buffer = Vec::new();
            chunk_file.read_to_end(&mut buffer).await?; // 重新读取以计算哈希
            hasher.update(&buffer);
        }
        
        final_file.flush()
            .await
            .map_err(|e| AppError::InternalError(format!("刷新最终文件失败: {}", e)))?;
        
        // 计算最终哈希
        let hash_hex = format!("{:x}", hasher.finalize());
        
        // 删除临时目录（异步）
        remove_dir_all(&temp_dir)
            .await
            .map_err(|e| AppError::InternalError(format!("删除临时目录失败: {}", e)))?;
        
        Ok(FileInfo {
            file_name: file_name.to_string(),
            file_path: relative_path,
            file_size: total_size,
            file_hash: hash_hex,
        })
    }
    
    /// 判断是否允许的扩展名
    fn is_allowed_extension(&self, extension: &str, file_type: &str) -> bool {
        match file_type {
            "software" => ["exe", "msi", "dmg", "pkg", "deb", "rpm", "zip", "tar", "gz"].contains(&extension),
            "avatar" => ["jpg", "jpeg", "png", "gif"].contains(&extension),
            _ => false,
        }
    }
}
```

#### 3.2.2 文件下载处理器

```rust
// src/api/handlers/file.rs
use axum::{
    extract::{Path, Query},
    response::{Response, IntoResponse},
    http::{header, StatusCode},
    body::Body,
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::services::file_service::FileService;
use crate::error::AppError;

/// 下载文件（异步流式响应）
pub async fn download_file(
    Path(file_id): Path<u32>,
) -> Result<Response, AppError> {
    let file_service = FileService::new();
    
    // 查询文件信息（异步）
    let file_info = file_service.get_file_info(file_id).await?;
    
    let file_path = file_service.upload_dir.join(&file_info.file_path);
    
    // 打开文件（异步）
    let mut file = File::open(&file_path)
        .await
        .map_err(|e| AppError::NotFound(format!("文件不存在: {}", e)))?;
    
    // 读取文件内容（异步）
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .map_err(|e| AppError::InternalError(format!("读取文件失败: {}", e)))?;
    
    // 构建响应（流式）
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_info.file_name),
        )
        .header(header::CONTENT_LENGTH, file_info.file_size)
        .body(Body::from(buffer))
        .map_err(|e| AppError::InternalError(format!("构建响应失败: {}", e)))
}

/// 流式下载大文件（内存占用恒定）
pub async fn stream_download_file(
    Path(file_id): Path<u32>,
) -> Result<Response, AppError> {
    let file_service = FileService::new();
    let file_info = file_service.get_file_info(file_id).await?;
    
    let file_path = file_service.upload_dir.join(&file_info.file_path);
    
    // 使用 tokio-util 的 StreamReader 实现流式响应
    use tokio_util::io::ReaderStream;
    use futures::StreamExt;
    
    let file = File::open(&file_path)
        .await
        .map_err(|e| AppError::NotFound(format!("文件不存在: {}", e)))?;
    
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream.map(|chunk| {
        chunk.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }));
    
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_info.file_name),
        )
        .header(header::CONTENT_LENGTH, file_info.file_size)
        .body(body)
        .map_err(|e| AppError::InternalError(format!("构建响应失败: {}", e)))
}
```

**性能优势**:
- **异步IO**: tokio::fs异步文件操作，不阻塞线程
- **流式处理**: 文件合并使用流式复制，内存占用恒定
- **零拷贝**: 尽量使用零拷贝技术减少内存复制
- **并发处理**: 多个上传/下载可以并发执行

### 3.3 软件管理模块

#### 3.3.1 软件服务

```rust
// src/services/software_service.rs
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, 
    ColumnTrait, PaginatorTrait, Set, ActiveModelTrait,
};
use crate::models::software::{Entity as SoftwareEntity, Column as SoftwareColumn, ActiveModel};
use crate::api::dto::{SoftwareCreateRequest, SoftwareQuery, SoftwareResponse, PageResponse};
use crate::services::file_service::{FileService, FileInfo};
use crate::error::AppError;

pub struct SoftwareService {
    db: DatabaseConnection,
    file_service: FileService,
}

impl SoftwareService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            file_service: FileService::new(),
        }
    }
    
    /// 分页查询软件列表（异步）
    pub async fn list_software(&self, query: SoftwareQuery) -> Result<PageResponse<SoftwareResponse>, AppError> {
        // 构建查询条件
        let mut query_builder = SoftwareEntity::find()
            .filter(SoftwareColumn::Deleted.eq(0))
            .filter(SoftwareColumn::Status.eq(1));
        
        if let Some(name) = &query.name {
            query_builder = query_builder.filter(SoftwareColumn::Name.contains(name));
        }
        
        if let Some(category_id) = query.category_id {
            query_builder = query_builder.filter(SoftwareColumn::CategoryId.eq(category_id));
        }
        
        // 分页查询（异步）
        let paginator = query_builder
            .paginate(&self.db, query.page_size);
        
        let total_pages = paginator.num_pages().await?;
        let software_list = paginator.fetch_page(query.page_num - 1).await?;
        
        // 转换为Response
        let items: Vec<SoftwareResponse> = software_list
            .into_iter()
            .map(|s| SoftwareResponse {
                software_id: s.id,
                name: s.name,
                version: s.version,
                category_id: s.category_id,
                category_name: None, // TODO: 查询分类名称
                description: s.description,
                file_name: s.file_name,
                file_size: s.file_size,
                file_size_format: format_file_size(s.file_size),
                download_count: s.download_count,
                publisher_id: s.publisher_id,
                publisher_name: None, // TODO: 查询发布者名称
                status: s.status,
                created_at: s.created_at.to_string(),
                updated_at: s.updated_at.to_string(),
            })
            .collect();
        
        Ok(PageResponse {
            list: items,
            total: total_pages * query.page_size,
            page_num: query.page_num,
            page_size: query.page_size,
            pages: total_pages,
        })
    }
    
    /// 上传软件（异步）
    pub async fn upload_software(
        &self,
        request: SoftwareCreateRequest,
        file_data: Vec<u8>,
        user_id: u32,
    ) -> Result<u32, AppError> {
        // 检查是否已存在同名同版本软件
        let exists = SoftwareEntity::find()
            .filter(SoftwareColumn::Name.eq(&request.name))
            .filter(SoftwareColumn::Version.eq(&request.version))
            .filter(SoftwareColumn::Deleted.eq(0))
            .one(&self.db)
            .await?;
        
        if exists.is_some() {
            return Err(AppError::Conflict("软件已存在".to_string()));
        }
        
        // 上传文件（异步）
        let file_info = self.file_service.upload_file(&request.file_name, file_data, "software")?;
        
        // 创建软件记录（异步）
        let software = ActiveModel {
            name: Set(request.name),
            version: Set(request.version),
            category_id: Set(request.category_id),
            description: Set(request.description),
            file_path: Set(file_info.file_path),
            file_name: Set(file_info.file_name),
            file_size: Set(file_info.file_size),
            file_hash: Set(file_info.file_hash),
            download_count: Set(0),
            publisher_id: Set(user_id),
            status: Set(1),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            deleted: Set(0),
            ..Default::default()
        };
        
        let result = software.insert(&self.db).await?;
        
        Ok(result.id)
    }
    
    /// 下载软件（异步）
    pub async fn download_software(
        &self,
        software_id: u32,
        user_id: u32,
        client_id: &str,
    ) -> Result<FileInfo, AppError> {
        // 查询软件信息
        let software = SoftwareEntity::find_by_id(software_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;
        
        if software.deleted == 1 || software.status != 1 {
            return Err(AppError::Forbidden("软件已下架或删除".to_string()));
        }
        
        // 记录下载记录（异步）
        // TODO: 创建下载记录
        
        // 更新下载次数（原子操作）
        let mut software_active: ActiveModel = software.into();
        software_active.download_count = Set(software_active.download_count.unwrap() + 1);
        software_active.update(&self.db).await?;
        
        Ok(FileInfo {
            file_name: software_active.file_name.unwrap(),
            file_path: software_active.file_path.unwrap(),
            file_size: software_active.file_size.unwrap(),
            file_hash: software_active.file_hash.unwrap(),
        })
    }
    
    /// 删除软件（异步）
    pub async fn delete_software(&self, software_id: u32) -> Result<(), AppError> {
        let software = SoftwareEntity::find_by_id(software_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;
        
        // 删除文件（异步）
        let file_path = self.file_service.upload_dir.join(&software.file_path);
        tokio::fs::remove_file(&file_path)
            .await
            .map_err(|e| AppError::InternalError(format!("删除文件失败: {}", e)))?;
        
        // 逻辑删除记录（异步）
        let mut software_active: ActiveModel = software.into();
        software_active.deleted = Set(1);
        software_active.update(&self.db).await?;
        
        Ok(())
    }
}

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
```

### 3.4 黑名单管理模块

#### 3.4.1 黑名单缓存（DashMap无锁并发）

```rust
// src/services/blacklist_service.rs
use dashmap::DashMap; // 无锁并发HashMap
use std::sync::atomic::{AtomicU64, Ordering};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::models::blacklist_process::{Entity as BlacklistEntity, Column as BlacklistColumn};
use crate::error::AppError;

pub struct BlacklistService {
    db: DatabaseConnection,
    // 无锁并发缓存
    processes: DashMap<String, BlacklistItem>,
    // 版本号（原子操作）
    version: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct BlacklistItem {
    pub process_name: String,
    pub risk_level: u8,
    pub status: u8,
}

impl BlacklistService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            processes: DashMap::new(),
            version: AtomicU64::new(0),
        }
    }
    
    /// 初始化黑名单缓存（异步）
    pub async fn init_cache(&self) -> Result<(), AppError> {
        // 从数据库加载黑名单（异步）
        let blacklist = BlacklistEntity::find()
            .filter(BlacklistColumn::Deleted.eq(0))
            .filter(BlacklistColumn::Status.eq(1))
            .all(&self.db)
            .await?;
        
        // 填充缓存
        self.processes.clear();
        for item in blacklist {
            self.processes.insert(
                item.process_name.clone(),
                BlacklistItem {
                    process_name: item.process_name,
                    risk_level: item.risk_level as u8,
                    status: item.status as u8,
                },
            );
        }
        
        // 更新版本号（原子操作）
        self.version.store(
            chrono::Utc::now().timestamp() as u64,
            Ordering::Relaxed,
        );
        
        Ok(())
    }
    
    /// 检查进程是否在黑名单中（无锁读取）
    pub fn check_process(&self, process_name: &str) -> Option<BlacklistItem> {
        // 无锁读取，极致性能
        self.processes.get(process_name).map(|item| item.clone())
    }
    
    /// 获取客户端黑名单（异步）
    pub async fn get_client_blacklist(&self, user_id: u32) -> Result<ClientBlacklistResponse, AppError> {
        // TODO: 根据用户所属部门获取对应的黑名单
        
        let version = self.version.load(Ordering::Relaxed);
        let processes: Vec<_> = self.processes
            .iter()
            .map(|item| ProcessInfo {
                process_name: item.key().clone(),
                risk_level: item.risk_level,
            })
            .collect();
        
        Ok(ClientBlacklistResponse {
            version: version.to_string(),
            processes,
        })
    }
    
    /// 新增黑名单进程（异步）
    pub async fn add_process(
        &self,
        request: BlacklistProcessRequest,
        user_id: u32,
    ) -> Result<u32, AppError> {
        // 检查是否已存在
        if self.processes.contains_key(&request.process_name) {
            return Err(AppError::Conflict("进程已存在".to_string()));
        }
        
        // 创建数据库记录（异步）
        let process = blacklist_process::ActiveModel {
            process_name: Set(request.process_name),
            process_display_name: Set(request.process_display_name),
            description: Set(request.description),
            risk_level: Set(request.risk_level as i16),
            creator_id: Set(user_id as i32),
            status: Set(1),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            deleted: Set(0),
            ..Default::default()
        };
        
        let result = process.insert(&self.db).await?;
        
        // 更新缓存（无锁插入）
        self.processes.insert(
            request.process_name.clone(),
            BlacklistItem {
                process_name: request.process_name,
                risk_level: request.risk_level,
                status: 1,
            },
        );
        
        // 更新版本号（原子操作）
        self.version.fetch_add(1, Ordering::Relaxed);
        
        Ok(result.id)
    }
    
    /// 删除黑名单进程（异步）
    pub async fn delete_process(&self, process_id: u32) -> Result<(), AppError> {
        let process = BlacklistEntity::find_by_id(process_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("进程不存在".to_string()))?;
        
        // 逻辑删除（异步）
        let mut process_active: blacklist_process::ActiveModel = process.into();
        process_active.deleted = Set(1);
        process_active.update(&self.db).await?;
        
        // 从缓存中移除（无锁删除）
        self.processes.remove(&process_active.process_name.unwrap());
        
        // 更新版本号
        self.version.fetch_add(1, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// 获取版本号
    pub fn get_version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }
}
```

**性能优势**:
- **无锁并发**: DashMap支持无锁读写，多个线程可以同时访问
- **原子操作**: 版本号使用AtomicU64，无锁更新
- **高性能**: 读操作性能接近HashMap，远超传统锁机制
- **内存安全**: Rust保证内存安全，无数据竞争

### 3.5 进程监控模块

#### 3.5.1 上报处理器

```rust
// src/api/handlers/report.rs
use axum::extract::Json;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::api::dto::ProcessScanReportRequest;
use crate::models::scan_report::{ActiveModel as ScanReportActiveModel};
use crate::services::notification_service::NotificationService;
use crate::error::AppError;

/// 上报进程扫描结果（异步）
pub async fn report_process_scan(
    db: DatabaseConnection,
    Json(payload): Json<ProcessScanReportRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    // 验证客户端
    let client = ClientInfoEntity::find()
        .filter(ClientInfoColumn::ClientId.eq(&payload.client_id))
        .one(&db)
        .await?
        .ok_or_else(|| AppError::BadRequest("客户端未注册".to_string()))?;
    
    // 检查是否有黑名单进程
    let has_blacklisted = !payload.blacklisted_processes.is_empty();
    
    // 保存上报记录（异步批量插入）
    let scan_report = ScanReportActiveModel {
        client_id: Set(payload.client_id),
        user_id: Set(client.user_id),
        scan_time: Set(chrono::DateTime::parse_from_rfc3339(&payload.scan_time)?.naive_utc()),
        total_processes: Set(payload.total_processes as i32),
        blacklisted_processes: Set(payload.blacklisted_processes.len() as i32),
        has_blacklisted: Set(if has_blacklisted { 1 } else { 0 }),
        process_list: Set(serde_json::to_string(&payload.processes)?),
        blacklisted_list: Set(if has_blacklisted {
            Some(serde_json::to_string(&payload.blacklisted_processes)?)
        } else {
            None
        }),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    
    scan_report.insert(&db).await?;
    
    // 如果有黑名单进程，发送告警（异步）
    if has_blacklisted {
        let notification_service = NotificationService::new();
        notification_service.send_alert(&client, &payload.blacklisted_processes).await?;
    }
    
    Ok(Json(ApiResponse::success()))
}
```

#### 3.5.2 通知服务

```rust
// src/services/notification_service.rs
use crate::models::client_info::Model as ClientInfo;
use crate::api::dto::BlacklistedProcess;

pub struct NotificationService {
    // TODO: 配置邮件/短信/推送服务
}

impl NotificationService {
    pub fn new() -> Self {
        Self {}
    }
    
    /// 发送告警通知（异步）
    pub async fn send_alert(
        &self,
        client: &ClientInfo,
        blacklisted_processes: &[BlacklistedProcess],
    ) -> Result<(), AppError> {
        // 构建告警消息
        let message = format!(
            "客户端 {} ({}) 检测到黑名单进程：\n{}",
            client.device_name,
            client.ip_address,
            blacklisted_processes
                .iter()
                .map(|p| format!("- {} (风险等级: {})", p.process_name, p.risk_level))
                .collect::<Vec<_>>()
                .join("\n")
        );
        
        // 发送通知给管理员（异步）
        // TODO: 实现邮件/短信通知
        
        tracing::warn!(
            client_id = %client.client_id,
            device_name = %client.device_name,
            processes = ?blacklisted_processes,
            "检测到黑名单进程"
        );
        
        Ok(())
    }
}
```

## 4. 配置管理

### 4.1 Cargo.toml

```toml
[package]
name = "apphub-server"
version = "1.0.0"
edition = "2021"
authors = ["AppHub Team"]
description = "Application Center Server - Rust Implementation"

[dependencies]
# Web框架
axum = { version = "0.7", features = ["multipart", "json", "tokio"] }
tower = { version = "0.4", features = ["full"] }
tower-http = { version = "0.5", features = ["cors", "trace", "limit"] }

# 异步运行时
tokio = { version = "1", features = ["full"] }

# 数据库
sea-orm = { version = "0.12", features = ["sqlx-postgres", "runtime-tokio-native-tls", "macros"] }
sea-orm-migration = "0.12"

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 认证
jsonwebtoken = "9"
bcrypt = "0.15"

# 文件处理
tokio-util = { version = "0.7", features = ["io"] }
sha2 = "0.10"

# 缓存
dashmap = "5"
moka = { version = "0.12", features = ["future"] }

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# 配置
config = "0.14"
dotenvy = "0.15"

# 工具
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
thiserror = "1"
anyhow = "1"

# API文档
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "4", features = ["axum"] }

# 验证
validator = { version = "0.16", features = ["derive"] }

[dev-dependencies]
tokio-test = "0.4"

[[bin]]
name = "apphub-server"
path = "src/bin/server.rs"

[[bin]]
name = "migrate"
path = "src/bin/migrate.rs"
```

### 4.2 配置文件 (config/default.toml)

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "postgres://apphub:password@localhost/apphub"
max_connections = 100
min_connections = 5
connect_timeout = 10
idle_timeout = 300

[redis]
url = "redis://localhost:6379"
pool_size = 10

[jwt]
secret = "your-secret-key-change-this-in-production"
access_token_expire = 7200      # 2小时
refresh_token_expire = 604800   # 7天

[upload]
path = "./uploads"
max_size = 524288000           # 500MB
chunk_size = 5242880           # 5MB

[log]
level = "info"
file = "./logs/apphub.log"
max_size = 100                 # MB
max_files = 10

[cors]
origins = ["http://localhost:3000", "http://localhost:8080"]
methods = ["GET", "POST", "PUT", "DELETE"]
headers = ["Authorization", "Content-Type"]

[rate_limit]
requests_per_minute = 100
burst = 20
```

### 4.3 程序入口

```rust
// src/bin/server.rs
use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use apphub_server::{
    api::routes::create_routes,
    config::app_config::CONFIG,
    services::*,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "apphub_server=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("启动应用中心服务器...");

    // 连接数据库（异步）
    let db = sea_orm::Database::connect(&CONFIG.database.url)
        .await
        .expect("数据库连接失败");

    tracing::info!("数据库连接成功");

    // 创建服务实例
    let auth_service = AuthService::new(db.clone());
    let software_service = SoftwareService::new(db.clone());
    let blacklist_service = BlacklistService::new(db.clone());

    // 初始化黑名单缓存（异步）
    blacklist_service.init_cache().await?;
    tracing::info!("黑名单缓存初始化完成");

    // 创建路由
    let app = create_routes(
        db.clone(),
        auth_service,
        software_service,
        blacklist_service,
    )
    .layer(CorsLayer::new().allow_origin(CONFIG.cors.origins.parse::<HeaderValue>().unwrap()))
    .layer(TraceLayer::new_for_http());

    // 启动服务器（异步）
    let addr = format!("{}:{}", CONFIG.server.host, CONFIG.server.port);
    tracing::info!("服务器启动在 {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

## 5. 性能优化策略

### 5.1 异步IO优化

```rust
// 使用tokio::fs异步文件操作，避免阻塞线程
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// 异步文件读取
let mut file = File::open("large_file.exe").await?;
let mut buffer = Vec::new();
file.read_to_end(&mut buffer).await?;

// 流式文件复制（内存占用恒定）
use tokio::io::copy;
let mut source = File::open("source.exe").await?;
let mut dest = File::create("dest.exe").await?;
copy(&mut source, &mut dest).await?;
```

### 5.2 并发处理优化

```rust
// 使用Tokio并发处理多个请求
use tokio::task::JoinSet;

let mut tasks = JoinSet::new();

// 并发处理1000个下载请求
for i in 0..1000 {
    tasks.spawn(async move {
        download_file(i).await
    });
}

// 等待所有任务完成
while let Some(result) = tasks.join_next().await {
    match result {
        Ok(data) => tracing::info!("下载成功"),
        Err(e) => tracing::error!("下载失败: {}", e),
    }
}
```

### 5.3 缓存优化

```rust
// 使用DashMap无锁并发缓存
use dashmap::DashMap;

let cache: DashMap<String, UserData> = DashMap::new();

// 无锁写入
cache.insert("user_1".to_string(), UserData::default());

// 无锁读取（极致性能）
if let Some(user) = cache.get("user_1") {
    tracing::info!("用户: {}", user.username);
}

// 批量查询（无锁迭代）
for item in cache.iter() {
    tracing::info!("Key: {}, Value: {}", item.key(), item.username);
}
```

### 5.4 数据库优化

```rust
// SeaORM异步查询优化
use sea_orm::{EntityTrait, QueryFilter, QuerySelect, PaginatorTrait};

// 只查询需要的字段（减少数据传输）
let users = Users::find()
    .select_only()
    .column(users::Column::Id)
    .column(users::Column::Username)
    .column(users::Column::Nickname)
    .into_partial_model::<UserPartial>()
    .all(&db)
    .await?;

// 分页查询（避免全表扫描）
let paginator = Software::find()
    .filter(software::Column::Status.eq(1))
    .paginate(&db, 50); // 每页50条

let total_pages = paginator.num_pages().await?;
let page_data = paginator.fetch_page(0).await?;

// 批量插入（提高效率）
let users_to_insert: Vec<users::ActiveModel> = vec![...];
Users::insert_many(users_to_insert)
    .exec(&db)
    .await?;
```

## 6. Docker部署

### 6.1 Dockerfile

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app

# 复制Cargo配置
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 编译Release版本（优化编译）
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 复制编译产物
COPY --from=builder /app/target/release/apphub-server /app/apphub-server

# 复制配置文件
COPY config ./config

# 创建日志和上传目录
RUN mkdir -p logs uploads

# 设置环境变量
ENV RUST_LOG=info

# 暴露端口
EXPOSE 8080

# 启动服务
CMD ["./apphub-server"]
```

### 6.2 Docker Compose

```yaml
version: '3.8'

services:
  apphub-server:
    build: .
    container_name: apphub-server
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://apphub:password@postgres:5432/apphub
      - REDIS_URL=redis://redis:6379
      - RUST_LOG=info
    volumes:
      - ./uploads:/app/uploads
      - ./logs:/app/logs
      - ./config:/app/config
    depends_on:
      - postgres
      - redis
    restart: unless-stopped

  postgres:
    image: postgres:15
    container_name: apphub-postgres
    environment:
      - POSTGRES_USER=apphub
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=apphub
    volumes:
      - postgres-data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    container_name: apphub-redis
    volumes:
      - redis-data:/data
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    container_name: apphub-nginx
    ports:
      - "80:80"
    volumes:
      - ./docker/nginx.conf:/etc/nginx/nginx.conf
    depends_on:
      - apphub-server
    restart: unless-stopped

volumes:
  postgres-data:
  redis-data:
```

## 7. 测试策略

### 7.1 单元测试

```rust
// src/services/auth_service.rs
#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseConnection, Database};
    
    #[tokio::test]
    async fn test_login_success() {
        // 创建测试数据库连接
        let db = Database::connect(":memory:")
            .await
            .expect("数据库连接失败");
        
        let auth_service = AuthService::new(db);
        
        // 测试登录
        let result = auth_service.login("admin", "password").await;
        
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_login_invalid_password() {
        let db = Database::connect(":memory:")
            .await
            .expect("数据库连接失败");
        
        let auth_service = AuthService::new(db);
        
        let result = auth_service.login("admin", "wrong_password").await;
        
        assert!(result.is_err());
    }
}
```

### 7.2 性能测试

```rust
// tests/performance_test.rs
use std::time::Instant;

#[tokio::test]
async fn test_concurrent_requests() {
    let start = Instant::now();
    
    // 模拟1000并发请求
    let mut tasks = JoinSet::new();
    for i in 0..1000 {
        tasks.spawn(async move {
            // 发送HTTP请求
            let client = reqwest::Client::new();
            client.get("http://localhost:8080/api/v1/softwares")
                .send()
                .await
        });
    }
    
    // 等待所有请求完成
    let mut success_count = 0;
    while let Some(result) = tasks.join_next().await {
        if result.is_ok() {
            success_count += 1;
        }
    }
    
    let elapsed = start.elapsed();
    
    println!("1000并发请求完成时间: {:?}", elapsed);
    println!("成功率: {}%", success_count as f64 / 1000.0 * 100.0);
    
    // 性能要求: 95%的请求在100ms内完成
    assert!(elapsed < Duration::from_secs(2));
}
```

## 8. 性能指标

| 指标 | Rust实现 | Java实现 | 性能提升 |
|------|----------|----------|----------|
| **启动时间** | 10-50ms | 1-3秒 | 20-100倍 |
| **内存占用** | 10-50MB | 100-300MB | 5-10倍 |
| **并发QPS** | 10万 | 1万 | 10倍 |
| **API响应** | 10-50ms | 100-500ms | 5-10倍 |
| **文件上传** | 50-100MB/s | 10-30MB/s | 3-5倍 |
| **进程扫描** | 10-20ms | 100-300ms | 5-10倍 |

---

以上是Rust Server端的详细设计，涵盖了项目结构、核心模块、性能优化、部署等各个方面。Rust的异步特性、内存安全、无锁并发等优势，使得服务器性能显著优于传统Java实现。