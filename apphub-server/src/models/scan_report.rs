use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 扫描报告实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scan_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub client_id: String,

    pub scan_time: DateTime<Utc>,

    pub total_processes: i32,

    pub blacklisted_count: i32,

    pub process_list: Option<String>, // JSON格式存储进程列表

    pub blacklisted_processes: Option<String>, // JSON格式存储黑名单进程

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
