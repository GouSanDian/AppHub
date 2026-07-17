use serde::{Deserialize, Serialize};
use std::env;

pub mod database;

pub use database::init_database;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub upload: UploadConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,           // 访问令牌过期秒数
    pub refresh_expiration: i64,   // 刷新令牌过期秒数
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    pub dir: String,
    pub max_size: usize,
}

impl AppConfig {
    /// 加载配置
    pub fn load() -> anyhow::Result<Self> {
        // 加载.env文件
        dotenvy::dotenv().ok();

        let config = Self {
            server: ServerConfig {
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/apphub".to_string()),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
                expiration: env::var("JWT_EXPIRATION")
                    .unwrap_or_else(|_| "7200".to_string()) // 2小时
                    .parse()
                    .unwrap_or(7200),
                refresh_expiration: env::var("JWT_REFRESH_EXPIRATION")
                    .unwrap_or_else(|_| "604800".to_string()) // 7天
                    .parse()
                    .unwrap_or(604800),
            },
            upload: UploadConfig {
                dir: env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string()),
                max_size: env::var("UPLOAD_MAX_SIZE")
                    .unwrap_or_else(|_| "524288000".to_string()) // 500MB
                    .parse()
                    .unwrap_or(524288000),
            },
        };

        Ok(config)
    }
}

/// 应用状态（传递给 Axum handlers）
#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub jwt_refresh_expiration: i64,
    pub upload_dir: std::path::PathBuf,
    pub upload_max_size: usize,
}
