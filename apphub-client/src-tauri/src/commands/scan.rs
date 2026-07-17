use serde::{Deserialize, Serialize};
use tauri::command;

/// 进程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe_path: Option<String>,
    pub memory: u64,
    pub cpu_usage: f32,
}

/// 扫描结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub scan_time: String,
    pub total_processes: usize,
    pub processes: Vec<String>,
    pub blacklisted_processes: Vec<ProcessInfo>,
}

/// 扫描进程
#[command]
pub async fn scan_processes() -> Result<ScanResult, String> {
    use sysinfo::{PidExt, ProcessExt, System, SystemExt};

    let mut system = System::new_all();
    system.refresh_all();

    let mut processes = Vec::new();

    for (pid, process) in system.processes() {
        let process_info = ProcessInfo {
            pid: pid.as_u32() as u32,
            name: process.name().to_string(),
            exe_path: Some(process.exe().to_string_lossy().to_string()),
            memory: process.memory(),
            cpu_usage: process.cpu_usage(),
        };
        processes.push(process_info);
    }

    // TODO: 对比黑名单
    let blacklisted: Vec<ProcessInfo> = Vec::new();

    Ok(ScanResult {
        scan_time: chrono::Local::now().to_rfc3339(),
        total_processes: processes.len(),
        processes: processes.iter().map(|p| p.name.clone()).collect(),
        blacklisted_processes: blacklisted,
    })
}

/// 获取扫描状态
#[command]
pub async fn get_scan_status() -> Result<bool, String> {
    Ok(true)
}

/// 更新黑名单
#[command]
pub async fn update_blacklist(blacklist: Vec<String>) -> Result<(), String> {
    tracing::info!("更新黑名单，共 {} 条记录", blacklist.len());
    Ok(())
}
