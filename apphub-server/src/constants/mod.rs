/// 常量定义

// 用户状态
pub const USER_STATUS_DISABLED: i16 = 0;
pub const USER_STATUS_ENABLED: i16 = 1;

// 软件状态
pub const SOFTWARE_STATUS_DISABLED: i16 = 0;
pub const SOFTWARE_STATUS_ENABLED: i16 = 1;

// 黑名单状态
pub const BLACKLIST_STATUS_DISABLED: i16 = 0;
pub const BLACKLIST_STATUS_ENABLED: i16 = 1;

// 客户端状态
pub const CLIENT_STATUS_OFFLINE: i16 = 0;
pub const CLIENT_STATUS_ONLINE: i16 = 1;

// 风险等级
pub const RISK_LEVEL_LOW: i16 = 1;
pub const RISK_LEVEL_MEDIUM: i16 = 2;
pub const RISK_LEVEL_HIGH: i16 = 3;

// 默认分页
pub const DEFAULT_PAGE_SIZE: u64 = 10;
pub const MAX_PAGE_SIZE: u64 = 100;
