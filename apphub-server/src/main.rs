use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod api;
mod models;
mod services;
mod utils;
mod error;
mod constants;

use config::AppConfig;
use api::middleware::cors::create_cors_layer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    init_tracing();

    info!("应用中心服务端启动");

    // 加载配置
    let config = AppConfig::load()?;
    info!("配置加载成功: {:?}", config);

    // 初始化数据库
    let db = config::init_database(&config.database.url).await?;
    info!("数据库连接成功");

    // 构建应用状态
    let state = config::AppState {
        db: db.clone(),
        jwt_secret: config.jwt.secret.clone(),
        jwt_expiration: config.jwt.expiration,
        jwt_refresh_expiration: config.jwt.refresh_expiration,
        upload_dir: std::path::PathBuf::from(&config.upload.dir),
        upload_max_size: config.upload.max_size,
    };

    // 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api::routes::create_routes(state))
        .layer(create_cors_layer());

    // 启动服务
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("服务启动成功，监听地址: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 初始化日志系统
fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

/// 健康检查接口
async fn health_check() -> &'static str {
    "OK"
}
