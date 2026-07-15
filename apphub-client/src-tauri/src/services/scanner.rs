use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;
use tracing::{info, debug, error};

/// 启动扫描服务
pub async fn start_scan_service(app_handle: AppHandle) {
    info!("启动扫描服务");

    let mut ticker = interval(Duration::from_secs(300)); // 每5分钟扫描一次

    loop {
        ticker.tick().await;

        // TODO: 执行进程扫描
        debug!("执行进程扫描");

        // 发送事件到前端
        app_handle.emit("scan", &()).ok();
    }
}
