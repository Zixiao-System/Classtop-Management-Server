# mssql-driver

A pure Rust implementation of the Microsoft SQL Server TDS (Tabular Data Stream) protocol driver.

## 🚧 Development Status

**This is a work in progress!** Currently in early development phase (Phase 1-2 of 6).

### ✅ Completed
- Project structure and module skeleton
- Error handling framework
- Connection configuration system
- Basic TDS packet structures
- Character encoding utilities (UCS-2 LE)
- Type system definitions

### 🔨 In Progress
- TCP connection establishment
- TDS protocol implementation (Pre-Login, Login7)

### 📅 Planned
- Query execution and result set parsing
- Parameterized queries
- Connection pooling
- Transaction support
- Full type conversion system

See [Development Plan](../docs/MSSQL_DRIVER_DEVELOPMENT.md) for detailed roadmap.

## 📦 Features

- **Pure Rust**: No C dependencies, safe and memory-efficient
- **Async/Await**: Built on Tokio for high-performance async I/O
- **TLS Support**: Secure connections via tokio-rustls
- **Type Safe**: Strong typing with compile-time guarantees

## 🚀 Quick Start

### Prerequisites

1. Install Rust (1.70+)
2. Start SQL Server test environment:

```bash
cd ..
./docker-mssql.sh start
```

### Run Examples

```bash
# Simple connection test
cargo run --example simple_connect

# Query execution test (when implemented)
cargo run --example query_test
```

### Basic Usage

```rust
use mssql_driver::{Connection, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConnectionConfig::builder()
        .host("localhost")
        .port(1433)
        .username("sa")
        .password("YourPassword")
        .database("master")
        .build()?;

    let mut conn = Connection::connect(config).await?;
    let result = conn.query("SELECT 1 AS num").await?;

    println!("Rows: {}", result.rows.len());

    Ok(())
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture

# Run specific test module
cargo test connection::tests
```

## 📚 Architecture

```
mssql-driver/
├── src/
│   ├── connection/     # Connection management & pooling
│   ├── protocol/       # TDS protocol implementation
│   ├── types/          # Type conversion system
│   ├── utils/          # Encoding and utilities
│   ├── error.rs        # Error types
│   └── transaction.rs  # Transaction support
├── examples/           # Example programs
└── tests/              # Integration tests
```

## 🔬 Development

### Build

```bash
cargo build
cargo build --release
```

### Code Quality

```bash
# Format code
cargo fmt

# Check lints
cargo clippy -- -D warnings

# Check without building
cargo check
```

### Documentation

```bash
# Generate and open docs
cargo doc --open
```

## 📖 TDS Protocol Resources

- [MS-TDS Protocol Specification](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-tds)
- [Tiberius Reference Implementation](https://github.com/prisma/tiberius)
- [FreeTDS Project](https://www.freetds.org/)

## 🤝 Contributing

This driver is developed as part of ClassTop Management Server. Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- Tiberius team for protocol insights
- Microsoft for TDS protocol documentation
- Rust async ecosystem (Tokio, rustls)

---

**Note**: This is a learning/production project combining educational goals with real-world usage. The implementation prioritizes correctness and maintainability over raw performance.
