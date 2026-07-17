//! 请求DTO

use serde::Deserialize;
use validator::Validate;

/// 登录请求
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
}

/// 创建用户请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role_id: i64,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub status: Option<i16>,
}

/// 创建软件请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSoftwareRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub version: String,
    pub category_id: i64,
}

/// 更新软件请求
#[derive(Debug, Deserialize)]
pub struct UpdateSoftwareRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<i16>,
}

/// 创建黑名单请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBlacklistRequest {
    #[validate(length(min = 1, max = 255))]
    pub process_name: String,
    pub description: Option<String>,
    pub risk_level: i16,
}

/// 更新黑名单请求
#[derive(Debug, Deserialize)]
pub struct UpdateBlacklistRequest {
    pub description: Option<String>,
    pub risk_level: Option<i16>,
    pub status: Option<i16>,
}

/// 黑名单列表查询参数
#[derive(Debug, Deserialize)]
pub struct BlacklistQueryParams {
    pub keyword: Option<String>,
    pub risk_level: Option<i16>,
    pub status: Option<i16>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

/// 客户端注册请求
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterClientRequest {
    #[validate(length(min = 1, max = 64))]
    pub client_id: String,
    #[validate(length(min = 1, max = 100))]
    pub device_name: String,
    #[validate(length(min = 1, max = 20))]
    pub os_type: String,
    #[validate(length(min = 1, max = 50))]
    pub os_version: String,
    #[validate(length(min = 1, max = 17))]
    pub mac_address: String,
}

/// 心跳请求
#[derive(Debug, Deserialize, Validate)]
pub struct HeartbeatRequest {
    #[validate(length(min = 1, max = 64))]
    pub client_id: String,
    pub ip_address: Option<String>,
}

/// 扫描报告请求
#[derive(Debug, Deserialize)]
pub struct ScanReportRequest {
    pub client_id: String,
    pub scan_time: String,
    pub total_processes: i32,
    pub processes: Vec<String>,
    pub blacklisted_processes: Vec<BlacklistedProcess>,
}

/// 黑名单进程
#[derive(Debug, Deserialize)]
pub struct BlacklistedProcess {
    pub process_name: String,
    pub pid: u32,
    pub risk_level: i16,
}
