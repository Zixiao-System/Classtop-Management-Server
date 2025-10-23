# SQL Server 支持实现方案对比

## 方案对比表

| 方案 | 优点 | 缺点 | 难度 | 时间 | 适用场景 |
|------|------|------|------|------|---------|
| **1. 使用 Tiberius** | ✅ 纯 Rust 实现<br>✅ 异步支持<br>✅ 社区维护<br>✅ 无 C 依赖 | ❌ 需要双驱动代码<br>❌ 不能用 SQLx 宏 | ⭐⭐ 中等 | 2-3周 | **🎯 小公司首选** |
| **2. 使用 odbc crate** | ✅ 现成的 FFI 封装<br>✅ 跨数据库支持<br>✅ 相对稳定 | ❌ 需要 ODBC 驱动<br>❌ 同步 API<br>❌ 部署复杂 | ⭐⭐⭐ 较难 | 1-2周 | 已有 ODBC 环境 |
| **3. 自己写 C 绑定 + FFI** | ✅ 官方 ODBC 驱动<br>✅ 性能最优<br>✅ 功能完整<br>✅ 深度定制 | ❌ unsafe 维护难<br>❌ 跨平台复杂<br>❌ 工作量大 | ⭐⭐⭐⭐⭐ 极高 | 2-3月 | 学习项目/特殊需求 |

---

## 详细分析

### 方案 1: Tiberius (当前推荐)

**技术栈**:
```toml
tiberius = "0.12"
tokio-util = "0.7"
```

**优点详解**:
- 纯 Rust 实现，无 C 依赖
- 完整的 async/await 支持
- TDS (Tabular Data Stream) 协议实现
- 活跃维护（Prisma 团队）
- 类型安全

**缺点详解**:
- 需要维护 PostgreSQL (SQLx) 和 SQL Server (Tiberius) 两套代码
- 无法使用 SQLx 的编译时查询检查
- 需要手动实现数据库抽象层

**实现复杂度**: ⭐⭐ (中等)

**代码示例**:
```rust
// 需要创建抽象层
trait Database {
    async fn execute(&self, query: &str) -> Result<u64>;
    async fn fetch_one(&self, query: &str) -> Result<Row>;
    // ...
}

struct PostgresDb(PgPool);
struct MssqlDb(tiberius::Client);

impl Database for PostgresDb { /* ... */ }
impl Database for MssqlDb { /* ... */ }
```

---

### 方案 2: 自己写 C 绑定 + FFI (你的建议)

**技术栈**:
```toml
# Cargo.toml
[build-dependencies]
cc = "1.0"

[dependencies]
libc = "0.2"
```

**优点详解**:
- 使用官方 ODBC 驱动，功能最完整
- 性能最优（零开销抽象）
- 完全控制实现细节
- 可以直接调用 SQL Server 特有功能

**缺点详解**:
- 大量 `unsafe` 代码维护
- 跨平台难度大：
  - Windows: `odbc32.dll`
  - Linux: `unixODBC`
  - macOS: `iODBC` 或 `unixODBC`
- 内存管理复杂（C 字符串、缓冲区）
- 错误处理繁琐
- 需要处理 ODBC 驱动安装依赖
- 测试和调试困难

**实现复杂度**: ⭐⭐⭐⭐⭐ (极高)

**需要实现的内容**:

1. **C 绑定层** (build.rs + FFI):
```rust
// src/ffi/odbc.rs
use std::os::raw::{c_char, c_short, c_void};

#[repr(C)]
pub struct SQLHENV(*mut c_void);
#[repr(C)]
pub struct SQLHDBC(*mut c_void);
#[repr(C)]
pub struct SQLHSTMT(*mut c_void);

extern "C" {
    pub fn SQLAllocHandle(
        handle_type: c_short,
        input_handle: *mut c_void,
        output_handle: *mut *mut c_void,
    ) -> c_short;

    pub fn SQLConnect(
        connection_handle: SQLHDBC,
        server_name: *const c_char,
        name_length: c_short,
        user_name: *const c_char,
        user_length: c_short,
        password: *const c_char,
        pwd_length: c_short,
    ) -> c_short;

    pub fn SQLExecDirect(
        statement_handle: SQLHSTMT,
        statement_text: *const c_char,
        text_length: c_short,
    ) -> c_short;

    pub fn SQLFetch(statement_handle: SQLHSTMT) -> c_short;

    pub fn SQLGetData(
        statement_handle: SQLHSTMT,
        column_number: c_short,
        target_type: c_short,
        target_value: *mut c_void,
        buffer_length: c_short,
        str_len_or_ind: *mut c_short,
    ) -> c_short;

    pub fn SQLDisconnect(connection_handle: SQLHDBC) -> c_short;
    pub fn SQLFreeHandle(handle_type: c_short, handle: *mut c_void) -> c_short;
    // ... 还需要 50+ 个函数
}
```

2. **安全封装层**:
```rust
// src/mssql/connection.rs
pub struct Connection {
    env: SQLHENV,
    dbc: SQLHDBC,
}

impl Connection {
    pub async fn connect(dsn: &str) -> Result<Self> {
        unsafe {
            let mut env = std::ptr::null_mut();
            let mut dbc = std::ptr::null_mut();

            // 分配环境句柄
            check_error(SQLAllocHandle(SQL_HANDLE_ENV, std::ptr::null_mut(), &mut env))?;

            // 设置 ODBC 版本
            check_error(SQLSetEnvAttr(
                env,
                SQL_ATTR_ODBC_VERSION,
                SQL_OV_ODBC3 as *mut c_void,
                0,
            ))?;

            // 分配连接句柄
            check_error(SQLAllocHandle(SQL_HANDLE_DBC, env, &mut dbc))?;

            // 连接到数据库
            let dsn_cstr = CString::new(dsn)?;
            check_error(SQLConnect(
                dbc,
                dsn_cstr.as_ptr(),
                dsn.len() as c_short,
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
            ))?;

            Ok(Connection { env, dbc })
        }
    }

    pub async fn execute(&self, query: &str) -> Result<u64> {
        unsafe {
            let mut stmt = std::ptr::null_mut();
            check_error(SQLAllocHandle(SQL_HANDLE_STMT, self.dbc, &mut stmt))?;

            let query_cstr = CString::new(query)?;
            check_error(SQLExecDirect(
                stmt,
                query_cstr.as_ptr(),
                query.len() as c_short,
            ))?;

            let mut row_count: c_long = 0;
            check_error(SQLRowCount(stmt, &mut row_count))?;

            check_error(SQLFreeHandle(SQL_HANDLE_STMT, stmt))?;

            Ok(row_count as u64)
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            SQLDisconnect(self.dbc);
            SQLFreeHandle(SQL_HANDLE_DBC, self.dbc);
            SQLFreeHandle(SQL_HANDLE_ENV, self.env);
        }
    }
}
```

3. **类型转换层**:
```rust
// src/mssql/types.rs
pub trait FromSql: Sized {
    unsafe fn from_sql(
        stmt: SQLHSTMT,
        col: c_short,
    ) -> Result<Self>;
}

impl FromSql for String {
    unsafe fn from_sql(stmt: SQLHSTMT, col: c_short) -> Result<Self> {
        let mut buffer = vec![0u8; 4096];
        let mut indicator: c_short = 0;

        check_error(SQLGetData(
            stmt,
            col,
            SQL_C_CHAR,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as c_short,
            &mut indicator,
        ))?;

        if indicator == SQL_NULL_DATA {
            return Err(Error::NullValue);
        }

        let len = indicator as usize;
        buffer.truncate(len);
        String::from_utf8(buffer).map_err(Into::into)
    }
}

impl FromSql for i32 {
    unsafe fn from_sql(stmt: SQLHSTMT, col: c_short) -> Result<Self> {
        let mut value: i32 = 0;
        let mut indicator: c_short = 0;

        check_error(SQLGetData(
            stmt,
            col,
            SQL_C_LONG,
            &mut value as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>() as c_short,
            &mut indicator,
        ))?;

        if indicator == SQL_NULL_DATA {
            return Err(Error::NullValue);
        }

        Ok(value)
    }
}

// 还需要为以下类型实现:
// - i64, f32, f64, bool
// - Vec<u8> (binary)
// - chrono::DateTime
// - uuid::Uuid
// - Option<T>
```

4. **异步适配层** (因为 ODBC 是同步的):
```rust
// src/mssql/async_adapter.rs
pub struct AsyncConnection {
    inner: Arc<Mutex<Connection>>,
    executor: tokio::runtime::Handle,
}

impl AsyncConnection {
    pub async fn execute(&self, query: String) -> Result<u64> {
        let conn = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute(&query)
        })
        .await?
    }
}
```

5. **构建脚本** (build.rs):
```rust
fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=odbc32");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=odbc");
        // 可能需要检测 unixODBC 安装路径
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=iodbc");
    }
}
```

**工作量估算**:
- FFI 绑定: 500-800 行
- 安全封装: 1000-1500 行
- 类型转换: 500-800 行
- 测试代码: 1000+ 行
- **总计**: 3000-4000+ 行代码

**时间估算**: 2-3 个月（全职开发）

---

### 方案 3: 使用 odbc crate

**技术栈**:
```toml
odbc = "0.18"
odbc-safe = "0.5"
```

**优点详解**:
- FFI 封装已完成，直接使用
- 支持多种数据库（不只是 SQL Server）
- 相对稳定，有一定社区使用

**缺点详解**:
- 同步 API，需要包装成异步
- 需要安装 ODBC 驱动
- API 不如 Tiberius 优雅
- 错误处理不友好

**实现复杂度**: ⭐⭐⭐ (较难)

**代码示例**:
```rust
use odbc::*;

pub async fn connect(dsn: &str) -> Result<Connection> {
    tokio::task::spawn_blocking(move || {
        let env = Environment::new().unwrap();
        let conn = env.connect_with_connection_string(dsn).unwrap();
        Connection { env, conn }
    })
    .await?
}
```

---

---

## 推荐决策树（小公司视角）

```
是否需要 SQL Server 支持？
├─ 否 → 使用 PostgreSQL (当前状态) ✅
└─ 是 → 预算和时间如何？
    ├─ 紧张 (2-3周) → 方案 1: Tiberius ⭐⭐⭐⭐⭐ 强烈推荐
    │   └─ 理由：性价比最高，风险最低
    │
    ├─ 宽裕 (2-3月) 且想自主可控 → 方案 3: 自己写 FFI ⭐⭐⭐
    │   ├─ 优势：完全掌控，深度定制
    │   ├─ 风险：维护成本高，需要 unsafe 专家
    │   └─ 建议：先做技术验证（2周 POC）
    │
    └─ 已有 ODBC 环境 → 方案 2: odbc crate ⭐⭐
        └─ 适合：传统企业，已部署 ODBC 驱动
```

---

## 小公司实际建议

### 🎯 生产环境 (立即可用)
**强烈推荐**: 方案 1 - **Tiberius**

**小公司视角的理由**:
1. ⏰ **时间成本低** - 2-3周 vs 2-3月，节省人力成本
2. 💰 **维护成本低** - 纯 Rust，无 unsafe，bug 少
3. 🛡️ **风险可控** - Prisma 团队维护，稳定可靠
4. 📚 **文档齐全** - 降低学习成本
5. 🔧 **部署简单** - 无需安装 ODBC 驱动

**ROI 分析**:
- 开发成本：1 人 × 2-3 周
- 维护成本：低（每月 < 4 小时）
- 风险等级：低

### 🔬 技术储备 (长期投资)
**可考虑**: 方案 3 - **自己写 FFI**

**小公司视角的考虑**:
1. ✅ **技术积累** - 团队能力提升，构建核心竞争力
2. ✅ **深度定制** - 可针对特定业务优化
3. ✅ **自主可控** - 不依赖第三方库的更新节奏
4. ❌ **风险较高** - 需要有经验的 Rust/C 专家
5. ❌ **成本较高** - 2-3 月开发 + 持续维护

**适合的情况**:
- 有 1-2 名资深 Rust 开发者（熟悉 unsafe）
- 有充足预算和时间（不急于上线）
- 有特殊性能或功能需求（Tiberius 满足不了）
- 想构建技术壁垒

**不适合的情况**:
- 团队都是 Rust 新手
- 赶项目进度
- 预算紧张
- 人手不足

### ⚠️ 不推荐
**方案 2 - odbc crate**: 除非你们已经有 ODBC 环境

**理由**:
- 部署复杂（需要安装和配置 ODBC 驱动）
- 同步 API（需要额外包装）
- 文档较少（踩坑多）
- 没有比 Tiberius 明显的优势

---

## 关于自己写 FFI (方案 3) 的深入分析

### 💰 成本效益分析 (小公司视角)

**开发成本**:
```
人力投入：
- 1 名资深 Rust 开发 (熟悉 unsafe): 2-3 月全职
- 或 1 名普通 Rust 开发 + 导师辅导: 4-6 月

薪资成本 (以北京为例):
- 资深: ¥30K-50K/月 × 3 = ¥90K-150K
- 普通: ¥20K-30K/月 × 6 = ¥120K-180K

机会成本:
- 这期间无法开发其他功能
- 可能延误产品上线时间
```

**维护成本**:
```
持续投入：
- bug 修复: 平均每月 8-16 小时
- 新平台适配: 每次 2-4 周
- 安全更新: 不定期

5 年总成本估算: ¥200K-300K+
```

**对比 Tiberius**:
```
开发成本: ¥40K-60K (2-3周)
维护成本: ¥10K-20K/年 (基本无需维护)
5 年总成本: ¥90K-160K

节省: ¥110K-140K+ (约 58%)
```

### ✅ 值得尝试的场景

1. **技术壁垒需求**
   - 公司战略需要自主可控的数据库层
   - 构建核心技术竞争力
   - 未来可能商业化这个组件

2. **特殊性能要求**
   - Tiberius 无法满足的性能需求
   - 需要微秒级延迟优化
   - 高并发场景（>10K QPS）

3. **特殊功能需求**
   - SQL Server 独有功能（如 FileStream、Spatial Data）
   - 需要深度定制查询优化器
   - 特殊的连接池策略

4. **团队建设**
   - 有预算培养技术专家
   - 作为团队技术能力提升项目
   - 有成功案例可分享（技术营销）

### ❌ 不推荐的场景

1. **时间压力大**
   - 产品急于上线
   - 客户在等待交付
   - 市场窗口期短

2. **团队能力不足**
   - 缺少 unsafe Rust 专家
   - 缺少 C/ODBC 经验
   - 团队规模 < 5 人

3. **预算紧张**
   - 无法承受 3-6 月的开发成本
   - 无法投入持续维护资源
   - ROI 回收期 > 2 年

4. **非核心业务**
   - 数据库访问不是你们的核心竞争力
   - 只是作为基础设施使用
   - Tiberius 能满足需求

### 🎯 如果决定自己实现

**分阶段策略**（降低风险）:

**Phase 1: POC 验证 (2周)**
```
目标：验证技术可行性
投入：1 人 × 2 周
输出：能连接并执行简单查询的原型
决策：继续 or 放弃
```

**Phase 2: MVP 开发 (4-6周)**
```
目标：支持基本 CRUD 操作
投入：1 人 × 6 周
输出：可在单一平台（如 Windows）运行的版本
决策：继续完善 or 切换到 Tiberius
```

**Phase 3: 生产就绪 (4-6周)**
```
目标：跨平台、错误处理、性能优化
投入：1-2 人 × 6 周
输出：可用于生产环境的版本
```

**Phase 4: 持续维护**
```
目标：bug 修复、功能迭代
投入：每月 8-16 小时
```

**风险控制**:
1. 每个 Phase 结束后评估 ROI
2. 随时可以切换到 Tiberius（代码抽象层设计）
3. 从单平台开始，逐步扩展
4. 先内部使用，稳定后再考虑开源

---

## 🎓 结论与行动建议

### 对于 ClassTop Management Server 项目

**立即行动**: 采用 **Tiberius** (方案 1)

**理由**:
- ⏰ 快速上线：2-3 周即可投入生产
- 💰 成本可控：开发 + 5 年维护 < ¥160K
- 🛡️ 风险最低：社区维护，稳定可靠
- 👥 适合小团队：无需 unsafe 专家

**实施计划**:
```
Week 1-2: 创建数据库抽象层，集成 Tiberius
Week 3: 迁移所有数据库操作到抽象层
Week 4: 测试和优化
```

### 对于技术能力提升

**可选方案**: 并行开发 FFI 版本 (方案 3)

**前提条件**:
- ✅ 有 1-2 名资深 Rust 开发者
- ✅ 有 3-6 月的预算和时间
- ✅ 公司战略支持技术储备
- ✅ 通过了 POC 验证

**实施策略**:
1. 主线使用 Tiberius 确保业务稳定
2. 并行开发 FFI 版本作为技术储备
3. 设计良好的抽象层，方便切换
4. 分阶段推进，随时可以停止

### 小公司生存法则

**优先级排序**:
1. 🥇 **业务价值** - 能否快速交付客户需求？
2. 🥈 **成本控制** - ROI 是否合理？
3. 🥉 **技术风险** - 团队能否 hold 住？
4. 🏅 **长期维护** - 5 年后还能维护吗？

**避免的陷阱**:
- ❌ 过度工程（为了技术而技术）
- ❌ 重复造轮子（已有成熟方案）
- ❌ 忽视维护成本（只看开发成本）
- ❌ 低估复杂度（unsafe 坑很多）

**成功的关键**:
- ✅ 务实选择（Tiberius 够用就行）
- ✅ 迭代改进（先跑起来再优化）
- ✅ 团队能力（匹配项目难度）
- ✅ 商业思维（技术为业务服务）

---

## 📚 学习资源

**如果决定深入 FFI**:
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust 圣经
- [FFI 最佳实践](https://michael-f-bryan.github.io/rust-ffi-guide/)
- [ODBC API 参考](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/odbc-api-reference)
- [Tiberius 源码](https://github.com/prisma/tiberius) - 学习 TDS 协议实现

**Tiberius 入门**:
- [官方文档](https://docs.rs/tiberius/)
- [示例代码](https://github.com/prisma/tiberius/tree/main/examples)
- [性能测试](https://github.com/prisma/tiberius/tree/main/benches)

---

## 🤝 最后的话

作为小公司，**务实比完美更重要**。

- Tiberius 不是最快的，但足够快
- Tiberius 不是最灵活的，但足够用
- Tiberius 不是你写的，但它稳定可靠

把时间花在业务创新上，而不是重复造轮子。

**除非**你们真的有特殊需求、充足预算和专业团队，否则 Tiberius 是明智的选择。

如果未来真的需要自己实现，到那时团队更强、经验更多、决策也会更准确。

---

**Good luck! 🚀**