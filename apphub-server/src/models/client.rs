use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 客户端信息实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "clients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    #[sea_orm(unique)]
    pub client_id: String, // UUID

    pub user_id: i64,

    pub device_name: String,

    pub os_type: String, // Windows, macOS, Linux

    pub os_version: String,

    pub mac_address: String,

    pub ip_address: Option<String>,

    pub last_heartbeat_at: Option<DateTime<Utc>>,

    pub status: i16, // 0-离线, 1-在线

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
