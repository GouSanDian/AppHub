# AppHub
AppHub，企业私有化部署的软件管家，服务端管理员可以监控操作系统运行了哪些软件，从而进行管理。

AppHub 采用 Rust + Tauri 技术栈构建。

## 系统架构

```
┌─────────────────────────────────────────────────────────┐
│                      用户层                              │
├──────────────────────┬──────────────────────────────────┤
│   管理员（浏览器）      │      普通用户（桌面客户端）          │
└──────────────────────┴──────────────────────────────────┘
           │                         │
           │ HTTPS                   │ HTTPS
           ▼                         ▼
┌─────────────────────────────────────────────────────────┐
│                    应用层（Server）                       │
│              Rust + Axum + SeaORM + PostgreSQL          │
└─────────────────────────────────────────────────────────┘
```

## 技术选型

### 服务端 (Server)
- **语言**: Rust 1.70+
- **Web框架**: Axum 0.7
- **ORM**: SeaORM 0.12
- **数据库**: PostgreSQL 15+
- **日志**: tracing

### 客户端 (Client)
- **框架**: Tauri 2.0
- **后端**: Rust
- **前端**: Vue 3 + TypeScript
- **UI**: Element Plus
- **状态管理**: Pinia

## 项目结构

```
apphub/
├── apphub-server/          # 服务端
│   ├── src/
│   ├── migrations/         # 数据库迁移
│   └── Cargo.toml
├── apphub-client/          # 客户端
│   ├── src/                # Vue前端
│   ├── src-tauri/          # Rust后端
│   └── package.json
├── docs/                   # 设计文档
│   ├── 01-系统总体设计.md
│   ├── 02-数据库设计.md
│   ├── 03-API接口设计.md
│   ├── 04-Server端详细设计.md
│   └── 05-Client端详细设计.md
└── README.md
```

## 开发计划

| 阶段 | 内容 | 预计时间 |
|------|------|----------|
| 第一阶段 | 基础框架搭建、数据库设计 | 1周 ✅ |
| 第二阶段 | 用户管理、认证授权 | 1.5周 ✅|
| 第三阶段 | 软件管理、文件上传下载 | 2.5周 ✅ |
| 第四阶段 | 黑名单管理、进程监控 | 1.5周 |
| 第五阶段 | 监控报表、系统管理 | 1周 |
| 第六阶段 | 测试、优化、文档 | 1.5周 |

## 快速开始

### 环境要求

- Rust 1.70+
- Node.js 18+
- PostgreSQL 15+

### 启动服务端

```bash
cd apphub-server

# 配置环境变量
cp .env.example .env

# 运行数据库迁移
# (详见 apphub-server/README.md)

# 启动服务
cargo run --bin apphub-server
```

### 启动客户端

```bash
cd apphub-client

# 安装依赖
npm install

# 开发模式
npm run tauri:dev
```

## 默认账号
```
默认管理员账号

在 migrations/001_create_users.sql 中通过 SQL 种子数据创建：

┌────────┬──────────────────────────────────────────────────┐
│  字段  │                        值                        │
├────────┼──────────────────────────────────────────────────┤
│ 用户名 │ admin                                            │
├────────┼──────────────────────────────────────────────────┤
│ 密码   │ admin（bcrypt hash 是广泛流传的示例 hash） │
├────────┼──────────────────────────────────────────────────┤
│ 角色   │ super_admin（role_id = 1）                       │
├────────┼──────────────────────────────────────────────────┤
│ 昵称   │ 超级管理员                                       │
└────────┴──────────────────────────────────────────────────┘

默认角色（migrations/002_create_roles.sql）

┌─────┬─────────────┬────────────┐
│ id  │    name     │    描述    │
├─────┼─────────────┼────────────┤
│ 1   │ super_admin │ 超级管理员 │
├─────┼─────────────┼────────────┤
│ 2   │ admin       │ 管理员     │
├─────┼─────────────┼────────────┤
│ 3   │ user        │ 普通用户   │
└─────┴─────────────┴────────────┘
```


## 文档

- [系统总体设计](docs/01-系统总体设计.md)
- [数据库设计](docs/02-数据库设计.md)
- [API接口设计](docs/03-API接口设计.md)
- [Server端详细设计](docs/04-Server端详细设计.md)
- [Client端详细设计](docs/05-Client端详细设计.md)


# 🤝欢迎 Star、使用我们团队的其它产品

大模型呼叫中心系统：https://github.com/lihaiya/freeipcc

大模型人力资源系统：https://github.com/FreeAiHR/FreeAiHR

大模型智能运维系统（持续迭代中）：https://github.com/lihaiya/FreeAiOps

Redis集群双活架构工具：https://github.com/GouSanDian/Redis-HA-Tool

自然语言对话数据库（目前在设计阶段）：https://github.com/GouSanDian/Talk-To-DB

企业私有化部署的软件管家：https://github.com/GouSanDian/AppHub

大模型沙盘系统（目前在设计阶段）：https://github.com/AiSandTable/AiSandTable

# 联系我们

## 微信专属群

<img width="206.2" height="295.6" alt="24a169629bcfead1e3f545e75b8d0e58" src="https://github.com/user-attachments/assets/3953f763-5368-4a97-bf94-6a46c6d83286" />

## 微信二维码

<img width="190.0" height="259.0" alt="b6add25034c22a813d7b45b17c95762a" src="https://github.com/user-attachments/assets/3f5f2463-6e87-43d6-95fe-a24a572fa9f2" />



