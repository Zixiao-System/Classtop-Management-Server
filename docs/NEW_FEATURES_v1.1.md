# 新功能说明（v1.1.0）

本文档记录了 ClassTop Management Server v1.1.0 版本中新增的功能和改进。

## 🔐 用户认证系统

### 功能概述

实现了基于 JWT (JSON Web Token) 的用户认证系统，提供安全的 API 访问控制。

### 核心特性

- ✅ JWT Token 认证
- ✅ 密码 bcrypt 哈希存储
- ✅ 用户注册和登录
- ✅ Token 自动过期（24小时）
- ✅ 可配置的认证开关

### API 端点

#### 用户注册
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "admin",
  "password": "secure_password",
  "email": "admin@example.com"
}
```

**响应：**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "uuid": "550e8400-e29b-41d4-a716-446655440000",
      "username": "admin",
      "email": "admin@example.com",
      "role": "user"
    }
  }
}
```

#### 用户登录
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "secure_password"
}
```

**响应格式同注册**

### 使用 Token

在需要认证的请求中添加 Authorization header：

```http
GET /api/clients
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### 配置

在 `.env` 文件中配置：

```env
# JWT 密钥（生产环境必须更改！）
JWT_SECRET=your-secret-key-change-this-in-production

# 是否启用认证（开发时可设为 false）
ENABLE_AUTH=true
```

**生成安全密钥：**
```bash
openssl rand -base64 32
```

## 📄 分页功能

### 功能概述

为列表类 API 添加了分页支持，提高大数据量时的性能和用户体验。

### 支持分页的端点

#### 客户端列表（分页）
```http
GET /api/clients/paginated?page=1&page_size=20
```

#### 课程列表（分页）
```http
GET /api/courses/paginated?page=1&page_size=20
```

### 查询参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `page` | integer | 1 | 页码（从 1 开始） |
| `page_size` | integer | 20 | 每页数量 |

### 响应格式

```json
{
  "success": true,
  "data": {
    "data": [ ... ],
    "pagination": {
      "page": 1,
      "page_size": 20,
      "total_items": 150,
      "total_pages": 8
    }
  }
}
```

## 📊 结构化日志

### 功能概述

使用 `tracing` 和 `tracing-subscriber` 替代了原有的 `env_logger`，提供更强大的结构化日志功能。

### 特性

- ✅ 结构化字段日志
- ✅ 可配置的日志级别
- ✅ 请求追踪
- ✅ 性能监控支持

### 配置

在 `.env` 中设置日志级别：

```env
# 日志级别: trace, debug, info, warn, error
RUST_LOG=info,actix_web=debug,sqlx=warn
```

### 日志示例

```
2024-01-01T10:00:00.000Z  INFO classtop_management_server: Starting ClassTop Management Server version=1.1.0
2024-01-01T10:00:01.000Z  INFO classtop_management_server: Authentication configuration loaded auth_enabled=true
2024-01-01T10:00:02.000Z  INFO classtop_management_server: Server starting address=0.0.0.0:8765
```

## 🔒 CORS 安全配置

### 功能概述

实现了基于环境的 CORS 配置，生产环境使用严格的白名单，开发环境保持宽松。

### 配置方式

在 `.env` 中设置允许的源：

```env
# 多个源用逗号分隔
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:8765,https://yourdomain.com
```

### 行为

- **ENABLE_AUTH=true**: 使用严格的 CORS 配置，只允许列表中的源
- **ENABLE_AUTH=false**: 开发模式，允许所有源（仅用于开发）

## ⚡ API 限流

### 功能概述

使用 `actix-governor` 实现了 API 限流，防止滥用和 DoS 攻击。

### 限流策略

- **速率**: 每秒 2 个请求
- **突发**: 允许突发 100 个请求
- **范围**: 全局（所有端点）

### 超限响应

当请求超过限制时，返回 `429 Too Many Requests`：

```json
{
  "error": "Too many requests"
}
```

## 🗄️ 数据库改进

### 新增表

#### users 表
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(36) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 迁移

数据库迁移自动在启动时运行。新的迁移文件：
- `migrations/005_add_user_auth.sql`

## 🧪 测试支持

### 新增测试

添加了单元测试和集成测试：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_password_hashing
cargo test test_jwt_token_generation
```

### 测试覆盖

- ✅ 认证模块（密码哈希、JWT 生成/验证）
- ✅ 分页逻辑
- ✅ 模型转换
- ✅ API 端点（健康检查、根路径）

## 📚 API 文档更新

Swagger UI 已更新，包含所有新端点的文档。

访问：`http://localhost:8765/api/docs`

### 新增的 API 标签

- **Authentication**: 用户认证相关端点
- 更新了所有现有端点的文档

## 🚀 升级指南

### 从 v1.0.0 升级到 v1.1.0

1. **更新依赖**
   ```bash
   cargo update
   ```

2. **更新环境配置**

   在 `.env` 文件中添加新的配置项：
   ```env
   JWT_SECRET=your-generated-secret-key
   ENABLE_AUTH=true
   CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:8765
   ```

3. **数据库迁移**

   迁移会自动运行。如果需要手动运行：
   ```bash
   psql -U username -d classtop -f migrations/005_add_user_auth.sql
   ```

4. **创建管理员账户**

   ```bash
   curl -X POST http://localhost:8765/api/auth/register \
     -H "Content-Type: application/json" \
     -d '{
       "username": "admin",
       "password": "your_password",
       "email": "admin@example.com"
     }'
   ```

5. **更新客户端代码**

   如果使用了 `/api/clients` 或 `/api/courses`，考虑迁移到分页版本以提高性能。

## ⚠️ 重要提示

### 安全建议

1. **生产环境必须更改 JWT_SECRET**
   ```bash
   openssl rand -base64 32
   ```

2. **启用认证**
   ```env
   ENABLE_AUTH=true
   ```

3. **配置严格的 CORS**
   ```env
   CORS_ALLOWED_ORIGINS=https://yourdomain.com
   ```

4. **使用 HTTPS**

   生产环境务必使用 HTTPS，保护 JWT token 和用户凭证。

### 性能建议

1. **使用分页端点**处理大量数据
2. **适当调整数据库连接池大小**（默认 10）
3. **根据负载调整 API 限流参数**

## 🔄 向后兼容性

- ✅ 所有现有 API 端点保持不变
- ✅ 可通过 `ENABLE_AUTH=false` 禁用认证（仅开发环境）
- ✅ 非分页端点仍然可用

## 📝 已知限制

1. 当前不支持：
   - 刷新 token
   - 用户权限细粒度控制
   - OAuth2 / 第三方登录
   - 用户管理 API（删除、更新等）

2. 这些功能计划在未来版本中添加

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

与主项目相同
