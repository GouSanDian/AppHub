# AppHub Server

应用中心服务端 - 基于 Rust + Axum + SeaORM 构建

## 技术栈

- **Web框架**: Axum 0.7
- **数据库**: PostgreSQL / MySQL
- **ORM**: SeaORM 0.12
- **日志**: tracing
- **认证**: JWT

## 项目结构

```
apphub-server/
├── src/
│   ├── main.rs              # 入口文件
│   ├── config/              # 配置模块
│   ├── api/                 # API层
│   │   ├── routes.rs        # 路由定义
│   │   ├── handlers/        # 请求处理器
│   │   ├── middleware/      # 中间件
│   │   └── dto/             # 数据传输对象
│   ├── models/              # 数据模型 (SeaORM)
│   ├── services/            # 业务逻辑层
│   ├── utils/               # 工具函数
│   ├── error.rs             # 错误处理
│   └── constants.rs         # 常量定义
├── migrations/              # 数据库迁移文件
└── Cargo.toml
```

## 快速开始

### 1. 安装依赖

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 SeaORM CLI
cargo install sea-orm-cli
```

### 2. 配置环境变量

```bash
cp .env.example .env
# 编辑 .env 文件，配置数据库连接等信息
```

### 3. 运行数据库迁移

```bash
# 创建数据库
psql -U postgres -c "CREATE DATABASE apphub;"

# 执行迁移
psql -U postgres -d apphub -f migrations/20240101000001_create_users.sql
psql -U postgres -d apphub -f migrations/20240101000002_create_roles.sql
psql -U postgres -d apphub -f migrations/20240101000003_create_softwares.sql
psql -U postgres -d apphub -f migrations/20240101000004_create_blacklists.sql
psql -U postgres -d apphub -f migrations/20240101000005_create_clients.sql
psql -U postgres -d apphub -f migrations/20240101000006_create_scan_reports.sql
psql -U postgres -d apphub -f migrations/20240101000007_create_categories.sql
```

## 数据库配置
### PostgreSQL
当前 Cargo.toml 配置的是 PostgreSQL：

```
sea-orm = { version = "0.12", features = ["sqlx-postgres", "runtime-tokio-rustls", "macros"] }
```



### MySQL
```
sea-orm = { version = "0.12", features = ["sqlx-mysql", "runtime-tokio-rustls", "macros"] }
```

连接字符串改为：
DATABASE_URL=mysql://user:password@localhost:3306/apphub


### SQLite：
```
sea-orm = { version = "0.12", features = ["sqlx-sqlite", "runtime-tokio-rustls", "macros"] }
```

连接字符串改为：
```
DATABASE_URL=sqlite:./apphub.db?mode=rwc
```

### 4. 运行服务

```bash
# 开发模式
cargo run

# 生产构建
cargo build --release
```

## API文档

启动服务后访问: http://localhost:8080/api/v1/health

## 数据库表结构

- **users**: 用户表
- **roles**: 角色表
- **softwares**: 软件表
- **blacklists**: 黑名单表
- **clients**: 客户端表
- **scan_reports**: 扫描报告表
- **categories**: 分类表
