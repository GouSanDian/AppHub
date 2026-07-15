# AppHub Client

应用中心客户端 - 基于 Tauri + Vue 3 + TypeScript 构建

## 技术栈

- **桌面框架**: Tauri 2.0
- **后端语言**: Rust
- **前端框架**: Vue 3
- **状态管理**: Pinia
- **UI组件库**: Element Plus
- **构建工具**: Vite

## 项目结构

```
apphub-client/
├── src-tauri/               # Rust后端代码
│   ├── Cargo.toml
│   ├── tauri.conf.json      # Tauri配置
│   ├── capabilities/          # 权限配置
│   └── src/
│       ├── main.rs          # 主入口
│       ├── commands/        # Tauri Commands
│       ├── services/        # 后台服务
│       └── utils/           # 工具函数
├── src/                     # Vue前端代码
│   ├── main.ts              # Vue入口
│   ├── App.vue              # 根组件
│   ├── views/               # 页面视图
│   ├── components/          # 组件
│   ├── stores/              # Pinia状态管理
│   ├── router/              # 路由配置
│   ├── styles/              # 样式文件
│   └── types/               # TypeScript类型
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 快速开始

### 1. 安装依赖

```bash
# 安装 Node.js (推荐 18+)
# https://nodejs.org/

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Tauri CLI
cargo install tauri-cli
```

### 2. 安装项目依赖

```bash
npm install
```

### 3. 开发模式运行

```bash
# 同时启动前端和后端
npm run tauri:dev
```

### 4. 生产构建

```bash
# 构建生产版本
npm run tauri:build

# 输出目录: src-tauri/target/release/
```

## 功能模块

### 用户认证
- 登录/登出
- Token自动刷新
- 记住密码

### 软件中心
- 软件列表浏览
- 软件搜索和筛选
- 软件下载
- 下载进度显示

### 下载管理
- 下载任务列表
- 暂停/恢复下载
- 下载历史

### 安全监控
- 进程扫描
- 黑名单检测
- 异常上报

### 系统设置
- 服务器地址配置
- 下载路径设置
- 扫描间隔配置
- 开机自启动

## 开发指南

### 添加新的 Tauri Command

1. 在 `src-tauri/src/commands/` 下创建命令文件
2. 在 `src-tauri/src/main.rs` 中注册命令
3. 在前端调用

```typescript
import { invoke } from '@tauri-apps/api/core'

const result = await invoke('command_name', { arg: value })
```

### 添加新的页面

1. 在 `src/views/` 下创建 Vue 组件
2. 在 `src/router/index.ts` 中添加路由
3. 在侧边栏添加菜单项

## 打包发布

### Windows
```bash
npm run tauri:build -- --target x86_64-pc-windows-msvc
```

### macOS
```bash
npm run tauri:build -- --target x86_64-apple-darwin
npm run tauri:build -- --target aarch64-apple-darwin
```

### Linux
```bash
npm run tauri:build -- --target x86_64-unknown-linux-gnu
```
