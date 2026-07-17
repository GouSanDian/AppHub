use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;
use tracing::{info, debug, error};

/// 启动心跳服务
pub async fn start_heartbeat_service(app_handle: AppHandle) {
    info!("启动心跳服务");

    let mut ticker = interval(Duration::from_secs(60)); // 每分钟心跳一次

    loop {
        ticker.tick().await;

        // TODO: 发送心跳到服务器
        debug!("发送心跳");

        // 发送事件到前端
        app_handle.emit("heartbeat", &()).ok();
    }
}
