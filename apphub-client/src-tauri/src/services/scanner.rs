//! 扫描服务 - 定期扫描进程并匹配黑名单

use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;
use tracing::{info, debug, warn, error};

use crate::api::{self, BlacklistedProcessReport, ScanReportRequest};

/// 全局 client_id（每次启动生成一个）
static CLIENT_ID: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    uuid::Uuid::new_v4().to_string()
});

/// 获取当前 client_id
pub fn get_client_id() -> String {
    CLIENT_ID.clone()
}

/// 启动扫描服务
pub async fn start_scan_service(app_handle: AppHandle) {
    info!("启动扫描服务，间隔 60 秒");

    // 首次启动延迟 10 秒，等登录完成
    tokio::time::sleep(Duration::from_secs(10)).await;

    let mut ticker = interval(Duration::from_secs(60)); // 每1分钟扫描一次（调试用）

    loop {
        ticker.tick().await;

        // 检查是否有 token
        let access_token = match api::get_access_token() {
            Some(t) => t,
            None => {
                info!("[扫描] 未登录，跳过本次扫描");
                continue;
            }
        };
        info!("[扫描] 已获取 token，开始扫描流程");

        // 获取用户信息
        let user_info = match api::get_user_info_api(&access_token).await {
            Ok(u) => {
                info!("[扫描] 获取用户信息成功: {}", u.username);
                u
            }
            Err(e) => {
                warn!("[扫描] 获取用户信息失败，跳过扫描: {}", e);
                continue;
            }
        };

        // 从服务端获取黑名单
        info!("[扫描] 正在从服务端获取黑名单...");
        let blacklist_data = match api::get_client_blacklist_api(&access_token).await {
            Ok(data) => {
                info!("[扫描] 获取黑名单成功，共 {} 条", data.list.len());
                data
            }
            Err(e) => {
                warn!("[扫描] 获取黑名单失败，跳过扫描: {}", e);
                continue;
            }
        };

        if blacklist_data.list.is_empty() {
            info!("[扫描] 黑名单为空，跳过扫描");
            continue;
        }

        // 构建黑名单进程名集合（小写匹配）
        let blacklist_map: std::collections::HashMap<String, i16> = blacklist_data
            .list
            .iter()
            .map(|item| (item.process_name.to_lowercase(), item.risk_level))
            .collect();

        // 扫描进程
        info!("开始扫描进程，黑名单数量: {}", blacklist_map.len());
        info!("黑名单列表: {:?}", blacklist_map.keys().collect::<Vec<_>>());
        let scan_result = match scan_processes_with_blacklist(&blacklist_map).await {
            Ok(r) => r,
            Err(e) => {
                error!("扫描进程失败: {}", e);
                continue;
            }
        };

        // 发送事件到前端
        app_handle.emit("scan-complete", &scan_result).ok();

        info!(
            "扫描完成: 总进程数 {}, 命中黑名单数 {}",
            scan_result.total_processes,
            scan_result.blacklisted.len()
        );
        info!("所有进程列表: {:?}", scan_result.all_process_names);
        if !scan_result.blacklisted.is_empty() {
            info!(
                "命中黑名单进程: {:?}",
                scan_result
                    .blacklisted
                    .iter()
                    .map(|p| format!("{}(pid={})", p.name, p.pid))
                    .collect::<Vec<_>>()
            );
        }

        // 如果有命中的黑名单进程，上报给服务端
        if !scan_result.blacklisted.is_empty() {
            info!(
                "发现 {} 个黑名单进程，上报服务端",
                scan_result.blacklisted.len()
            );

            let report = ScanReportRequest {
                client_id: get_client_id(),
                user_id: user_info.id,
                username: user_info.username.clone(),
                scan_time: chrono::Utc::now().to_rfc3339(),
                total_processes: scan_result.total_processes as i32,
                processes: scan_result.all_process_names,
                blacklisted_processes: scan_result
                    .blacklisted
                    .iter()
                    .map(|p| BlacklistedProcessReport {
                        process_name: p.name.clone(),
                        pid: p.pid,
                        risk_level: *blacklist_map.get(&p.name.to_lowercase()).unwrap_or(&1),
                    })
                    .collect(),
            };

            if let Err(e) = api::report_scan_api(&access_token, &report).await {
                error!("上报扫描结果失败: {}", e);
            } else {
                info!("扫描结果上报成功");
            }
        }

    }
}

/// 进程信息
#[derive(serde::Serialize)]
struct MatchedProcess {
    name: String,
    pid: u32,
}

/// 扫描结果
#[derive(serde::Serialize)]
struct ScanMatchResult {
    total_processes: usize,
    all_process_names: Vec<String>,
    blacklisted: Vec<MatchedProcess>,
}

/// 扫描进程并匹配黑名单
async fn scan_processes_with_blacklist(
    blacklist_map: &std::collections::HashMap<String, i16>,
) -> Result<ScanMatchResult, String> {
    use sysinfo::{PidExt, ProcessExt, System, SystemExt};

    let mut system = System::new_all();
    system.refresh_all();

    let mut all_names = Vec::new();
    let mut blacklisted = Vec::new();

    for (_pid, process) in system.processes() {
        let name = process.name().to_string();
        all_names.push(name.clone());

        // 匹配黑名单（进程名小写比较）
        let name_lower = name.to_lowercase();
        if blacklist_map.contains_key(&name_lower) {
            blacklisted.push(MatchedProcess {
                name,
                pid: _pid.as_u32(),
            });
        }
    }

    Ok(ScanMatchResult {
        total_processes: all_names.len(),
        all_process_names: all_names,
        blacklisted,
    })
}
