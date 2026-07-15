use serde::{Deserialize, Serialize};
use tauri::command;
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;
use once_cell::sync::Lazy;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_url: String,
    pub download_path: String,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub scan_enabled: bool,
    pub scan_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:8080/api/v1".to_string(),
            download_path: dirs::download_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "/downloads".to_string()),
            auto_start: false,
            minimize_to_tray: true,
            scan_enabled: true,
            scan_interval: 300,
        }
    }
}

/// 配置文件路径
fn config_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("apphub");
    let _ = fs::create_dir_all(&dir);
    dir.join("config.json")
}

/// 从文件加载配置
fn load_config() -> AppConfig {
    let path = config_path();
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str(&data) {
                return cfg;
            }
        }
    }
    // 首次启动，写入默认配置
    let cfg = AppConfig::default();
    let _ = save_config_to_disk(&cfg);
    cfg
}

/// 将配置写入磁盘
fn save_config_to_disk(config: &AppConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_path(), json).map_err(|e| e.to_string())?;
    Ok(())
}

// 全局配置
static CONFIG: Lazy<Mutex<AppConfig>> = Lazy::new(|| Mutex::new(load_config()));

/// 获取配置
#[command]
pub async fn get_config() -> Result<AppConfig, String> {
    let config = CONFIG.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

/// 设置配置
#[command]
pub async fn set_config(key: String, value: serde_json::Value) -> Result<(), String> {
    let mut config = CONFIG.lock().map_err(|e| e.to_string())?;

    match key.as_str() {
        "server_url" => {
            if let Some(v) = value.as_str() {
                config.server_url = v.to_string();
            }
        }
        "download_path" => {
            if let Some(v) = value.as_str() {
                config.download_path = v.to_string();
            }
        }
        "auto_start" => {
            if let Some(v) = value.as_bool() {
                config.auto_start = v;
            }
        }
        "minimize_to_tray" => {
            if let Some(v) = value.as_bool() {
                config.minimize_to_tray = v;
            }
        }
        "scan_enabled" => {
            if let Some(v) = value.as_bool() {
                config.scan_enabled = v;
            }
        }
        "scan_interval" => {
            if let Some(v) = value.as_u64() {
                config.scan_interval = v;
            }
        }
        _ => {}
    }

    // 自动持久化
    save_config_to_disk(&config)?;
    Ok(())
}

/// 保存配置
#[command]
pub async fn save_config() -> Result<(), String> {
    let config = CONFIG.lock().map_err(|e| e.to_string())?;
    save_config_to_disk(&config)?;
    tracing::info!("配置已保存到 {:?}", config_path());
    Ok(())
}
