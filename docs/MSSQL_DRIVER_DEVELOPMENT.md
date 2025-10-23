# SQL Server 驱动自研开发计划

## 项目概述

从零开始实现一个纯 Rust 的 SQL Server 驱动，支持 ClassTop Management Server 的所有数据库操作需求。

**开发周期估算**: 3-6 个月
**技术难度**: ⭐⭐⭐⭐⭐ (高难度)
**技术收益**: 完全掌控底层实现，积累协议开发经验

---

## 技术架构

### 模块划分

```
mssql-driver/
├── src/
│   ├── lib.rs              # 驱动入口，暴露公开 API
│   ├── connection/         # 连接管理
│   │   ├── mod.rs          # 连接抽象
│   │   ├── pool.rs         # 连接池实现
│   │   └── config.rs       # 连接配置
│   ├── protocol/           # TDS 协议实现
│   │   ├── mod.rs          # 协议抽象
│   │   ├── packets.rs      # 包结构定义
│   │   ├── login.rs        # 登录流程
│   │   ├── query.rs        # 查询执行
│   │   └── result.rs       # 结果集解析
│   ├── types/              # 类型系统
│   │   ├── mod.rs          # 类型转换接口
│   │   ├── rust_to_sql.rs  # Rust → SQL Server
│   │   └── sql_to_rust.rs  # SQL Server → Rust
│   ├── transaction.rs      # 事务支持
│   ├── error.rs            # 错误处理
│   └── utils/              # 工具函数
│       ├── mod.rs
│       └── encoding.rs     # 字符编码处理
└── tests/
    ├── integration_tests.rs
    └── protocol_tests.rs
```

---

## 开发阶段

### 阶段 1: TDS 协议研究 (1-2 周)

**目标**: 深入理解 TDS 7.x/8.0 协议

**任务清单**:
- [ ] 阅读 [MS-TDS] 官方协议文档 (https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-tds)
- [ ] 研究 Tiberius 源码作为参考 (https://github.com/prisma/tiberius)
- [ ] 绘制协议交互流程图
- [ ] 定义核心数据结构

**关键协议要点**:
- **Pre-Login**: 协商 TLS/加密参数
- **Login7**: 发送认证信息 (用户名、密码、数据库)
- **SQL Batch**: 执行 SQL 语句
- **RPC Request**: 执行存储过程
- **Token Stream**: 解析结果集 (COLMETADATA, ROW, DONE)

**输出物**:
- 协议交互时序图 (Mermaid/PlantUML)
- 包结构 Rust 定义 (struct Packet, enum TokenType)

---

### 阶段 2: 基础连接实现 (2-3 周)

**目标**: 实现 TCP 连接、Pre-Login、Login7

**核心代码框架**:

```rust
// src/connection/mod.rs
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

pub struct Connection {
    stream: TcpStream,
    config: ConnectionConfig,
}

impl Connection {
    pub async fn connect(config: ConnectionConfig) -> Result<Self> {
        // 1. TCP 连接到 host:port
        let stream = TcpStream::connect(&config.addr).await?;

        // 2. 发送 Pre-Login 包
        send_prelogin(&stream, &config).await?;
        let prelogin_response = recv_prelogin(&stream).await?;

        // 3. 如果需要 TLS，升级连接
        let stream = if prelogin_response.encryption_required {
            upgrade_to_tls(stream, &config).await?
        } else {
            stream
        };

        // 4. 发送 Login7 包
        send_login7(&stream, &config).await?;
        let login_response = recv_login_response(&stream).await?;

        if !login_response.success {
            return Err(Error::AuthFailed);
        }

        Ok(Self { stream, config })
    }
}
```

**测试验证**:
```rust
#[tokio::test]
async fn test_basic_connection() {
    let config = ConnectionConfig {
        host: "localhost".to_string(),
        port: 1433,
        username: "sa".to_string(),
        password: "YourPassword".to_string(),
        database: "master".to_string(),
    };

    let conn = Connection::connect(config).await.unwrap();
    assert!(conn.is_alive());
}
```

---

### 阶段 3: 查询执行 (3-4 周)

**目标**: 实现 SQL Batch 执行和结果集解析

**核心功能**:

1. **SQL Batch 包构造**:
```rust
// src/protocol/query.rs
pub async fn execute_query(
    conn: &mut Connection,
    sql: &str,
) -> Result<QueryResult> {
    // 1. 构造 SQL_BATCH 包
    let packet = Packet {
        packet_type: PacketType::SqlBatch,
        status: 0x01, // EOM (End of Message)
        length: calculate_length(sql),
        headers: vec![],
        data: encode_sql_batch(sql),
    };

    // 2. 发送到服务器
    send_packet(conn, packet).await?;

    // 3. 接收响应流
    let tokens = recv_token_stream(conn).await?;

    // 4. 解析结果集
    parse_result_set(tokens)
}

fn encode_sql_batch(sql: &str) -> Vec<u8> {
    // TDS 使用 UCS-2 LE 编码
    sql.encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect()
}
```

2. **Token Stream 解析**:
```rust
// src/protocol/result.rs
pub fn parse_result_set(tokens: Vec<Token>) -> Result<QueryResult> {
    let mut columns = Vec::new();
    let mut rows = Vec::new();

    for token in tokens {
        match token {
            Token::ColMetaData(col_info) => {
                columns = col_info.columns;
            }
            Token::Row(row_data) => {
                let row = parse_row(&columns, row_data)?;
                rows.push(row);
            }
            Token::Done(done_token) => {
                return Ok(QueryResult {
                    columns,
                    rows,
                    rows_affected: done_token.row_count,
                });
            }
            Token::Error(err) => {
                return Err(Error::ServerError(err.message));
            }
            _ => {} // 忽略其他 token
        }
    }

    Err(Error::IncompleteResponse)
}
```

**测试用例**:
```rust
#[tokio::test]
async fn test_simple_select() {
    let mut conn = connect_test_db().await;

    let result = execute_query(&mut conn, "SELECT 1 AS num, 'hello' AS str").await.unwrap();

    assert_eq!(result.columns.len(), 2);
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0].get::<i32>("num"), Some(1));
    assert_eq!(result.rows[0].get::<String>("str"), Some("hello".to_string()));
}
```

---

### 阶段 4: 参数化查询 (2-3 周)

**目标**: 实现 RPC Request (sp_executesql) 支持参数绑定

**核心实现**:
```rust
// src/protocol/query.rs
pub async fn execute_prepared(
    conn: &mut Connection,
    sql: &str,
    params: Vec<Parameter>,
) -> Result<QueryResult> {
    // 1. 构造 RPC 请求调用 sp_executesql
    let rpc = RpcRequest {
        proc_name: "sp_executesql".to_string(),
        params: vec![
            Parameter::NVarChar(sql.to_string()),  // @stmt
            Parameter::NVarChar(build_param_def(&params)), // @params
        ]
        .into_iter()
        .chain(params)
        .collect(),
    };

    // 2. 编码并发送
    let packet = encode_rpc_request(rpc)?;
    send_packet(conn, packet).await?;

    // 3. 接收结果
    let tokens = recv_token_stream(conn).await?;
    parse_result_set(tokens)
}

fn build_param_def(params: &[Parameter]) -> String {
    params.iter()
        .enumerate()
        .map(|(i, p)| format!("@P{} {}", i + 1, p.sql_type()))
        .collect::<Vec<_>>()
        .join(", ")
}
```

**类型转换**:
```rust
// src/types/rust_to_sql.rs
pub enum Parameter {
    Int(i32),
    BigInt(i64),
    NVarChar(String),
    DateTime(chrono::NaiveDateTime),
    UniqueIdentifier(uuid::Uuid),
    // ... 更多类型
}

impl Parameter {
    pub fn sql_type(&self) -> &'static str {
        match self {
            Parameter::Int(_) => "INT",
            Parameter::BigInt(_) => "BIGINT",
            Parameter::NVarChar(_) => "NVARCHAR(MAX)",
            Parameter::DateTime(_) => "DATETIME2",
            Parameter::UniqueIdentifier(_) => "UNIQUEIDENTIFIER",
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Parameter::Int(val) => val.to_le_bytes().to_vec(),
            Parameter::NVarChar(s) => {
                let utf16: Vec<u8> = s.encode_utf16()
                    .flat_map(|c| c.to_le_bytes())
                    .collect();

                let len = (utf16.len() as u16).to_le_bytes();
                [&len[..], &utf16[..]].concat()
            }
            // ... 其他类型
        }
    }
}
```

---

### 阶段 5: 连接池与事务 (2-3 周)

**目标**: 实现生产级特性

**连接池**:
```rust
// src/connection/pool.rs
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct ConnectionPool {
    connections: Arc<Mutex<Vec<Connection>>>,
    semaphore: Arc<Semaphore>,
    config: ConnectionConfig,
    max_size: usize,
}

impl ConnectionPool {
    pub async fn new(config: ConnectionConfig, max_size: usize) -> Result<Self> {
        let mut connections = Vec::with_capacity(max_size);

        // 预创建连接
        for _ in 0..max_size {
            connections.push(Connection::connect(config.clone()).await?);
        }

        Ok(Self {
            connections: Arc::new(Mutex::new(connections)),
            semaphore: Arc::new(Semaphore::new(max_size)),
            config,
            max_size,
        })
    }

    pub async fn acquire(&self) -> Result<PooledConnection> {
        let permit = self.semaphore.acquire().await?;

        let conn = {
            let mut conns = self.connections.lock().await;
            conns.pop().ok_or(Error::PoolExhausted)?
        };

        Ok(PooledConnection {
            conn: Some(conn),
            pool: self.clone(),
            _permit: permit,
        })
    }
}

pub struct PooledConnection {
    conn: Option<Connection>,
    pool: ConnectionPool,
    _permit: SemaphorePermit<'static>,
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let pool = self.pool.clone();
            tokio::spawn(async move {
                pool.connections.lock().await.push(conn);
            });
        }
    }
}
```

**事务支持**:
```rust
// src/transaction.rs
pub struct Transaction<'conn> {
    conn: &'conn mut Connection,
    committed: bool,
}

impl<'conn> Transaction<'conn> {
    pub async fn begin(conn: &'conn mut Connection) -> Result<Self> {
        execute_query(conn, "BEGIN TRANSACTION").await?;
        Ok(Self {
            conn,
            committed: false,
        })
    }

    pub async fn commit(mut self) -> Result<()> {
        execute_query(self.conn, "COMMIT TRANSACTION").await?;
        self.committed = true;
        Ok(())
    }

    pub async fn rollback(mut self) -> Result<()> {
        execute_query(self.conn, "ROLLBACK TRANSACTION").await?;
        self.committed = true;
        Ok(())
    }
}

impl<'conn> Drop for Transaction<'conn> {
    fn drop(&mut self) {
        if !self.committed {
            // 自动回滚
            tokio::spawn(async move {
                let _ = execute_query(self.conn, "ROLLBACK TRANSACTION").await;
            });
        }
    }
}
```

---

### 阶段 6: 集成到 Classtop (2 周)

**目标**: 替换现有的 PostgreSQL 驱动

**统一接口**:
```rust
// src/db/mod.rs
pub enum DbPool {
    PostgreSQL(sqlx::Pool<sqlx::Postgres>),
    MSSQL(mssql_driver::ConnectionPool),
}

impl DbPool {
    pub async fn from_config(config: &Config) -> Result<Self> {
        match config.database_type {
            DatabaseType::PostgreSQL => {
                let pool = sqlx::PgPool::connect(&config.database_url).await?;
                Ok(DbPool::PostgreSQL(pool))
            }
            DatabaseType::MSSQL => {
                let pool = mssql_driver::ConnectionPool::new(
                    parse_mssql_config(&config.database_url)?,
                    10, // max_connections
                ).await?;
                Ok(DbPool::MSSQL(pool))
            }
        }
    }

    pub async fn execute(&self, sql: &str, params: Vec<Parameter>) -> Result<QueryResult> {
        match self {
            DbPool::PostgreSQL(pool) => {
                // SQLx 查询
                let query = sqlx::query(sql);
                // ... 绑定参数并执行
            }
            DbPool::MSSQL(pool) => {
                let mut conn = pool.acquire().await?;
                mssql_driver::execute_prepared(&mut conn, sql, params).await
            }
        }
    }
}
```

---

## 测试策略

### 单元测试
- 每个模块独立测试 (protocol, types, connection)
- 使用 mock 测试协议包构造和解析

### 集成测试
- Docker Compose 启动 SQL Server 实例
- 运行完整的 CRUD 测试套件
- 压力测试 (连接池、并发查询)

### 测试覆盖率目标
- 核心协议层: 90%+
- 类型转换层: 95%+
- 连接管理层: 85%+

---

## 风险与挑战

### 技术风险
1. **TDS 协议复杂度**: 协议文档长达 500+ 页，细节繁多
   - **缓解**: 先实现 MVP (最小可行功能)，逐步增加特性

2. **字符编码问题**: UCS-2 LE、UTF-8、Collation 处理
   - **缓解**: 使用 Rust encoding_rs 库，充分测试

3. **TLS/加密处理**: Pre-Login 协商加密参数
   - **缓解**: 使用 tokio-rustls，参考 Tiberius 实现

4. **性能优化**: 包解析、内存分配开销
   - **缓解**: 后期使用 `bytes::BytesMut` 零拷贝优化

### 资源风险
1. **开发时间**: 可能超出预估 (3-6 月 → 6-9 月)
   - **缓解**: 分阶段交付，优先保证核心功能

2. **维护成本**: 协议可能升级，需要跟进
   - **缓解**: 模块化设计，降低修改影响范围

---

## 里程碑

| 阶段 | 时间 | 交付物 | 验收标准 |
|------|------|--------|---------|
| 阶段 1 | Week 1-2 | 协议文档 + 架构设计 | 通过技术评审 |
| 阶段 2 | Week 3-5 | 基础连接 | 成功连接并登录 SQL Server |
| 阶段 3 | Week 6-9 | 查询执行 | 执行 SELECT 并解析结果集 |
| 阶段 4 | Week 10-12 | 参数化查询 | 支持 INSERT/UPDATE with params |
| 阶段 5 | Week 13-15 | 连接池 + 事务 | 通过并发压力测试 |
| 阶段 6 | Week 16-17 | 集成到 Classtop | 完整迁移脚本运行成功 |

---

## 参考资料

- [MS-TDS 协议文档](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-tds)
- [Tiberius 源码](https://github.com/prisma/tiberius)
- [FreeTDS 项目](https://www.freetds.org/) (C 实现参考)
- [Rust async book](https://rust-lang.github.io/async-book/)

---

## 下一步行动

✅ 创建 `mssql-driver` 子项目
✅ 搭建开发环境 (Docker SQL Server)
✅ 开始阶段 1: 协议研究
