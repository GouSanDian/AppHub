# Client端详细设计文档 (Tauri版本)

## 1. 项目概述

### 1.1 项目简介
apphub-client 是应用中心系统的桌面客户端，运行在用户操作系统上，提供软件下载、进程监控、黑名单检测等功能。客户端通过HTTPS协议与Server端通信，定期上报系统状态。

### 1.2 技术选型

| 技术领域 | 技术选型 | 版本 | 说明 |
|----------|----------|------|------|
| 开发框架 | Tauri | 2.0+ | Rust + WebView 桌面应用框架 |
| 后端语言 | Rust | 1.70+ | 系统级编程语言，内存安全 |
| 前端框架 | Vue 3 | 3.3+ | 现代化前端框架 |
| 状态管理 | Pinia | 2.1+ | Vue官方推荐状态管理 |
| HTTP客户端 | reqwest | 0.11+ | Rust异步HTTP客户端 |
| 进程监控 | sysinfo | 0.29+ | 跨平台系统信息库 |
| 构建工具 | Cargo | - | Rust包管理器 |
| 前端构建 | Vite | 5.0+ | 快速前端构建工具 |
| UI组件库 | Element Plus | 2.4+ | Vue 3组件库 |

### 1.3 支持平台
- Windows 10/11
- Windows Server 2016+
- macOS 10.15+
- Linux (Ubuntu 20.04+, CentOS 8+)

### 1.4 Tauri技术优势

| 特性 | Tauri方案 | 传统方案(PyQt) | 优势 |
|------|-----------|----------------|------|
| 安装包体积 | 5-10MB | 50-100MB | 体积小10倍 |
| 内存占用 | 50-100MB | 200-500MB | 内存占用低 |
| 启动速度 | <1秒 | 2-5秒 | 启动快 |
| 安全性 | 多层沙箱隔离 | 依赖系统权限 | 更安全 |
| 跨平台 | 一套代码 | 需适配不同系统 | 开发效率高 |
| 前端生态 | 完整Web生态 | Qt专有组件 | 生态丰富 |

## 2. 项目结构

### 2.1 目录结构

```
apphub-client/
├── src-tauri/                         # Rust后端代码
│   ├── Cargo.toml                     # Rust依赖配置
│   ├── tauri.conf.json                # Tauri配置文件
│   ├── capabilities/                  # 权限配置
│   │   ├── default.json               # 默认权限
│   │   └── admin.json                 # 管理员权限（进程监控）
│   ├── icons/                         # 应用图标
│   │   ├── icon.ico                   # Windows图标
│   │   ├── icon.icns                  # macOS图标
│   │   └── icon.png                   # Linux图标
│   └── src/
│       ├── main.rs                    # 主入口
│       ├── lib.rs                     # 库文件
│       │
│       ├── commands/                  # Tauri Commands
│       │   ├── mod.rs
│       │   ├── scan.rs                # 进程扫描命令
│       │   ├── download.rs            # 下载命令
│       │   ├── config.rs              # 配置命令
│       │   ├── system.rs              # 系统命令
│       │   └── auth.rs                # 认证命令
│       │
│       ├── services/                  # 服务层
│       │   ├── mod.rs
│       │   ├── scanner.rs             # 进程扫描器（sysinfo）
│       │   ├── reporter.rs            # 上报服务
│       │   ├── downloader.rs          # 下载器（异步）
│       │   ├── heartbeat.rs           # 心跳服务
│       │   └── blacklist_sync.rs      # 黑名单同步服务
│       │
│       ├── utils/                     # 工具模块
│       │   ├── mod.rs
│       │   ├── permissions.rs         # 权限检查
│       │   ├── config.rs              # 配置管理
│       │   ├── logger.rs              # 日志工具
│       │   └── system.rs              # 系统工具
│       │
│       ├── models/                    # 数据模型
│       │   ├── mod.rs
│       │   ├── process.rs             # 进程模型
│       │   ├── scan_result.rs         # 扫描结果
│       │   ├── software.rs            # 软件模型
│       │   └── config.rs              # 配置模型
│       │
│       ├── api/                       # API客户端
│       │   ├── mod.rs
│       │   ├── client.rs              # HTTP客户端（reqwest）
│       │   └── endpoints.rs           # API端点
│       │
│       └── error.rs                   # 错误处理
│
├── src/                               # Vue前端代码
│   ├── main.ts                        # Vue入口
│   ├── App.vue                        # 根组件
│   ├── env.d.ts                       # 环境类型声明
│   │
│   ├── components/                    # 组件
│   │   ├── common/                    # 通用组件
│   │   │   ├── AppHeader.vue          # 应用头部
│   │   │   ├── AppSidebar.vue         # 侧边栏
│   │   │   └── AppFooter.vue          # 应用底部
│   │   ├── auth/
│   │   │   └── LoginForm.vue          # 登录表单
│   │   ├── software/
│   │   │   ├── SoftwareList.vue       # 软件列表
│   │   │   ├── SoftwareCard.vue       # 软件卡片
│   │   │   └── SoftwareDetail.vue     # 软件详情
│   │   ├── download/
│   │   │   ├── DownloadList.vue       # 下载列表
│   │   │   └── DownloadProgress.vue   # 下载进度
│   │   └── settings/
│   │       ├── SettingsForm.vue       # 设置表单
│   │       └── PermissionPrompt.vue   # 权限提示
│   │
│   ├── views/                         # 页面视图
│   │   ├── LoginView.vue              # 登录页
│   │   ├── HomeView.vue               # 首页
│   │   ├── SoftwareView.vue           # 软件中心
│   │   ├── DownloadView.vue           # 下载管理
│   │   └── SettingsView.vue           # 设置页面
│   │
│   ├── stores/                        # Pinia状态管理
│   │   ├── index.ts                   # Store入口
│   │   ├── auth.ts                    # 认证状态
│   │   ├── software.ts                # 软件状态
│   │   ├── download.ts                # 下载状态
│   │   ├── scan.ts                    # 扫描状态
│   │   └── config.ts                  # 配置状态
│   │
│   ├── api/                           # 前端API接口
│   │   ├── index.ts                   # API入口
│   │   ├── auth.ts                    # 认证API
│   │   ├── software.ts                # 软件API
│   │   ├── blacklist.ts               # 黑名单API
│   │   └── report.ts                  # 上报API
│   │
│   ├── utils/                         # 工具函数
│   │   ├── http.ts                    # HTTP工具
│   │   ├── storage.ts                 # 本地存储
│   │   ├── notification.ts            # 通知工具
│   │   ├── date.ts                    # 日期工具
│   │   └── tauri.ts                   # Tauri IPC封装
│   │
│   ├── router/                        # Vue Router配置
│   │   └── index.ts                   # 路由定义
│   │
│   ├── styles/                        # 样式文件
│   │   ├── main.css                   # 主样式
│   │   ├── variables.css              # CSS变量
│   │   └── themes/                    # 主题样式
│   │
│   ├── types/                         # TypeScript类型定义
│   │   ├── user.d.ts                  # 用户类型
│   │   ├── software.d.ts              # 软件类型
│   │   ├── scan.d.ts                  # 扫描类型
│   │   └── api.d.ts                   # API类型
│   │
│   └── assets/                        # 资源文件
│       ├── images/                    # 图片资源
│       └── icons/                     # 图标资源
│
├── public/                            # 公共资源
│   └── favicon.ico
│
├── dist/                              # 前端构建输出
├── src-tauri/target/                  # Rust构建输出
│
├── package.json                       # NPM配置
├── vite.config.ts                     # Vite配置
├── tsconfig.json                      # TypeScript配置
├── .env                               # 环境变量
├── .env.example                       # 环境变量示例
├── README.md                          # 项目说明
└── tauri.info.md                      # Tauri配置说明
```

### 2.2 架构设计

```
┌─────────────────────────────────────────────────────────────────┐
│                        前端层 (Vue 3)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │   页面视图    │  │   UI组件    │  │   Pinia状态 │            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             │ IPC通信 (invoke/listen)
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Tauri Runtime层                             │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Tauri Commands (Rust)                        │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Rust后端层                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   扫描服务   │  │   下载服务   │  │   心跳服务   │          │
│  │  (sysinfo)   │  │  (reqwest)   │  │  (tokio)     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   API客户端  │  │   配置管理   │  │   日志服务   │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Server端 (HTTPS)                               │
└─────────────────────────────────────────────────────────────────┘
```

## 3. Tauri配置

### 3.1 tauri.conf.json

```json
{
  "productName": "应用中心客户端",
  "version": "1.0.0",
  "identifier": "com.example.apphub-client",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "title": "应用中心客户端",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "center": true,
        "resizable": true,
        "fullscreen": false,
        "transparent": false,
        "decorations": true
      }
    ],
    "security": {
      "csp": null,
      "capabilities": ["default", "admin"]
    },
    "withGlobalTauri": true
  },
  "bundle": {
    "active": true,
    "targets": ["msi", "nsis", "dmg", "app", "deb", "rpm"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "resources": [],
    "copyright": "Copyright © 2024",
    "category": "DeveloperTool",
    "shortDescription": "企业软件管理平台客户端",
    "longDescription": "应用中心客户端提供软件下载、进程监控和安全管控功能"
  },
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": ["https://apphub.example.com/api/v1/client/update"],
      "dialog": true,
      "pubkey": "your-public-key-here"
    },
    "notification": {
      "active": true
    }
  }
}
```

### 3.2 权限配置 (capabilities/default.json)

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "默认权限配置",
  "local": true,
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:default",
    "core:window:allow-close",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "core:window:allow-hide",
    "core:window:allow-show",
    "core:window:allow-center",
    "core:window:allow-set-size",
    "core:window:allow-set-focus",
    "core:app:default",
    "core:app:allow-version",
    "core:app:allow-name",
    "core:app:allow-tauri-version",
    "core:event:default",
    "core:event:allow-listen",
    "core:event:allow-emit",
    "core:notification:default",
    "core:notification:allow-notify",
    "core:path:default",
    "core:path:allow-resolve-directory",
    "core:fs:default",
    "core:fs:allow-read-file",
    "core:fs:allow-write-file",
    "core:fs:allow-read-dir",
    "core:fs:allow-copy-file",
    "core:fs:allow-remove",
    "core:fs:allow-create-dir",
    "core:os:default",
    "core:os:allow-platform",
    "core:os:allow-version",
    "core:os:allow-kind",
    "core:os:allow-arch",
    "core:process:default",
    "core:process:allow-restart",
    "core:process:allow-exit",
    "http:default",
    "http:allow-fetch",
    "http:allow-fetch-send",
    "dialog:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "dialog:allow-ask",
    "dialog:allow-message",
    "shell:default",
    "shell:allow-open"
  ]
}
```

### 3.3 管理员权限配置 (capabilities/admin.json)

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "admin",
  "description": "管理员权限配置（进程监控需要）",
  "local": true,
  "windows": ["main"],
  "permissions": [
    "core:default",
    "process:allow-spawn",
    "process:allow-execute",
    "fs:allow-read-all",
    "fs:allow-write-all"
  ]
}
```

## 4. Rust后端核心模块

### 4.1 主入口 (src-tauri/src/main.rs)

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tracing::info;

mod commands;
mod services;
mod utils;
mod models;
mod api;
mod error;

use utils::logger::setup_logger;
use utils::config::init_config;

fn main() {
    // 初始化日志
    setup_logger();
    
    // 初始化配置
    init_config();
    
    info!("应用中心客户端启动");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            // 认证命令
            commands::auth::login,
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
                services::heartbeat::start_heartbeat_service(app_handle).await;
            });
            
            // 启动扫描服务
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                services::scanner::start_scan_service(app_handle).await;
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时出错");
}
```

### 4.2 进程扫描模块 (src-tauri/src/services/scanner.rs)

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt, get_current_pid};
use tauri::AppHandle;
use tokio::time::interval;
use tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};

use crate::models::process::ProcessInfo;
use crate::models::scan_result::ScanResult;
use crate::services::reporter::report_scan_result;
use crate::utils::config::get_config;

/// 黑名单缓存
static BLACKLIST: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub interval: u64,  // 扫描间隔（秒）
    pub enabled: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            interval: 300,  // 默认5分钟
            enabled: true,
        }
    }
}

/// 更新黑名单
pub fn update_blacklist(blacklist: Vec<String>) {
    let mut list = BLACKLIST.lock().unwrap();
    *list = blacklist;
    info!("黑名单已更新，共 {} 条记录", list.len());
}

/// 获取所有进程
pub fn get_all_processes() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_all();
    
    let mut processes = Vec::new();
    
    for (pid, process) in system.processes() {
        let process_info = ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            exe_path: process.exe().map(|p| p.to_string_lossy().to_string()),
            memory: process.memory(),
            cpu_usage: process.cpu_usage(),
        };
        processes.push(process_info);
    }
    
    processes
}

/// 执行扫描
pub fn scan_processes() -> ScanResult {
    debug!("开始进程扫描...");
    let start_time = std::time::Instant::now();
    
    // 获取所有进程
    let processes = get_all_processes();
    let process_names: Vec<String> = processes.iter()
        .map(|p| p.name.clone())
        .collect();
    
    // 获取黑名单
    let blacklist = BLACKLIST.lock().unwrap();
    
    // 检测黑名单进程
    let mut blacklisted = Vec::new();
    for process in &processes {
        if blacklist.contains(&process.name) {
            blacklisted.push(process.clone());
            warn!("发现黑名单进程: {} (PID: {})", process.name, process.pid);
        }
    }
    
    let elapsed = start_time.elapsed();
    info!(
        "扫描完成，共 {} 个进程，发现 {} 个黑名单进程，耗时 {:?}",
        processes.len(),
        blacklisted.len(),
        elapsed
    );
    
    ScanResult {
        scan_time: chrono::Local::now().to_rfc3339(),
        total_processes: processes.len(),
        processes: process_names,
        blacklisted_processes: blacklisted,
    }
}

/// 启动扫描服务
pub async fn start_scan_service(app_handle: AppHandle) {
    info!("启动扫描服务");
    
    let config = get_config().scan;
    if !config.enabled {
        info!("扫描服务已禁用");
        return;
    }
    
    let mut ticker = interval(Duration::from_secs(config.interval));
    
    loop {
        ticker.tick().await;
        
        // 执行扫描
        let result = scan_processes();
        
        // 上报结果
        if let Err(e) = report_scan_result(&result).await {
            error!("扫描结果上报失败: {}", e);
        }
        
        // 发送事件到前端
        app_handle.emit_all("scan-completed", &result).ok();
    }
}
```

### 4.3 进程扫描命令 (src-tauri/src/commands/scan.rs)

```rust
use tauri::command;
use crate::services::scanner::{scan_processes, update_blacklist};
use crate::models::scan_result::ScanResult;

/// 扫描进程命令
#[command]
pub fn scan_processes_cmd() -> Result<ScanResult, String> {
    Ok(scan_processes())
}

/// 获取扫描状态
#[command]
pub fn get_scan_status() -> Result<bool, String> {
    // 返回扫描服务是否运行
    Ok(true)
}

/// 更新黑名单
#[command]
pub fn update_blacklist_cmd(blacklist: Vec<String>) -> Result<(), String> {
    update_blacklist(blacklist);
    Ok(())
}
```

### 4.4 下载服务 (src-tauri/src/services/downloader.rs)

```rust
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use tracing::{info, error};

/// 下载任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// 下载任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub task_id: String,
    pub software_id: i64,
    pub software_name: String,
    pub file_name: String,
    pub file_size: u64,
    pub save_path: String,
    pub status: DownloadStatus,
    pub progress: f64,
    pub downloaded: u64,
    pub speed: f64,
    pub error_message: Option<String>,
}

/// 下载管理器
pub struct DownloadManager {
    client: Client,
    tasks: Arc<Mutex<Vec<DownloadTask>>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// 添加下载任务
    pub fn add_task(&self, task: DownloadTask) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }
    
    /// 执行下载
    pub async fn download(&self, task_id: &str, url: &str, token: &str) -> Result<(), String> {
        // 获取任务
        let task = {
            let mut tasks = self.tasks.lock().unwrap();
            let task = tasks.iter_mut()
                .find(|t| t.task_id == task_id)
                .map(|t| {
                    t.status = DownloadStatus::Downloading;
                    t.clone()
                });
            task
        };
        
        let task = match task {
            Some(t) => t,
            None => return Err("任务不存在".to_string()),
        };
        
        // 发送HTTP请求
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("服务器返回错误: {}", response.status()));
        }
        
        // 获取文件大小
        let total_size = response.content_length().unwrap_or(0);
        
        // 创建文件
        let mut file = File::create(&task.save_path)
            .map_err(|e| format!("创建文件失败: {}", e))?;
        
        // 下载内容
        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("下载失败: {}", e))?;
            file.write_all(&chunk)
                .map_err(|e| format!("写入文件失败: {}", e))?;
            
            downloaded += chunk.len() as u64;
            
            // 更新进度
            if total_size > 0 {
                let progress = downloaded as f64 / total_size as f64;
                self.update_progress(task_id, progress, downloaded);
            }
        }
        
        // 更新状态为完成
        self.update_status(task_id, DownloadStatus::Completed);
        
        info!("下载完成: {}", task.software_name);
        Ok(())
    }
    
    /// 更新进度
    fn update_progress(&self, task_id: &str, progress: f64, downloaded: u64) {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.progress = progress;
            task.downloaded = downloaded;
        }
    }
    
    /// 更新状态
    fn update_status(&self, task_id: &str, status: DownloadStatus) {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.status = status;
        }
    }
    
    /// 获取任务
    pub fn get_task(&self, task_id: &str) -> Option<DownloadTask> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().find(|t| t.task_id == task_id).cloned()
    }
    
    /// 获取所有任务
    pub fn get_all_tasks(&self) -> Vec<DownloadTask> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }
}
```

### 4.5 API客户端 (src-tauri/src/api/client.rs)

```rust
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tracing::{info, error};

use crate::error::AppError;

/// API响应结构
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

/// API客户端
pub struct ApiClient {
    client: Client,
    base_url: String,
}

// 全局Token存储
static ACCESS_TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static REFRESH_TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

impl ApiClient {
    /// 创建新的API客户端
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("创建HTTP客户端失败");
        
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }
    
    /// 设置Token
    pub fn set_token(access: String, refresh: Option<String>) {
        let mut access_token = ACCESS_TOKEN.lock().unwrap();
        *access_token = Some(access);
        
        if let Some(refresh) = refresh {
            let mut refresh_token = REFRESH_TOKEN.lock().unwrap();
            *refresh_token = Some(refresh);
        }
    }
    
    /// 清除Token
    pub fn clear_token() {
        let mut access_token = ACCESS_TOKEN.lock().unwrap();
        *access_token = None;
        let mut refresh_token = REFRESH_TOKEN.lock().unwrap();
        *refresh_token = None;
    }
    
    /// 发送GET请求
    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<T, AppError> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.get(&url);
        
        // 添加认证头
        let token = ACCESS_TOKEN.lock().unwrap();
        if let Some(token) = token.as_ref() {
            request = request.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }
        
        let response = request.send().await.map_err(|e| {
            AppError::NetworkError(format!("请求失败: {}", e))
        })?;
        
        self.handle_response(response).await
    }
    
    /// 发送POST请求
    pub async fn post<T: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T, AppError> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.post(&url).json(body);
        
        // 添加认证头
        let token = ACCESS_TOKEN.lock().unwrap();
        if let Some(token) = token.as_ref() {
            request = request.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }
        
        let response = request.send().await.map_err(|e| {
            AppError::NetworkError(format!("请求失败: {}", e))
        })?;
        
        self.handle_response(response).await
    }
    
    /// 处理响应
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, AppError> {
        let status = response.status();
        
        if status == 401 {
            return Err(AppError::AuthenticationError("认证已过期".to_string()));
        }
        
        if status == 403 {
            return Err(AppError::AuthenticationError("权限不足".to_string()));
        }
        
        if status.is_server_error() {
            return Err(AppError::ServerError(format!("服务器错误: {}", status)));
        }
        
        let api_response: ApiResponse<T> = response.json().await.map_err(|e| {
            AppError::ServerError(format!("解析响应失败: {}", e))
        })?;
        
        if api_response.code != 200 {
            return Err(AppError::ApiError {
                code: api_response.code,
                message: api_response.message,
            });
        }
        
        api_response.data.ok_or_else(|| {
            AppError::ApiError {
                code: 500,
                message: "响应数据为空".to_string(),
            }
        })
    }
}

// 全局API客户端实例
static API_CLIENT: Lazy<ApiClient> = Lazy::new(|| {
    ApiClient::new("https://apphub.example.com/api/v1")
});

/// 获取API客户端
pub fn get_api_client() -> &'static ApiClient {
    &API_CLIENT
}
```

### 4.6 配置管理 (src-tauri/src/utils/config.rs)

```rust
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing::{info, error};

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub scan: ScanConfig,
    pub heartbeat: HeartbeatConfig,
    pub download: DownloadConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub base_url: String,
    pub timeout: u64,
    pub retry_times: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub interval: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatConfig {
    pub interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub save_path: Option<String>,
    pub concurrent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub minimize_to_tray: bool,
    pub auto_start: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                base_url: "https://apphub.example.com/api/v1".to_string(),
                timeout: 30,
                retry_times: 3,
            },
            scan: ScanConfig {
                interval: 300,
                enabled: true,
            },
            heartbeat: HeartbeatConfig {
                interval: 60,
            },
            download: DownloadConfig {
                save_path: None,
                concurrent: 3,
            },
            ui: UiConfig {
                minimize_to_tray: true,
                auto_start: false,
            },
        }
    }
}

/// 配置目录
fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("无法获取用户目录");
    home.join(".apphub")
}

/// 配置文件路径
fn get_config_file() -> PathBuf {
    get_config_dir().join("config.json")
}

/// 全局配置
static CONFIG: Lazy<Mutex<AppConfig>> = Lazy::new(|| {
    Mutex::new(load_config())
});

/// 加载配置
fn load_config() -> AppConfig {
    let config_file = get_config_file();
    
    if config_file.exists() {
        match fs::read_to_string(&config_file) {
            Ok(content) => {
                match serde_json::from_str::<AppConfig>(&content) {
                    Ok(config) => {
                        info!("配置加载成功");
                        return config;
                    }
                    Err(e) => {
                        error!("解析配置文件失败: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("读取配置文件失败: {}", e);
            }
        }
    }
    
    // 返回默认配置
    AppConfig::default()
}

/// 保存配置
pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let config_dir = get_config_dir();
    let config_file = get_config_file();
    
    // 确保目录存在
    fs::create_dir_all(&config_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;
    
    // 序列化配置
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    
    // 写入文件
    fs::write(&config_file, content).map_err(|e| format!("写入配置文件失败: {}", e))?;
    
    // 更新全局配置
    let mut global_config = CONFIG.lock().unwrap();
    *global_config = config.clone();
    
    info!("配置保存成功");
    Ok(())
}

/// 获取配置
pub fn get_config() -> AppConfig {
    CONFIG.lock().unwrap().clone()
}

/// 初始化配置
pub fn init_config() {
    // 确保配置目录存在
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("创建配置目录失败");
    }
    
    // 加载配置
    let _ = get_config();
}
```

### 4.7 错误处理 (src-tauri/src/error.rs)

```rust
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    #[error("API错误: [{code}] {message}")]
    ApiError { code: i32, message: String },
    
    #[error("认证错误: {0}")]
    AuthenticationError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
    
    #[error("服务器错误: {0}")]
    ServerError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("IO错误: {0}")]
    IoError(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ConfigError(err.to_string())
    }
}
```

## 5. Vue前端核心模块

### 5.1 主入口 (src/main.ts)

```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'

import App from './App.vue'
import router from './router'
import './styles/main.css'

const app = createApp(App)

// 注册Element Plus图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

app.use(createPinia())
app.use(router)
app.use(ElementPlus)

app.mount('#app')
```

### 5.2 Tauri IPC封装 (src/utils/tauri.ts)

```typescript
import { invoke } from '@tauri-apps/api/core'
import { listen, Event } from '@tauri-apps/api/event'

// 调用Rust命令
export async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args)
  } catch (error) {
    console.error(`调用命令 ${command} 失败:`, error)
    throw error
  }
}

// 监听Rust事件
export async function listenEvent<T>(event: string, handler: (event: Event<T>) => void) {
  return await listen<T>(event, handler)
}

// 认证相关命令
export const authCommands = {
  login: (username: string, password: string) => 
    invokeCommand<{ token: string; user: UserInfo }>('login', { username, password }),
  logout: () => invokeCommand<void>('logout'),
  getUserInfo: () => invokeCommand<UserInfo>('get_user_info'),
}

// 扫描相关命令
export const scanCommands = {
  scanProcesses: () => invokeCommand<ScanResult>('scan_processes'),
  getScanStatus: () => invokeCommand<boolean>('get_scan_status'),
  updateBlacklist: (blacklist: string[]) => 
    invokeCommand<void>('update_blacklist', { blacklist }),
}

// 下载相关命令
export const downloadCommands = {
  downloadSoftware: (softwareId: number, savePath: string) => 
    invokeCommand<string>('download_software', { softwareId, savePath }),
  pauseDownload: (taskId: string) => invokeCommand<void>('pause_download', { taskId }),
  resumeDownload: (taskId: string) => invokeCommand<void>('resume_download', { taskId }),
  cancelDownload: (taskId: string) => invokeCommand<void>('cancel_download', { taskId }),
  getDownloadProgress: (taskId: string) => 
    invokeCommand<DownloadProgress>('get_download_progress', { taskId }),
}

// 配置相关命令
export const configCommands = {
  getConfig: () => invokeCommand<AppConfig>('get_config'),
  setConfig: (key: string, value: unknown) => 
    invokeCommand<void>('set_config', { key, value }),
  saveConfig: () => invokeCommand<void>('save_config'),
}

// 系统相关命令
export const systemCommands = {
  getSystemInfo: () => invokeCommand<SystemInfo>('get_system_info'),
  checkAdminPermission: () => invokeCommand<boolean>('check_admin_permission'),
  requestAdminPermission: () => invokeCommand<void>('request_admin_permission'),
  setAutoStart: (enable: boolean) => invokeCommand<void>('set_auto_start', { enable }),
}
```

### 5.3 Pinia状态管理

#### 认证状态 (src/stores/auth.ts)

```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authCommands } from '@/utils/tauri'

export const useAuthStore = defineStore('auth', () => {
  // State
  const token = ref<string | null>(null)
  const userInfo = ref<UserInfo | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const isLoggedIn = computed(() => !!token.value)
  const username = computed(() => userInfo.value?.username || '')

  // Actions
  async function login(username: string, password: string) {
    loading.value = true
    error.value = null
    
    try {
      const result = await authCommands.login(username, password)
      token.value = result.token
      userInfo.value = result.user
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : '登录失败'
      return false
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    try {
      await authCommands.logout()
    } finally {
      token.value = null
      userInfo.value = null
    }
  }

  async function fetchUserInfo() {
    try {
      const info = await authCommands.getUserInfo()
      userInfo.value = info
    } catch (e) {
      console.error('获取用户信息失败:', e)
    }
  }

  return {
    token,
    userInfo,
    loading,
    error,
    isLoggedIn,
    username,
    login,
    logout,
    fetchUserInfo,
  }
})
```

#### 扫描状态 (src/stores/scan.ts)

```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { scanCommands, listenEvent } from '@/utils/tauri'
import { useNotification } from '@/utils/notification'

export const useScanStore = defineStore('scan', () => {
  const notification = useNotification()
  
  // State
  const isScanning = ref(false)
  const lastScanResult = ref<ScanResult | null>(null)
  const blacklist = ref<string[]>([])
  const scanInterval = ref(300) // 默认5分钟

  // Getters
  const blacklistedCount = computed(() => 
    lastScanResult.value?.blacklisted_processes.length || 0
  )

  // Actions
  async function scanProcesses() {
    isScanning.value = true
    try {
      const result = await scanCommands.scanProcesses()
      lastScanResult.value = result
      
      // 如果发现黑名单进程，发送通知
      if (result.blacklisted_processes.length > 0) {
        notification.warning(
          `发现 ${result.blacklisted_processes.length} 个黑名单进程`,
          '安全警告'
        )
      }
      
      return result
    } finally {
      isScanning.value = false
    }
  }

  function updateBlacklist(newBlacklist: string[]) {
    blacklist.value = newBlacklist
    scanCommands.updateBlacklist(newBlacklist)
  }

  // 监听Rust扫描完成事件
  function setupEventListeners() {
    listenEvent<ScanResult>('scan-completed', (event) => {
      lastScanResult.value = event.payload
      
      if (event.payload.blacklisted_processes.length > 0) {
        notification.warning(
          `发现 ${event.payload.blacklisted_processes.length} 个黑名单进程`,
          '安全警告'
        )
      }
    })
  }

  return {
    isScanning,
    lastScanResult,
    blacklist,
    scanInterval,
    blacklistedCount,
    scanProcesses,
    updateBlacklist,
    setupEventListeners,
  }
})
```

#### 下载状态 (src/stores/download.ts)

```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { downloadCommands, listenEvent } from '@/utils/tauri'
import { useNotification } from '@/utils/notification'

export const useDownloadStore = defineStore('download', () => {
  const notification = useNotification()
  
  // State
  const tasks = ref<DownloadTask[]>([])
  const activeDownloads = ref(0)

  // Getters
  const downloadingTasks = computed(() => 
    tasks.value.filter(t => t.status === 'downloading')
  )
  
  const completedTasks = computed(() => 
    tasks.value.filter(t => t.status === 'completed')
  )
  
  const totalProgress = computed(() => {
    if (tasks.value.length === 0) return 0
    const total = tasks.value.reduce((sum, t) => sum + t.progress, 0)
    return total / tasks.value.length
  })

  // Actions
  async function startDownload(software: Software, savePath: string) {
    try {
      const taskId = await downloadCommands.downloadSoftware(software.id, savePath)
      
      const task: DownloadTask = {
        taskId,
        softwareId: software.id,
        softwareName: software.name,
        fileName: software.fileName,
        fileSize: software.fileSize,
        savePath,
        status: 'downloading',
        progress: 0,
        downloaded: 0,
        speed: 0,
      }
      
      tasks.value.push(task)
      activeDownloads.value++
      
      return taskId
    } catch (e) {
      notification.error(`下载启动失败: ${e}`)
      throw e
    }
  }

  async function pauseDownload(taskId: string) {
    await downloadCommands.pauseDownload(taskId)
    updateTaskStatus(taskId, 'paused')
  }

  async function resumeDownload(taskId: string) {
    await downloadCommands.resumeDownload(taskId)
    updateTaskStatus(taskId, 'downloading')
  }

  async function cancelDownload(taskId: string) {
    await downloadCommands.cancelDownload(taskId)
    updateTaskStatus(taskId, 'cancelled')
    activeDownloads.value--
  }

  function updateTaskStatus(taskId: string, status: DownloadStatus) {
    const task = tasks.value.find(t => t.taskId === taskId)
    if (task) {
      task.status = status
    }
  }

  function updateTaskProgress(taskId: string, progress: number, downloaded: number, speed: number) {
    const task = tasks.value.find(t => t.taskId === taskId)
    if (task) {
      task.progress = progress
      task.downloaded = downloaded
      task.speed = speed
    }
  }

  // 监听下载进度事件
  function setupEventListeners() {
    listenEvent<DownloadProgress>('download-progress', (event) => {
      const { taskId, progress, downloaded, speed } = event.payload
      updateTaskProgress(taskId, progress, downloaded, speed)
    })
    
    listenEvent<{ taskId: string }>('download-completed', (event) => {
      updateTaskStatus(event.payload.taskId, 'completed')
      activeDownloads.value--
      notification.success('下载完成')
    })
    
    listenEvent<{ taskId: string; error: string }>('download-error', (event) => {
      updateTaskStatus(event.payload.taskId, 'failed')
      activeDownloads.value--
      notification.error(`下载失败: ${event.payload.error}`)
    })
  }

  return {
    tasks,
    activeDownloads,
    downloadingTasks,
    completedTasks,
    totalProgress,
    startDownload,
    pauseDownload,
    resumeDownload,
    cancelDownload,
    setupEventListeners,
  }
})
```

### 5.4 Vue组件示例

#### 登录组件 (src/components/auth/LoginForm.vue)

```vue
<template>
  <div class="login-container">
    <el-card class="login-card">
      <template #header>
        <h2 class="login-title">应用中心</h2>
      </template>
      
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        @keyup.enter="handleLogin"
      >
        <el-form-item label="用户名" prop="username">
          <el-input
            v-model="form.username"
            placeholder="请输入用户名"
            prefix-icon="User"
            size="large"
          />
        </el-form-item>
        
        <el-form-item label="密码" prop="password">
          <el-input
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            prefix-icon="Lock"
            size="large"
            show-password
          />
        </el-form-item>
        
        <el-form-item>
          <el-checkbox v-model="form.remember">记住密码</el-checkbox>
        </el-form-item>
        
        <el-form-item>
          <el-button
            type="primary"
            size="large"
            :loading="authStore.loading"
            @click="handleLogin"
            style="width: 100%"
          >
            登录
          </el-button>
        </el-form-item>
      </el-form>
      
      <el-alert
        v-if="authStore.error"
        :title="authStore.error"
        type="error"
        :closable="false"
        show-icon
        style="margin-top: 16px"
      />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import type { FormInstance, FormRules } from 'element-plus'

const router = useRouter()
const authStore = useAuthStore()
const formRef = ref<FormInstance>()

const form = reactive({
  username: '',
  password: '',
  remember: false,
})

const rules: FormRules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }],
}

const handleLogin = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      const success = await authStore.login(form.username, form.password)
      if (success) {
        router.push('/')
      }
    }
  })
}
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 400px;
}

.login-title {
  text-align: center;
  margin: 0;
  color: #409eff;
  font-size: 24px;
}
</style>
```

#### 软件列表组件 (src/components/software/SoftwareList.vue)

```vue
<template>
  <div class="software-list">
    <el-row :gutter="16">
      <el-col
        v-for="software in softwareList"
        :key="software.id"
        :xs="24"
        :sm="12"
        :md="8"
        :lg="6"
        class="software-col"
      >
        <software-card
          :software="software"
          @download="handleDownload"
        />
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSoftwareStore } from '@/stores/software'
import { useDownloadStore } from '@/stores/download'
import SoftwareCard from './SoftwareCard.vue'

const softwareStore = useSoftwareStore()
const downloadStore = useDownloadStore()
const softwareList = ref<Software[]>([])

onMounted(async () => {
  await softwareStore.fetchSoftwareList()
  softwareList.value = softwareStore.softwareList
})

const handleDownload = async (software: Software) => {
  // 选择保存路径
  const savePath = await softwareStore.selectDownloadPath()
  if (savePath) {
    await downloadStore.startDownload(software, savePath)
  }
}
</script>

<style scoped>
.software-col {
  margin-bottom: 16px;
}
</style>
```

## 6. 类型定义

### 6.1 TypeScript类型 (src/types/)

```typescript
// user.d.ts
interface UserInfo {
  id: number
  username: string
  nickname: string
  email: string
  avatar?: string
  role: 'admin' | 'user'
}

// software.d.ts
interface Software {
  id: number
  name: string
  description: string
  version: string
  fileName: string
  fileSize: number
  icon?: string
  categoryId: number
  categoryName: string
  downloadCount: number
  createdAt: string
}

// scan.d.ts
interface ProcessInfo {
  pid: number
  name: string
  exe_path?: string
  memory: number
  cpu_usage: number
}

interface ScanResult {
  scan_time: string
  total_processes: number
  processes: string[]
  blacklisted_processes: ProcessInfo[]
}

type DownloadStatus = 'pending' | 'downloading' | 'paused' | 'completed' | 'failed' | 'cancelled'

interface DownloadTask {
  taskId: string
  softwareId: number
  softwareName: string
  fileName: string
  fileSize: number
  savePath: string
  status: DownloadStatus
  progress: number
  downloaded: number
  speed: number
  errorMessage?: string
}

interface DownloadProgress {
  taskId: string
  progress: number
  downloaded: number
  speed: number
}

// api.d.ts
interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

interface AppConfig {
  server: {
    base_url: string
    timeout: number
    retry_times: number
  }
  scan: {
    interval: number
    enabled: boolean
  }
  heartbeat: {
    interval: number
  }
  download: {
    save_path?: string
    concurrent: number
  }
  ui: {
    minimize_to_tray: boolean
    auto_start: boolean
  }
}

interface SystemInfo {
  deviceName: string
  osType: string
  osVersion: string
  macAddress: string
  ipAddress: string
}
```

## 7. 构建与打包

### 7.1 package.json

```json
{
  "name": "apphub-client",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "lint": "eslint . --ext .vue,.ts,.tsx --fix",
    "format": "prettier --write ."
  },
  "dependencies": {
    "vue": "^3.3.8",
    "vue-router": "^4.2.5",
    "pinia": "^2.1.7",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-notification": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0",
    "@tauri-apps/plugin-shell": "^2.0.0",
    "@tauri-apps/plugin-http": "^2.0.0",
    "element-plus": "^2.4.4",
    "@element-plus/icons-vue": "^2.1.1",
    "axios": "^1.6.2"
  },
  "devDependencies": {
    "@types/node": "^20.9.0",
    "@vitejs/plugin-vue": "^4.5.0",
    "@tauri-apps/cli": "^2.0.0",
    "typescript": "^5.2.2",
    "vue-tsc": "^1.8.22",
    "vite": "^5.0.0",
    "eslint": "^8.54.0",
    "@vue/eslint-config-typescript": "^12.0.0",
    "prettier": "^3.1.0"
  }
}
```

### 7.2 Cargo.toml

```toml
[package]
name = "apphub-client"
version = "1.0.0"
description = "应用中心客户端"
edition = "2021"
rust-version = "1.70"

[dependencies]
# Tauri核心
tauri = { version = "2.0", features = [] }
tauri-plugin-notification = "2.0"
tauri-plugin-dialog = "2.0"
tauri-plugin-shell = "2.0"
tauri-plugin-http = "2.0"
tauri-plugin-updater = "2.0"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# HTTP客户端
reqwest = { version = "0.11", features = ["json", "stream"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 系统信息
sysinfo = "0.29"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 错误处理
thiserror = "1.0"

# 工具库
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
dirs = "5.0"
uuid = { version = "1.6", features = ["v4"] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = 3
strip = true
```

### 7.3 构建命令

```bash
# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build

# 构建特定平台
# Windows
npm run tauri:build -- --target x86_64-pc-windows-msvc

# macOS
npm run tauri:build -- --target x86_64-apple-darwin
npm run tauri:build -- --target aarch64-apple-darwin

# Linux
npm run tauri:build -- --target x86_64-unknown-linux-gnu
```

## 8. 与Server端交互流程

### 8.1 登录流程

```
Vue组件 (LoginForm.vue)
    ↓ 调用
Pinia Store (auth.ts)
    ↓ invoke
Tauri Command (auth.rs)
    ↓ HTTP POST
Rust API Client (reqwest)
    ↓ HTTPS
Server (Axum/Actix-web)
    ↓ JWT Token
Tauri Command
    ↓ 返回
Pinia Store (存储Token)
    ↓ 跳转
Vue Router (HomeView.vue)
```

### 8.2 进程扫描流程

```
Rust后台服务 (scanner.rs)
    ↓ 定时触发 (tokio::time::interval)
sysinfo crate (获取进程列表)
    ↓ 对比
黑名单缓存 (DashMap)
    ↓ 发现异常
生成扫描结果 (ScanResult)
    ↓ emit_event
Vue前端监听 (scan.ts store)
    ↓ 更新状态
UI显示警告 (Notification)
    ↓ HTTP POST
上报Server (reporter.rs)
```

### 8.3 软件下载流程

```
Vue组件 (SoftwareCard.vue)
    ↓ 点击下载
Pinia Store (download.ts)
    ↓ invoke
Tauri Command (download.rs)
    ↓ 创建任务
Rust下载服务 (downloader.rs)
    ↓ HTTP GET (reqwest)
Server (文件流)
    ↓ emit_event
下载进度事件 (download-progress)
    ↓ 监听
Vue进度更新 (DownloadProgress.vue)
    ↓ 完成
文件保存到本地
```

## 9. 性能优化

### 9.1 Rust后端优化

```rust
// 1. 使用DashMap实现无锁并发
use dashmap::DashMap;
static BLACKLIST: Lazy<DashMap<String, i32>> = Lazy::new(|| DashMap::new());

// 2. 异步文件操作
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// 3. 连接池复用
use reqwest::Client;
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .pool_max_idle_per_host(10)
        .build()
        .unwrap()
});

// 4. 批量上报
use tokio::sync::mpsc::channel;
let (tx, mut rx) = channel::<ScanReport>(100);

// 5. 内存优化 - 使用Arc减少克隆
use std::sync::Arc;
let shared_config = Arc::new(config);
```

### 9.2 Vue前端优化

```typescript
// 1. 虚拟滚动
import { useVirtualList } from '@vueuse/core'
const { list, containerProps, wrapperProps } = useVirtualList(softwareList, {
  itemHeight: 80,
})

// 2. 防抖处理
import { debounce } from 'lodash-es'
const debouncedSearch = debounce((query: string) => {
  searchSoftware(query)
}, 300)

// 3. 组件懒加载
const SoftwareDetail = defineAsyncComponent(() => 
  import('./SoftwareDetail.vue')
)

// 4. 状态持久化
import { persist } from 'pinia-plugin-persistedstate'
```

## 10. 安全考虑

### 10.1 数据安全

```rust
// 1. Token安全存储
use keyring::Entry;
let entry = Entry::new("apphub", "token");
entry.set_password(&token)?;

// 2. 配置文件加密
use aes_gcm::{Aes256Gcm, Key, Nonce};
let cipher = Aes256Gcm::new(key);

// 3. 敏感信息脱敏
task.save_path.replace_home_dir_with("~");
```

### 10.2 通信安全

```rust
// 1. HTTPS强制验证
let client = Client::builder()
    .danger_accept_invalid_certs(false)
    .build()?;

// 2. 请求签名
use hmac::{Hmac, Mac};
type HmacSha256 = Hmac<sha2::Sha256>;
let mut mac = HmacSha256::new_from_slice(secret)?;
mac.update(request_body);
let signature = mac.finalize().into_bytes();
```

## 11. 部署与更新

### 11.1 自动更新配置

```rust
// src-tauri/src/main.rs
.plugin(tauri_plugin_updater::Builder::new()
    .on_progress(|app, progress| {
        app.emit_all("update-progress", progress).ok();
    })
    .on_finished(|app| {
        app.emit_all("update-finished", ()).ok();
    })
    .build())
```

### 11.2 更新检查

```typescript
// Vue前端
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'

async function checkForUpdates() {
  const update = await checkUpdate()
  if (update.shouldUpdate) {
    // 显示更新提示
    ElMessageBox.confirm(
      `发现新版本 ${update.manifest?.version}，是否更新？`,
      '更新提示',
      { confirmButtonText: '更新', cancelButtonText: '稍后' }
    ).then(async () => {
      await installUpdate()
      // 重启应用
      await relaunch()
    })
  }
}
```

---

以上是Tauri版本的Client端详细设计文档，涵盖了项目结构、Tauri配置、Rust后端核心模块、Vue前端核心模块、类型定义、构建打包、交互流程、性能优化、安全考虑和部署更新等内容。相比原Python/PyQt5方案，Tauri方案具有体积小、性能好、安全性高、跨平台支持好等优势。
