pub mod user;
pub mod role;
pub mod software;
pub mod blacklist;
pub mod client;
pub mod scan_report;
pub mod scan_record;
pub mod category;

use sea_orm::entity::prelude::*;

/// 统一导出所有实体
pub use user::Entity as User;
pub use role::Entity as Role;
pub use software::Entity as Software;
pub use blacklist::Entity as Blacklist;
pub use client::Entity as Client;
pub use scan_report::Entity as ScanReport;
pub use scan_record::Entity as ScanRecord;
pub use category::Entity as Category;
