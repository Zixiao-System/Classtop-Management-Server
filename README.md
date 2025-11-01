# ClassTop Management Server

ClassTop 客户端的集中管理服务器，用于管理多个 ClassTop 客户端设备并同步数据。

## 🎯 项目简介

这是 [ClassTop](https://github.com/Zixiao-System/classtop) 项目的配套管理服务器，提供：

- 📱 **多客户端管理** - 注册和管理多个 ClassTop 客户端设备
- 🔄 **数据同步** - 从客户端同步课程和课程表数据
- 📊 **统计分析** - 查看所有客户端的统计信息
- 🎨 **Web 管理界面** - 基于 Material Design 的现代化管理后台
- 🗄️ **数据库支持** - PostgreSQL (完全支持) / SQL Server (Developer Build 可用)

## ✨ 功能特性

### 客户端管理
- 客户端注册/注销
- 客户端状态监控 (online/offline)
- 客户端信息管理
- 查看客户端详情

### 数据同步
- 自动同步客户端课程数据
- 自动同步课程表数据
- 同步历史记录
- 冲突处理

### 数据查看
- 按客户端查看课程列表
- 按客户端查看课程表
- 统计信息展示
- 客户端对比分析

### API 接口
- RESTful API 设计
- Swagger UI 文档
- ReDoc 文档
- 完整的 OpenAPI 规范

## 🛠️ 技术栈

- **后端框架**: Actix-Web 4.9
- **数据库**: SQLx (PostgreSQL / SQL Server)
- **API 文档**: utoipa + Swagger UI + ReDoc
- **前端框架**: Vue 3 + Vite
- **前端 UI**: MDUI 2 (Material Design)
- **语言**: Rust 2021 Edition

## 💻 平台支持

| 平台                   | 支持状态 | 说明                                             |
|----------------------|------|------------------------------------------------|
| ✅ **Windows Server** | 完全支持 | 推荐用于生产环境                                       |
| ✅ **Linux**          | 完全支持 | 推荐用于生产环境 (Ubuntu, CentOS, Debian 等)            |
| ✅ **macOS**          | 完全支持 | 推荐用于生产环境(支持Apple silicon和Intel)，无需macOS Server |

## 📦 快速开始

### 环境要求

- Rust 1.70+
- Node.js 18+ (用于前端开发)
- PostgreSQL 14+ 或 SQL Server 2019+
- 操作系统: Windows Server / Linux / macOS

> 💡 **数据库选择**:
> - **PostgreSQL** - 生产环境推荐，跨平台支持完善
> - **SQL Server** - Developer Build 可用，适合 Windows Server 环境（使用自研驱动）

### 安装步骤

1. **克隆项目**
```bash
git clone https://github.com/ZiXiao-System/Classtop-Management-Server.git
cd Classtop-Management-Server
```

2. **配置数据库**

复制环境变量示例文件：
```bash
cp .env.example .env
```

编辑 `.env` 文件，配置数据库：

**PostgreSQL 配置:**
```env
DATABASE_TYPE=postgresql
DATABASE_URL=postgresql://username:password@localhost:5432/classtop
HOST=0.0.0.0
PORT=8765
```

**SQL Server 配置:**
```env
DATABASE_TYPE=mssql
MSSQL_HOST=localhost
MSSQL_PORT=1433
MSSQL_USERNAME=sa
MSSQL_PASSWORD=YourPassword
MSSQL_DATABASE=classtop
HOST=0.0.0.0
PORT=8765
```

<details>
<summary><b>SQL Server 支持状态 (Developer Build 可用)</b></summary>

SQL Server 驱动目前处于 **Developer Build** 阶段，已实现核心功能：

**✅ 已实现功能:**
- ✅ TDS 7.4 协议实现
- ✅ Pre-Login 握手
- ✅ Login7 认证（含密码混淆）
- ✅ SQL Batch 查询执行
- ✅ 列元数据解析
- ✅ 行数据值解析
- ✅ 事务支持 (BEGIN/COMMIT/ROLLBACK)
- ✅ 常用数据类型支持（INT, VARCHAR, NVARCHAR, FLOAT 等）
- ✅ 错误处理

**⏳ 待完善功能:**
- ⏳ 参数化查询 (sp_executesql)
- ⏳ 连接池
- ⏳ DateTime/Decimal 类型支持
- ⏳ 生产环境稳定性测试

**项目位置**: `mssql-driver/` (独立子项目)

**使用方式**:
```rust
use mssql_driver::{Connection, ConnectionConfig};

let config = ConnectionConfig::builder()
    .host("localhost")
    .port(1433)
    .username("sa")
    .password("password")
    .database("classtop")
    .build()?;

let mut conn = Connection::connect(config).await?;
let result = conn.query("SELECT * FROM clients").await?;

// 事务支持
conn.begin_transaction().await?;
conn.query("INSERT INTO ...").await?;
conn.commit().await?;
```

**注意事项**:
- 🔬 Developer Build 质量，建议在开发/测试环境使用
- 🐛 如遇问题请提交 Issue
- 📝 详见: [MSSQL Status](docs/MSSQL_STATUS.md)

</details>

3. **构建前端**

```bash
cd frontend
npm install
npm run build
cd ..
```

构建完成后，前端文件会输出到 `static/` 目录，后端会自动提供这些文件。

4. **运行项目**

开发模式：
```bash
cargo run
```

生产模式：
```bash
cargo run --release
```

5. **访问服务**

- **Web 管理界面**: http://localhost:8765
- **Swagger UI**: http://localhost:8765/api/docs
- **ReDoc**: http://localhost:8765/api/redoc
- **健康检查**: http://localhost:8765/api/health

### 前端开发

如果需要修改前端代码，可以使用开发模式：

```bash
cd frontend
npm run dev
```

前端开发服务器会在 http://localhost:5173 启动，并自动代理 API 请求到后端服务器。

## 📖 API 文档

### 客户端管理

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/clients` | 获取所有客户端 |
| GET | `/api/clients/{id}` | 获取单个客户端 |
| POST | `/api/clients/register` | 注册新客户端 |
| PUT | `/api/clients/{id}` | 更新客户端信息 |
| DELETE | `/api/clients/{id}` | 删除客户端 |
| GET | `/api/clients/{id}/courses` | 获取客户端课程 |
| GET | `/api/clients/{id}/schedule` | 获取客户端课程表 |

### 数据同步

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | `/api/sync` | 同步客户端数据 |

### 统计信息

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/statistics` | 获取服务器统计 |
| GET | `/api/statistics/clients` | 获取客户端统计 |

### 设置管理

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/settings` | 获取所有设置 |
| GET | `/api/settings/{key}` | 获取单个设置 |
| PUT | `/api/settings/{key}` | 更新设置 |

详细的 API 文档请访问 Swagger UI: http://localhost:8765/api/docs

## 🔄 客户端同步

ClassTop 客户端可以通过以下方式同步数据到管理服务器：

### 同步请求格式

```json
POST /api/sync
{
  "client_uuid": "550e8400-e29b-41d4-a716-446655440000",
  "courses": [
    {
      "id": 1,
      "name": "高等数学",
      "teacher": "张三",
      "color": "#FF5722",
      "note": null
    }
  ],
  "schedule_entries": [
    {
      "id": 1,
      "course_id": 1,
      "day_of_week": 1,
      "start_time": "08:00",
      "end_time": "09:40",
      "weeks": [1, 2, 3, 4, 5]
    }
  ]
}
```

### 响应格式

```json
{
  "success": true,
  "message": "Data synced successfully",
  "synced_courses": 1,
  "synced_entries": 1
}
```

## 🗄️ 数据库架构

### 主要表结构

**clients** - 客户端信息
- id, uuid, name, description
- api_url, api_key
- last_sync, status, created_at

**courses** - 课程信息
- id, client_id, course_id_on_client
- name, teacher, color, note
- synced_at

**schedule_entries** - 课程表
- id, client_id, entry_id_on_client, course_id
- day_of_week, start_time, end_time, weeks
- synced_at

**sync_logs** - 同步日志
- id, client_id, sync_type, status
- courses_count, entries_count, error_message
- created_at

**settings** - 服务器配置
- key, value

## 🔧 配置说明

### 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| DATABASE_URL | PostgreSQL 数据库连接字符串 | - |
| HOST | 服务器监听地址 | 0.0.0.0 |
| PORT | 服务器端口 | 8765 |
| APP_VERSION | 应用版本 | 1.0.0 |
| RUST_LOG | 日志级别 | info |

### 服务器设置

可通过 API 或数据库修改以下设置：

- `server_name` - 服务器名称
- `auto_sync_interval` - 自动同步间隔（秒）
- `max_clients` - 最大客户端数量

## 📂 项目结构

```
Classtop-Management-Server/
├── src/                      # 后端源代码
│   ├── main.rs              # 应用入口
│   ├── config.rs            # 配置管理
│   ├── db.rs                # 数据库连接和仓储
│   ├── models.rs            # 数据模型
│   ├── handlers.rs          # API 处理器
│   ├── routes.rs            # 路由配置
│   └── error.rs             # 错误处理
├── mssql-driver/            # SQL Server 驱动（独立子项目）
│   ├── src/
│   │   ├── connection/      # 连接管理
│   │   ├── protocol/        # TDS 协议实现
│   │   ├── types.rs         # 类型系统
│   │   └── error.rs         # 错误处理
│   ├── examples/            # 示例代码
│   │   ├── test_connection.rs
│   │   └── test_query.rs
│   └── Cargo.toml           # 驱动依赖配置
├── frontend/                # 前端源代码 (Vue 3)
│   ├── src/
│   │   ├── App.vue          # 主应用组件
│   │   ├── main.js          # 前端入口
│   │   ├── api.js           # API 请求封装
│   │   └── components/      # Vue 组件
│   │       ├── DashboardView.vue
│   │       ├── ClientsView.vue
│   │       └── DataView.vue
│   ├── index.html           # HTML 模板
│   ├── vite.config.js       # Vite 配置
│   └── package.json         # 前端依赖
├── migrations/              # 数据库迁移文件
│   ├── 001_initial_postgresql.sql
│   └── 002_initial_mssql.sql
├── static/                  # 前端构建输出 (由 frontend/npm run build 生成)
├── docs/                    # 文档
│   ├── ClassTop-Client-API.md         # ClassTop 客户端 API 文档
│   ├── CLIENT_ADAPTATION.md           # 客户端适配指南
│   ├── CLIENT_INTEGRATION_TODO.md     # 客户端集成任务清单
│   ├── MSSQL_SETUP.md                 # SQL Server 配置指南
│   └── MSSQL_STATUS.md                # SQL Server 支持状态
├── Cargo.toml               # Rust 项目依赖
├── .env.example             # 环境变量示例
└── README.md                # 项目说明
```

## 🚀 部署

### 生产部署步骤

1. **构建前端**
```bash
cd frontend
npm install
npm run build
cd ..
```

2. **构建后端**
```bash
cargo build --release
```

3. **运行服务**
```bash
# 确保 .env 文件配置正确
./target/release/classtop-management-server
```

### Docker 部署 (推荐)

Dockerfile 示例：
```dockerfile
FROM rust:1.70 AS backend-builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM node:18 AS frontend-builder
WORKDIR /app
COPY frontend ./
RUN npm install && npm run build

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend-builder /app/target/release/classtop-management-server .
COPY --from=frontend-builder /app/dist ./static
COPY migrations ./migrations
ENV DATABASE_URL=postgresql://user:pass@host:5432/db
ENV HOST=0.0.0.0
ENV PORT=8765
EXPOSE 8765
CMD ["./classtop-management-server"]
```

构建和运行：
```bash
docker build -t classtop-server .
docker run -d \
  -p 8765:8765 \
  -e DATABASE_URL=postgresql://user:pass@host:5432/db \
  classtop-server
```

## 🔒 安全建议

- ⚠️ **生产环境必须配置身份验证**
- 🔐 使用防火墙限制数据库访问
- 🌐 配置 HTTPS（使用 Nginx/Caddy 反向代理）
- 🔑 使用强密码
- 📁 定期备份数据库
- 🚫 不要在公网直接暴露数据库端口

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🔗 相关项目

- [ClassTop](https://github.com/Zixiao-System/classtop) - ClassTop 客户端应用

## 📮 联系方式

c如有问题或建议，请：

- 提交 [Issue](https://github.com/YOUR_USERNAME/Classtop-Management-Server/issues)
- Pull Request

---

**Made with ZiXiao System ❤️ and Rust**
