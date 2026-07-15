use sea_orm::{Database, DatabaseConnection};
use tracing::info;

/// 初始化数据库连接
pub async fn init_database(database_url: &str) -> anyhow::Result<DatabaseConnection> {
    let db = Database::connect(database_url).await?;
    info!("数据库连接成功");
    Ok(db)
}
