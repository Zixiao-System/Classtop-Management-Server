# Microsoft SQL Server 配置指南

本文档说明如何在 ClassTop Management Server 中使用 Microsoft SQL Server 作为数据库。

## 📋 概述

ClassTop Management Server 支持以下数据库：
- **PostgreSQL** 14+ (推荐用于 Linux/macOS)
- **Microsoft SQL Server** 2019+ (推荐用于 Windows Server)

本指南专注于 SQL Server 的配置和使用。

---

## 🔧 SQL Server 安装

### Windows Server

#### 方式 1: SQL Server Express (免费)

1. **下载 SQL Server 2019/2022 Express**
   - 访问: https://www.microsoft.com/sql-server/sql-server-downloads
   - 选择 "Express" 版本下载

2. **安装步骤**
   ```powershell
   # 运行安装程序
   # 选择 "Basic" 或 "Custom" 安装
   # 记录安装路径和实例名称（默认：SQLEXPRESS）
   ```

3. **启用 TCP/IP 连接**
   ```powershell
   # 打开 SQL Server Configuration Manager
   # SQL Server Network Configuration > Protocols for SQLEXPRESS
   # 启用 TCP/IP
   # 重启 SQL Server 服务
   ```

4. **创建数据库和用户**
   ```sql
   -- 使用 SSMS (SQL Server Management Studio) 连接到服务器

   -- 创建数据库
   CREATE DATABASE classtop;
   GO

   -- 创建登录用户
   CREATE LOGIN classtop_user WITH PASSWORD = 'YourStrongPassword123!';
   GO

   -- 切换到 classtop 数据库
   USE classtop;
   GO

   -- 创建数据库用户并授权
   CREATE USER classtop_user FOR LOGIN classtop_user;
   GO

   ALTER ROLE db_owner ADD MEMBER classtop_user;
   GO
   ```

#### 方式 2: SQL Server Developer Edition (免费，仅用于开发)

类似于 Express，但功能更完整。下载地址相同。

#### 方式 3: SQL Server Standard/Enterprise (生产环境)

需要购买许可证。安装步骤类似。

---

### Linux (Docker)

使用 Docker 运行 SQL Server：

```bash
# 拉取 SQL Server 2022 镜像
docker pull mcr.microsoft.com/mssql/server:2022-latest

# 运行 SQL Server 容器
docker run -e "ACCEPT_EULA=Y" \
  -e "MSSQL_SA_PASSWORD=YourStrongPassword123!" \
  -p 1433:1433 \
  --name sqlserver \
  --hostname sqlserver \
  -d mcr.microsoft.com/mssql/server:2022-latest

# 检查容器状态
docker ps

# 连接到 SQL Server 创建数据库
docker exec -it sqlserver /opt/mssql-tools/bin/sqlcmd \
  -S localhost -U SA -P 'YourStrongPassword123!'

# 在 sqlcmd 中执行
CREATE DATABASE classtop;
GO
EXIT
```

---

### macOS (Docker)

macOS 不支持原生 SQL Server，但可以使用 Docker（仅限 Intel Mac，Apple Silicon 不支持）：

```bash
# 使用 Docker Desktop for Mac
# 拉取并运行 SQL Server（仅 Intel Mac）
docker run -e "ACCEPT_EULA=Y" \
  -e "MSSQL_SA_PASSWORD=YourStrongPassword123!" \
  -p 1433:1433 \
  --name sqlserver \
  -d mcr.microsoft.com/mssql/server:2022-latest

# 如果是 Apple Silicon Mac，建议使用 PostgreSQL
```

**注意**: 对于 Apple Silicon (M1/M2/M3) Mac，SQL Server 镜像不兼容。建议使用 PostgreSQL。

---

## ⚙️ 配置 ClassTop Management Server

### 1. 环境变量配置

编辑 `.env` 文件：

```env
# SQL Server 配置

# 方式 1: 简单格式（推荐用于本地开发）
DATABASE_URL=mssql://username:password@localhost:1433/classtop

# 方式 2: 完整格式（推荐用于生产环境）
DATABASE_URL=sqlserver://classtop_user:YourStrongPassword123!@localhost:1433;database=classtop;TrustServerCertificate=true

# 数据库类型（可选，会自动检测）
DB_TYPE=mssql

# 服务器配置
HOST=0.0.0.0
PORT=8765

# 应用配置
APP_VERSION=1.0.0
RUST_LOG=info
```

### 连接字符串格式说明

**基本格式**:
```
mssql://username:password@host:port/database
```

**完整格式（带参数）**:
```
sqlserver://username:password@host:port;database=dbname;TrustServerCertificate=true;Encrypt=true
```

**常用参数**:
| 参数 | 说明 | 默认值 |
|------|------|--------|
| `TrustServerCertificate` | 信任服务器证书（开发环境） | false |
| `Encrypt` | 启用加密连接 | true |
| `IntegratedSecurity` | 使用 Windows 身份验证 | false |
| `ConnectTimeout` | 连接超时（秒） | 30 |
| `ApplicationName` | 应用程序名称 | - |

### 2. Windows 身份验证（可选）

如果使用 Windows 身份验证（仅 Windows）：

```env
DATABASE_URL=sqlserver://localhost:1433;database=classtop;IntegratedSecurity=true;TrustServerCertificate=true
```

### 3. 远程 SQL Server

连接到远程服务器：

```env
# 使用主机名
DATABASE_URL=mssql://user:password@sqlserver.example.com:1433/classtop

# 使用 IP 地址
DATABASE_URL=mssql://user:password@192.168.1.100:1433/classtop

# 使用命名实例
DATABASE_URL=mssql://user:password@server\\INSTANCE:1433/classtop
```

---

## 🚀 运行服务器

### 首次启动

```bash
# 1. 确保 .env 配置正确
cat .env

# 2. 构建前端（如果需要）
cd frontend
npm install
npm run build
cd ..

# 3. 运行服务器（开发模式）
cargo run

# 或使用 release 模式
cargo run --release
```

### 启动日志示例

成功连接到 SQL Server 时，应该看到类似的日志：

```
[2025-10-09T12:00:00Z INFO  classtop_management_server] Starting ClassTop Management Server v1.0.0
[2025-10-09T12:00:00Z INFO  classtop_management_server] Database: MSSQL
[2025-10-09T12:00:01Z INFO  classtop_management_server] Running database migrations...
[2025-10-09T12:00:01Z INFO  classtop_management_server] Migrations completed successfully
[2025-10-09T12:00:01Z INFO  classtop_management_server] Server starting on http://0.0.0.0:8765
```

---

## ✅ 验证安装

### 1. 检查数据库连接

访问健康检查端点：

```bash
curl http://localhost:8765/api/health
```

成功响应：
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2025-10-09T12:00:00Z",
    "version": "1.0.0"
  }
}
```

### 2. 检查数据库表

使用 SSMS 或 sqlcmd 检查表是否创建：

```sql
USE classtop;
GO

-- 查看所有表
SELECT TABLE_NAME
FROM INFORMATION_SCHEMA.TABLES
WHERE TABLE_TYPE = 'BASE TABLE';
GO

-- 应该看到以下表：
-- clients
-- courses
-- schedule_entries
-- settings
-- sync_logs
```

### 3. 访问管理界面

打开浏览器访问：
- http://localhost:8765 - Web 管理界面
- http://localhost:8765/api/docs - API 文档

---

## 🔍 故障排查

### 问题 1: 无法连接到 SQL Server

**错误信息**: `Error: Network error` 或 `Connection refused`

**解决方法**:

1. **检查 SQL Server 是否运行**
   ```powershell
   # Windows
   Get-Service MSSQL*

   # 如果未运行，启动服务
   Start-Service MSSQL$SQLEXPRESS
   ```

2. **检查 TCP/IP 是否启用**
   - 打开 SQL Server Configuration Manager
   - 导航到: SQL Server Network Configuration > Protocols for [INSTANCE]
   - 确保 TCP/IP 已启用
   - 重启 SQL Server 服务

3. **检查防火墙规则**
   ```powershell
   # Windows Firewall - 允许端口 1433
   New-NetFirewallRule -DisplayName "SQL Server" -Direction Inbound -Protocol TCP -LocalPort 1433 -Action Allow
   ```

4. **验证端口**
   ```powershell
   # 检查 SQL Server 监听端口
   netstat -an | findstr 1433
   ```

---

### 问题 2: 身份验证失败

**错误信息**: `Login failed for user` 或 `Authentication failed`

**解决方法**:

1. **检查用户名和密码**
   - 确保 `.env` 中的凭据正确
   - 密码包含特殊字符时需要 URL 编码

2. **启用 SQL Server 身份验证模式**
   ```sql
   -- 在 SSMS 中，右键服务器 > 属性 > 安全性
   -- 选择 "SQL Server 和 Windows 身份验证模式"
   -- 重启 SQL Server 服务
   ```

3. **检查用户权限**
   ```sql
   USE classtop;
   GO

   -- 查看用户权限
   EXEC sp_helpuser 'classtop_user';
   GO

   -- 如果权限不足，重新授权
   ALTER ROLE db_owner ADD MEMBER classtop_user;
   GO
   ```

---

### 问题 3: 证书验证错误

**错误信息**: `Certificate verification failed`

**解决方法**:

在连接字符串中添加 `TrustServerCertificate=true`:

```env
DATABASE_URL=sqlserver://user:password@localhost:1433;database=classtop;TrustServerCertificate=true
```

**注意**: 仅在开发环境使用。生产环境应配置有效的 SSL 证书。

---

### 问题 4: 数据库不存在

**错误信息**: `Cannot open database "classtop"`

**解决方法**:

手动创建数据库：

```sql
CREATE DATABASE classtop;
GO

-- 验证数据库已创建
SELECT name FROM sys.databases WHERE name = 'classtop';
GO
```

---

### 问题 5: 迁移脚本执行失败

**错误信息**: `Migration failed` 或 `Object already exists`

**解决方法**:

1. **检查日志**
   ```bash
   # 使用 debug 日志级别
   RUST_LOG=debug cargo run
   ```

2. **手动运行迁移脚本**
   ```sql
   -- 在 SSMS 中打开并执行
   -- migrations/002_initial_mssql.sql
   ```

3. **重置数据库（仅开发环境）**
   ```sql
   USE master;
   GO

   DROP DATABASE classtop;
   GO

   CREATE DATABASE classtop;
   GO
   ```

---

## 🔒 生产环境最佳实践

### 1. 安全配置

- ✅ 使用强密码（至少 12 位，包含大小写字母、数字、特殊字符）
- ✅ 启用 SSL/TLS 加密连接
- ✅ 限制数据库用户权限（使用 db_datareader + db_datawriter 而非 db_owner）
- ✅ 配置防火墙规则，仅允许应用服务器访问数据库
- ✅ 定期更新 SQL Server 补丁

### 2. 性能优化

```sql
-- 创建推荐的索引（已包含在迁移脚本中）
-- 监控慢查询
-- 定期更新统计信息
UPDATE STATISTICS clients;
UPDATE STATISTICS courses;
UPDATE STATISTICS schedule_entries;
GO

-- 重建索引（根据需要）
ALTER INDEX ALL ON clients REBUILD;
GO
```

### 3. 备份策略

```sql
-- 完整备份
BACKUP DATABASE classtop
TO DISK = 'C:\Backups\classtop_full.bak'
WITH FORMAT;
GO

-- 差异备份
BACKUP DATABASE classtop
TO DISK = 'C:\Backups\classtop_diff.bak'
WITH DIFFERENTIAL;
GO

-- 事务日志备份
BACKUP LOG classtop
TO DISK = 'C:\Backups\classtop_log.trn';
GO
```

建议备份计划：
- 完整备份：每天一次（凌晨）
- 差异备份：每 6 小时一次
- 事务日志备份：每小时一次（如果使用完整恢复模式）

### 4. 监控

```sql
-- 查看当前连接数
SELECT
    DB_NAME(dbid) as DatabaseName,
    COUNT(dbid) as NumberOfConnections
FROM sys.sysprocesses
WHERE dbid > 0
GROUP BY dbid;
GO

-- 查看数据库大小
EXEC sp_spaceused;
GO

-- 查看活动查询
SELECT
    session_id,
    status,
    command,
    cpu_time,
    total_elapsed_time
FROM sys.dm_exec_requests
WHERE database_id = DB_ID('classtop');
GO
```

---

## 📊 性能对比

| 特性 | PostgreSQL | SQL Server |
|------|-----------|-----------|
| **开源** | ✅ 免费开源 | ❌ 需要许可证（Express 免费但有限制） |
| **跨平台** | ✅ 全平台支持 | ⚠️ Windows 原生，Linux 支持有限 |
| **性能** | 优秀 | 优秀 |
| **管理工具** | pgAdmin, CLI | SSMS (强大的 GUI) |
| **推荐场景** | Linux/macOS 服务器 | Windows Server 环境 |
| **最大数据库大小 (Express)** | 无限制 | 10 GB |

---

## 🔄 从 PostgreSQL 迁移到 SQL Server

如果需要从 PostgreSQL 迁移到 SQL Server：

### 1. 导出 PostgreSQL 数据

```bash
pg_dump -U username -d classtop > classtop_export.sql
```

### 2. 转换 SQL 语法

PostgreSQL 和 SQL Server 的 SQL 语法略有不同，需要手动调整：

| PostgreSQL | SQL Server |
|-----------|-----------|
| `SERIAL` | `IDENTITY(1,1)` |
| `VARCHAR` | `NVARCHAR` |
| `TEXT` | `NVARCHAR(MAX)` |
| `TIMESTAMP` | `DATETIME2` |
| `BOOLEAN` | `BIT` |

### 3. 导入到 SQL Server

使用 SSMS 的导入向导或手动执行修改后的 SQL。

### 4. 切换应用程序配置

更新 `.env` 文件为 SQL Server 连接字符串，重启应用。

---

## 🆘 获取帮助

如果遇到问题：

1. 查看应用日志：设置 `RUST_LOG=debug`
2. 查看 SQL Server 错误日志：
   - Windows: SQL Server Configuration Manager > SQL Server 服务 > 右键 > 属性 > 高级 > 错误日志目录
3. 提交 Issue: https://github.com/YOUR_REPO/issues

---

## 📚 相关文档

- [SQL Server 官方文档](https://docs.microsoft.com/sql/sql-server/)
- [SQLx 文档](https://docs.rs/sqlx/)
- [项目 README](../README.md)
- [客户端适配指南](./CLIENT_ADAPTATION.md)

---

**版本**: 1.0.0
**最后更新**: 2025-10-09
**维护者**: ClassTop Team
