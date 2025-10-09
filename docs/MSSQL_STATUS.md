# SQL Server 支持说明

## ⚠️ 当前状态

由于 SQLx 0.8 版本不支持 Microsoft SQL Server，SQL Server 集成目前处于**规划阶段**。

## 📋 实现方案

### 方案 1: 使用 Tiberius (推荐)

使用 `tiberius` crate 作为 SQL Server 驱动：

**优点**:
- ✅ 专为 SQL Server 设计
- ✅ 异步支持（tokio）
- ✅ 活跃维护

**缺点**:
- ❌ 需要维护两套数据库访问代码
- ❌ 不能使用 SQLx 的编译时查询检查

### 方案 2: 等待 SQLx 官方支持

SQLx 计划在未来版本中添加 MSSQL 支持。

**优点**:
- ✅ 统一的数据库接口
- ✅ 编译时查询检查
- ✅ 更好的类型安全

**缺点**:
- ❌ 需要等待上游更新
- ❌ 时间不确定

### 方案 3: 使用多数据库抽象层

创建自己的数据库抽象层，支持 PostgreSQL (SQLx) 和 SQL Server (Tiberius)。

## 🔧 当前实现

项目已经准备好了 SQL Server 的基础设施：

1. ✅ **数据库迁移脚本** - `migrations/002_initial_mssql.sql`
2. ✅ **配置系统** - 支持数据库类型检测
3. ✅ **文档** - 完整的 SQL Server 配置指南
4. ⏳ **运行时支持** - 待实现

## 🚀 临时解决方案

在官方支持之前，推荐使用 PostgreSQL：

### Windows Server 上运行 PostgreSQL

```powershell
# 使用 Docker Desktop for Windows
docker run -d \
  --name postgres \
  -e POSTGRES_PASSWORD=yourpassword \
  -e POSTGRES_DB=classtop \
  -p 5432:5432 \
  postgres:14

# 或下载 Windows 安装包
# https://www.postgresql.org/download/windows/
```

### 为什么选择 PostgreSQL

- ✅ 完全开源免费
- ✅ 跨平台支持（Windows/Linux/macOS）
- ✅ 性能优秀
- ✅ 功能丰富
- ✅ SQLx 完美支持

## 📅 开发路线图

### 短期 (1-2 个月)

- [ ] 评估 Tiberius 集成可行性
- [ ] 创建数据库抽象层 trait
- [ ] 实现双驱动支持原型

### 中期 (3-6 个月)

- [ ] 完整实现 Tiberius 驱动
- [ ] 添加集成测试
- [ ] 性能基准测试

### 长期

- [ ] 跟踪 SQLx MSSQL 支持进度
- [ ] 迁移到 SQLx 统一接口（如果可用）

## 🤝 贡献

如果您有兴趣帮助实现 SQL Server 支持，欢迎：

1. 查看 [Tiberius 文档](https://docs.rs/tiberius/)
2. 参考现有的 PostgreSQL 实现
3. 提交 Pull Request

## 📚 相关资源

- [Tiberius GitHub](https://github.com/prisma/tiberius)
- [SQLx GitHub Issue - MSSQL Support](https://github.com/launchbadge/sqlx/issues/74)
- [本项目 SQL Server 配置指南](./MSSQL_SETUP.md)

---

**最后更新**: 2025-10-09
**状态**: 规划中
