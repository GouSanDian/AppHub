use serde::{Deserialize, Serialize};
use tauri::command;

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub device_name: String,
    pub os_type: String,
    pub os_version: String,
    pub mac_address: String,
    pub ip_address: String,
}

/// 获取系统信息
#[command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    use sysinfo::SystemExt;

    let sys = sysinfo::System::new_all();

    Ok(SystemInfo {
        device_name: whoami::hostname(),
        os_type: sys.name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: sys.os_version().unwrap_or_else(|| "Unknown".to_string()),
        mac_address: get_mac_address(),
        ip_address: get_local_ip(),
    })
}

/// 检查管理员权限
#[command]
pub async fn check_admin_permission() -> Result<bool, String> {
    // TODO: 实现权限检查
    Ok(true)
}

/// 请求管理员权限
#[command]
pub async fn request_admin_permission() -> Result<(), String> {
    // TODO: 实现权限提升
    tracing::info!("请求管理员权限");
    Ok(())
}

/// 设置开机自启动
#[command]
pub async fn set_auto_start(enable: bool) -> Result<(), String> {
    tracing::info!("设置开机自启动: {}", enable);
    // TODO: 实现开机自启动
    Ok(())
}

/// 获取MAC地址
fn get_mac_address() -> String {
    use std::net::UdpSocket;

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if let Ok(_) = socket.connect("8.8.8.8:80") {
                if let Ok(addr) = socket.local_addr() {
                    return addr.ip().to_string();
                }
            }
        }
        Err(_) => {}
    }

    "127.0.0.1".to_string()
}

/// 获取本地IP地址
fn get_local_ip() -> String {
    use std::net::UdpSocket;

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if let Ok(_) = socket.connect("8.8.8.8:80") {
                if let Ok(addr) = socket.local_addr() {
                    return addr.ip().to_string();
                }
            }
        }
        Err(_) => {}
    }

    "127.0.0.1".to_string()
}
