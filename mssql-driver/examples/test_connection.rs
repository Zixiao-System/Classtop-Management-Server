//! Test connection to SQL Server
//!
//! Usage: RUST_LOG=info cargo run --example test_connection

use mssql_driver::{Connection, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Testing SQL Server Connection ===\n");

    // Build configuration
    let config = ConnectionConfig::builder()
        .host("localhost")
        .port(1433)
        .username("sa")
        .password("ClassTop@2024Dev!")
        .database("master")
        .encrypt(false) // Azure SQL Edge doesn't require encryption
        .build()?;

    println!("Connecting to SQL Server at {}:{}...", config.host, config.port);
    println!("Database: {}", config.database);
    println!("Username: {}\n", config.username);

    // Attempt connection
    match Connection::connect(config).await {
        Ok(_conn) => {
            println!("\n✓ Connection successful!");
            println!("✓ Authentication completed");
            println!("✓ Database connection established");
            Ok(())
        }
        Err(e) => {
            eprintln!("\n✗ Connection failed: {}", e);
            eprintln!("\nMake sure SQL Server is running:");
            eprintln!("  docker compose -f docker-compose.mssql.yml up -d");
            Err(e.into())
        }
    }
}
