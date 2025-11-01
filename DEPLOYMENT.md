# ClassTop Management Server - 部署指南

本指南帮助你部署和运行 ClassTop Management Server，包括前端、后端和自动启动配置。

## 📋 目录

- [系统要求](#系统要求)
- [快速启动](#快速启动)
- [详细配置](#详细配置)
- [Nginx 配置](#nginx-配置)
- [自动启动](#自动启动)
- [故障排查](#故障排查)

## 系统要求

### 基础要求

- **操作系统**: macOS, Linux, Windows Server
- **Rust**: 1.70+ (稳定版)
- **Node.js**: 16+ (用于前端构建)
- **PostgreSQL**: 14+
- **Nginx**: 1.18+ (可选，推荐用于生产环境)

### macOS 安装依赖

```bash
# 安装 Homebrew（如果未安装）
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装依赖
brew install rust node postgresql nginx

# 启动 PostgreSQL
brew services start postgresql
```

### Linux (Ubuntu) 安装依赖

```bash
# 更新包列表
sudo apt update

# 安装依赖
sudo apt install -y build-essential curl postgresql postgresql-contrib nginx

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js (通过 NodeSource)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# 启动 PostgreSQL
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

## 快速启动

### 1. 克隆项目

```bash
git clone https://github.com/Zixiao-System/Classtop-Management-Server.git
cd Classtop-Management-Server
```

### 2. 配置环境

```bash
# 复制环境配置
cp .env.example .env

# 生成 JWT 密钥
openssl rand -base64 32

# 编辑 .env 文件
nano .env
```

**必须配置的项：**

```env
# 数据库连接
DATABASE_URL=postgresql://username:password@localhost:5432/classtop

# JWT 密钥（使用上面生成的）
JWT_SECRET=生成的密钥粘贴在这里

# CORS 配置
CORS_ALLOWED_ORIGINS=http://localhost,http://localhost:8765

# 启用认证
ENABLE_AUTH=true
```

### 3. 创建数据库

```bash
# PostgreSQL
psql -U postgres
CREATE DATABASE classtop;
\q
```

### 4. 使用一键启动脚本

```bash
# 赋予执行权限
chmod +x start.sh

# 启动服务（会自动构建前端、检查依赖、配置 Nginx）
./start.sh
```

### 5. 访问服务

- **前端界面**: http://localhost （通过 Nginx）或 http://localhost:8765 （直连）
- **API 文档**: http://localhost:8765/api/docs
- **健康检查**: http://localhost:8765/api/health

### 6. 创建管理员账户

首次访问会进入注册页面：

1. 点击"注册"标签
2. 输入用户名和密码
3. 点击"注册"按钮
4. 自动登录到管理面板

## 详细配置

### 环境变量说明

| 变量 | 说明 | 默认值 | 必需 |
|------|------|--------|------|
| `DATABASE_URL` | PostgreSQL 连接字符串 | - | ✅ |
| `JWT_SECRET` | JWT 签名密钥 | - | ✅ |
| `ENABLE_AUTH` | 是否启用认证 | true | ❌ |
| `CORS_ALLOWED_ORIGINS` | CORS 允许的源 | localhost:5173,localhost:8765 | ❌ |
| `HOST` | 服务器监听地址 | 0.0.0.0 | ❌ |
| `PORT` | 服务器监听端口 | 8765 | ❌ |
| `RUST_LOG` | 日志级别 | info | ❌ |

### 手动构建前端

```bash
cd frontend
npm install
npm run build
cd ..
```

构建后的文件会自动输出到 `static/` 目录。

### 手动运行后端

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/classtop-management-server
```

## Nginx 配置

### macOS 配置

```bash
# 编辑 Nginx 配置
nano /opt/homebrew/etc/nginx/nginx.conf

# 或者创建独立配置（推荐）
sudo mkdir -p /opt/homebrew/etc/nginx/servers
sudo cp nginx.conf /opt/homebrew/etc/nginx/servers/classtop.conf

# 测试配置
nginx -t

# 重启 Nginx
brew services restart nginx
```

### Linux 配置

```bash
# 复制配置文件
sudo cp nginx.conf /etc/nginx/sites-available/classtop.conf

# 创建符号链接
sudo ln -s /etc/nginx/sites-available/classtop.conf /etc/nginx/sites-enabled/

# 测试配置
sudo nginx -t

# 重启 Nginx
sudo systemctl restart nginx
```

### Nginx 配置要点

1. **静态文件路径**: 修改 `root` 指向项目的 `static` 目录
2. **后端代理**: 确保 `proxy_pass` 指向正确的后端地址（默认 8765）
3. **域名**: 生产环境修改 `server_name`
4. **HTTPS**: 生产环境启用 SSL 配置

## 自动启动

### macOS (launchd)

```bash
# 修改 plist 文件中的路径
nano com.classtop.management-server.plist

# 安装服务
cp com.classtop.management-server.plist ~/Library/LaunchAgents/

# 加载并启动服务
launchctl load ~/Library/LaunchAgents/com.classtop.management-server.plist

# 检查状态
launchctl list | grep classtop

# 查看日志
tail -f /tmp/classtop-server.log

# 停止服务
launchctl unload ~/Library/LaunchAgents/com.classtop.management-server.plist
```

### Linux (systemd)

```bash
# 修改 service 文件中的路径和用户
sudo nano classtop.service

# 安装服务
sudo cp classtop.service /etc/systemd/system/

# 重新加载 systemd
sudo systemctl daemon-reload

# 启用服务（开机自启）
sudo systemctl enable classtop

# 启动服务
sudo systemctl start classtop

# 查看状态
sudo systemctl status classtop

# 查看日志
sudo journalctl -u classtop -f

# 停止服务
sudo systemctl stop classtop
```

## 生产环境部署清单

### 安全性

- [ ] 生成强 JWT 密钥（至少 32 字符）
- [ ] 启用 HTTPS（使用 Let's Encrypt 或其他 SSL 证书）
- [ ] 配置严格的 CORS 白名单
- [ ] 设置防火墙规则（只开放 80, 443 端口）
- [ ] 定期备份数据库
- [ ] 使用非 root 用户运行服务

### 性能

- [ ] 启用 Nginx gzip 压缩
- [ ] 配置静态资源缓存
- [ ] 调整数据库连接池大小
- [ ] 监控服务器资源使用
- [ ] 配置日志轮转

### 监控

- [ ] 设置健康检查监控
- [ ] 配置日志聚合（如 ELK）
- [ ] 设置错误报警（如 Sentry）
- [ ] 监控 API 性能

## 故障排查

### 前端无法访问

**检查：**

1. Nginx 是否正常运行：`nginx -t` 或 `systemctl status nginx`
2. 静态文件是否已构建：检查 `static/` 目录
3. 端口是否被占用：`lsof -i :80` 或 `netstat -tulpn | grep :80`

### 后端 API 错误

**检查：**

1. 数据库连接：`psql -U username -d classtop`
2. 环境变量配置：检查 `.env` 文件
3. 日志输出：`RUST_LOG=debug cargo run`
4. 端口冲突：`lsof -i :8765`

### 认证问题

**检查：**

1. JWT_SECRET 是否配置
2. 浏览器控制台是否有 CORS 错误
3. Token 是否过期（默认 24 小时）
4. 清除浏览器 LocalStorage 重新登录

### 数据库迁移失败

**解决：**

```bash
# 手动运行迁移
psql -U username -d classtop < migrations/001_initial_postgresql.sql
psql -U username -d classtop < migrations/003_add_lms_support.sql
psql -U username -d classtop < migrations/004_add_cctv_support.sql
psql -U username -d classtop < migrations/005_add_user_auth.sql
```

### Nginx 502 Bad Gateway

**原因：** 后端服务未运行或端口错误

**解决：**

1. 检查后端服务：`ps aux | grep classtop`
2. 检查后端日志
3. 确认 `proxy_pass` 地址正确

## 更新部署

```bash
# 1. 停止服务
sudo systemctl stop classtop  # Linux
# 或
launchctl unload ~/Library/LaunchAgents/com.classtop.management-server.plist  # macOS

# 2. 拉取最新代码
git pull origin main

# 3. 重新构建
cd frontend && npm install && npm run build && cd ..
cargo build --release

# 4. 运行数据库迁移（如果有新迁移）
# 服务启动时会自动运行

# 5. 重启服务
sudo systemctl start classtop  # Linux
# 或
launchctl load ~/Library/LaunchAgents/com.classtop.management-server.plist  # macOS
```

## 开发模式

开发时可以禁用认证以便于测试：

```env
# .env
ENABLE_AUTH=false
```

这样就可以直接访问所有 API 而无需 token。

**⚠️ 注意：生产环境必须启用认证！**

## 性能调优

### 数据库连接池

编辑 `src/db.rs`：

```rust
let pool = PgPoolOptions::new()
    .max_connections(20)  // 增加连接数
    .acquire_timeout(Duration::from_secs(30))
    .connect(database_url)
    .await?;
```

### API 限流

编辑 `src/main.rs`：

```rust
let governor_conf = GovernorConfigBuilder::default()
    .per_second(5)  // 调整每秒请求数
    .burst_size(200)  // 调整突发数量
    .finish()
    .unwrap();
```

## 备份策略

### 数据库备份

```bash
# 备份
pg_dump -U username classtop > backup_$(date +%Y%m%d).sql

# 恢复
psql -U username classtop < backup_20241101.sql
```

### 自动备份脚本

```bash
#!/bin/bash
# backup.sh
BACKUP_DIR="/path/to/backups"
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump -U postgres classtop | gzip > "$BACKUP_DIR/classtop_$DATE.sql.gz"

# 保留最近 7 天的备份
find "$BACKUP_DIR" -name "classtop_*.sql.gz" -mtime +7 -delete
```

配置定时任务：

```bash
# crontab -e
0 2 * * * /path/to/backup.sh
```

## 支持和帮助

- **GitHub Issues**: https://github.com/Zixiao-System/Classtop-Management-Server/issues
- **API 文档**: http://localhost:8765/api/docs
- **项目文档**: 查看 `docs/` 目录

---

**最后更新**: 2024-11-01
**版本**: v1.1.0
