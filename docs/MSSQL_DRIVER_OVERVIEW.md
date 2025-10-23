# SQL Server 驱动自研项目总览

## 📊 项目信息

| 项目 | 值 |
|------|-----|
| **项目名称** | mssql-driver |
| **版本** | 0.1.0 (开发中) |
| **开发周期** | 3-6 个月 (预估) |
| **当前阶段** | 阶段 0: 环境搭建 ✅ |
| **下一阶段** | 阶段 1: TDS 协议研究 |
| **完成度** | ~5% (骨架完成) |

## 📁 项目文件结构

```
Classtop-Management-Server/
├── docker-compose.mssql.yml          # SQL Server 测试环境
├── docker-mssql.sh                   # 环境管理脚本 (可执行)
├── docker/
│   └── mssql/
│       └── init.sql                  # 数据库初始化脚本
├── docs/
│   ├── MSSQL_DRIVER_DEVELOPMENT.md   # 完整开发计划 (17周路线图)
│   ├── MSSQL_DRIVER_QUICKSTART.md    # 快速启动指南 (本文档)
│   └── MSSQL_IMPLEMENTATION_APPROACHES.md  # 技术方案对比
└── mssql-driver/                     # 驱动子项目 ✅
    ├── Cargo.toml
    ├── README.md
    ├── src/
    │   ├── lib.rs
    │   ├── error.rs              ✅
    │   ├── connection/
    │   │   ├── mod.rs           🚧
    │   │   ├── config.rs        ✅
    │   │   └── pool.rs          🚧
    │   ├── protocol/
    │   │   ├── mod.rs           🚧
    │   │   ├── packets.rs       ✅
    │   │   └── tokens.rs        ✅
    │   ├── types.rs             ✅
    │   ├── transaction.rs       🚧
    │   └── utils/
    │       └── encoding.rs      ✅
    ├── examples/
    │   ├── simple_connect.rs
    │   └── query_test.rs
    └── tests/

✅ = 完成  🚧 = 占位/待实现  ❌ = 未开始
```

## 🎯 开发阶段

| 阶段 | 时间 | 状态 | 关键交付物 |
|------|------|------|-----------|
| **阶段 0** | Week 0 | ✅ 完成 | 环境搭建 + 项目骨架 |
| **阶段 1** | Week 1-2 | 📅 下一步 | TDS 协议文档研究 |
| **阶段 2** | Week 3-5 | ⏳ 待开始 | TCP 连接 + Pre-Login + Login7 |
| **阶段 3** | Week 6-9 | ⏳ 待开始 | SQL Batch + 结果集解析 |
| **阶段 4** | Week 10-12 | ⏳ 待开始 | 参数化查询 (sp_executesql) |
| **阶段 5** | Week 13-15 | ⏳ 待开始 | 连接池 + 事务支持 |
| **阶段 6** | Week 16-17 | ⏳ 待开始 | 集成到 Classtop Server |

## 🚀 快速命令

### Docker 环境管理

```bash
# 启动 SQL Server
./docker-mssql.sh start

# 停止 SQL Server
./docker-mssql.sh stop

# 查看状态
./docker-mssql.sh status

# 查看日志
./docker-mssql.sh logs

# 进入 SQL Shell
./docker-mssql.sh shell

# 执行查询
./docker-mssql.sh query "SELECT * FROM test_types"
```

### 驱动开发

```bash
cd mssql-driver

# 编译检查
cargo check

# 运行测试
cargo test

# 运行示例
cargo run --example simple_connect

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 生成文档
cargo doc --open
```

## 📋 当前 TODO

### 立即行动 (本周)

- [ ] 阅读 MS-TDS 协议文档（重点：章节 2.2.6）
- [ ] 研究 Tiberius 的 Pre-Login 实现
- [ ] 绘制 Pre-Login 数据结构图
- [ ] 实现 `PreLoginPacket` 结构体

### 短期目标 (2-3周)

- [ ] 实现 Pre-Login 包构造和发送
- [ ] 实现 Pre-Login 响应解析
- [ ] 实现 Login7 包构造
- [ ] 处理 LoginAck 和 EnvChange tokens
- [ ] 完成基础连接功能

### 中期目标 (1-2月)

- [ ] 实现 SQL Batch 执行
- [ ] 实现结果集解析 (ColMetaData, Row, Done tokens)
- [ ] 实现参数化查询 (RPC Request)
- [ ] 类型转换系统完善

### 长期目标 (3-6月)

- [ ] 连接池实现
- [ ] 事务支持
- [ ] 集成到 Classtop Management Server
- [ ] 完整测试覆盖

## 📖 关键资源

### 文档

| 资源 | URL | 用途 |
|------|-----|------|
| MS-TDS 协议 | [链接](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-tds) | 官方协议规范 |
| Tiberius 源码 | [链接](https://github.com/prisma/tiberius) | 参考实现 |
| Rust Async Book | [链接](https://rust-lang.github.io/async-book/) | Async/await 学习 |

### 本地文档

- `docs/MSSQL_DRIVER_DEVELOPMENT.md` - 完整开发计划
- `docs/MSSQL_DRIVER_QUICKSTART.md` - 快速启动指南
- `mssql-driver/README.md` - 驱动项目文档

## 🔍 开发环境验证

### ✅ 验证清单

- [x] Rust 工具链安装 (1.70+)
- [x] Docker 可用
- [x] SQL Server 容器配置
- [x] mssql-driver 项目创建
- [x] 项目可编译 (`cargo check` 通过)
- [x] 测试框架就绪
- [x] 文档完备

### 🧪 快速验证

```bash
# 1. 检查 Rust 版本
rustc --version

# 2. 检查 Docker
docker --version

# 3. 启动 SQL Server
./docker-mssql.sh start

# 4. 验证数据库
./docker-mssql.sh query "SELECT @@VERSION"

# 5. 编译驱动
cd mssql-driver && cargo check

# 6. 运行测试
cargo test
```

## 💡 技术决策记录

### 为什么选择自研？

1. **充足的开发周期** - 3-6 个月可用，不急于快速交付
2. **技术积累** - 深入理解 TDS 协议，掌握底层实现
3. **完全掌控** - 可以按需定制，避免第三方库限制
4. **学习价值** - 协议开发经验，Rust async 实践

### 为什么不用 Tiberius？

Tiberius 是优秀的参考实现，但我们选择自研是为了：
- 更深入的学习和技术沉淀
- 按项目需求定制功能
- 避免外部依赖的潜在问题
- 积累协议开发经验

**但我们会参考 Tiberius 的实现细节！**

### 技术栈选择

- **Tokio** - 成熟的 async 运行时
- **tokio-rustls** - 纯 Rust TLS 实现（避免 OpenSSL 依赖）
- **bytes** - 高效的字节缓冲区
- **thiserror** - 优雅的错误处理

## 📞 联系方式

有问题或需要帮助？

- 查看文档：`docs/MSSQL_DRIVER_*.md`
- 查看示例：`mssql-driver/examples/`
- 运行测试：`cargo test`

---

**最后更新**: 2024-01-XX (阶段 0 完成)

**下一里程碑**: 完成阶段 1 - TDS 协议研究 (预计 2 周)
