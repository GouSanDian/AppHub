use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 软件实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "softwares")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub name: String,

    pub description: Option<String>,

    pub version: String,

    pub file_name: String,

    pub file_size: i64,

    pub file_path: String,

    pub file_hash: Option<String>, // MD5/SHA256

    pub icon: Option<String>,

    pub category_id: i64,

    pub status: i16, // 0-禁用, 1-启用

    pub platform: String, // 逗号分隔，如 "mac,linux,windows"

    pub download_count: i64,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,

    pub created_by: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
