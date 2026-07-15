//! 分类模型

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 分类实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub name: String,

    pub description: Option<String>,

    pub sort_order: i32,

    pub status: i16, // 0-禁用, 1-启用

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::software::Entity")]
    Software,
}

impl Related<super::software::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Software.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
