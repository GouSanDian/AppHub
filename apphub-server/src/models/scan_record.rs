use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 黑名单扫描记录实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scan_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub client_id: String,

    pub user_id: i64,

    pub username: String,

    pub process_name: String,

    pub risk_level: i16,

    pub scan_time: DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
