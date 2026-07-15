//! 响应DTO

use serde::Serialize;

/// 分页响应
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub list: Vec<T>,
    pub total: i64,
    pub page: u64,
    pub page_size: u64,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// 用户信息响应
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
}

/// 软件信息响应
#[derive(Debug, Serialize)]
pub struct SoftwareInfoResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub icon: Option<String>,
    pub category_id: i64,
    pub category_name: String,
    pub status: i16,
    pub download_count: i64,
    pub created_at: String,
}

/// 黑名单信息响应
#[derive(Debug, Serialize)]
pub struct BlacklistInfoResponse {
    pub id: i64,
    pub process_name: String,
    pub description: Option<String>,
    pub risk_level: i16,
    pub status: i16,
    pub version: i64,
}

/// 客户端黑名单响应
#[derive(Debug, Serialize)]
pub struct ClientBlacklistResponse {
    pub version: i64,
    pub list: Vec<BlacklistItemResponse>,
}

/// 黑名单项响应
#[derive(Debug, Serialize)]
pub struct BlacklistItemResponse {
    pub process_name: String,
    pub risk_level: i16,
}

/// 客户端信息响应
#[derive(Debug, Serialize)]
pub struct ClientInfoResponse {
    pub client_id: String,
    pub device_name: String,
    pub os_type: String,
    pub os_version: String,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub status: i16,
    pub last_heartbeat_at: Option<String>,
}

/// 统计信息响应
#[derive(Debug, Serialize)]
pub struct StatisticsResponse {
    pub total_clients: i64,
    pub online_clients: i64,
    pub total_softwares: i64,
    pub total_downloads: i64,
    pub today_scans: i64,
    pub today_alerts: i64,
}
