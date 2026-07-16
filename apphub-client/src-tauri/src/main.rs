// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tracing::info;

mod commands;
mod services;
mod utils;
mod models;
mod api;

use utils::logger::setup_logger;

fn main() {
    // 初始化日志
    setup_logger();

    info!("应用中心客户端启动");

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            // 认证命令
            commands::auth::login,
            commands::auth::sync_token,
            commands::auth::sync_server_url,
            commands::auth::logout,
            commands::auth::get_user_info,
            // 扫描命令
            commands::scan::scan_processes,
            commands::scan::get_scan_status,
            commands::scan::update_blacklist,
            // 下载命令
            commands::download::download_software,
            commands::download::pause_download,
            commands::download::resume_download,
            commands::download::cancel_download,
            commands::download::get_download_progress,
            // 配置命令
            commands::config::get_config,
            commands::config::set_config,
            commands::config::save_config,
            // 系统命令
            commands::system::get_system_info,
            commands::system::check_admin_permission,
            commands::system::request_admin_permission,
            commands::system::set_auto_start,
        ])
        .setup(|app| {
            // 启动后台服务
            let app_handle = app.handle().clone();

            // 启动心跳服务
            tauri::async_runtime::spawn(async move {
                //services::heartbeat::start_heartbeat_service(app_handle).await;
            });

            // 启动扫描服务
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                //services::scanner::start_scan_service(app_handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时出错");
}
