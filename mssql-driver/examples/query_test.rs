//! Query execution example

use mssql_driver::{Connection, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 mssql-driver Query Test Example\n");

    let config = ConnectionConfig::builder()
        .host("localhost")
        .port(1433)
        .username("sa")
        .password("ClassTop@2024Dev!")
        .database("classtop_dev")
        .encrypt(false)
        .trust_server_certificate(true)
        .build()?;

    println!("📡 Connecting to database...");

    let mut conn = Connection::connect(config).await?;
    println!("✅ Connected\n");

    // Test simple SELECT
    println!("🔍 Executing: SELECT 1 AS num, 'hello' AS msg");
    match conn.query("SELECT 1 AS num, 'hello' AS msg").await {
        Ok(result) => {
            println!("✅ Query successful!");
            println!("   Columns: {}", result.columns.len());
            println!("   Rows: {}", result.rows.len());
            println!("   Rows affected: {}", result.rows_affected);
        }
        Err(e) => {
            eprintln!("⚠️  Query failed (expected - not implemented yet): {}", e);
        }
    }

    conn.close().await?;
    println!("\n🔌 Connection closed");

    Ok(())
}
