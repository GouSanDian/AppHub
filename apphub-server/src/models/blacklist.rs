use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 黑名单实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "blacklists")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub process_name: String, // 进程名，如 "game.exe"

    pub description: Option<String>,

    pub risk_level: i16, // 1-低, 2-中, 3-高

    pub status: i16, // 0-禁用, 1-启用

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,

    pub created_by: i64,

    pub version: i64, // 版本号，用于增量同步
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
