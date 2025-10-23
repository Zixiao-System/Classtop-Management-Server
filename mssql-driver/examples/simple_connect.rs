//! Simple connection example

use mssql_driver::{Connection, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 mssql-driver Simple Connection Example\n");

    let config = ConnectionConfig::builder()
        .host("localhost")
        .port(1433)
        .username("sa")
        .password("ClassTop@2024Dev!")
        .database("classtop_dev")
        .encrypt(false) // Disable for local development
        .trust_server_certificate(true)
        .build()?;

    println!("📡 Connecting to {}:{}/{}", config.host, config.port, config.database);

    match Connection::connect(config).await {
        Ok(conn) => {
            println!("✅ Connection successful!");
            println!("   Connection is alive: {}", conn.is_alive());

            // Close connection
            conn.close().await?;
            println!("🔌 Connection closed");
        }
        Err(e) => {
            eprintln!("❌ Connection failed: {}", e);
            eprintln!("\n💡 Make sure SQL Server is running:");
            eprintln!("   ./docker-mssql.sh start");
            return Err(e.into());
        }
    }

    Ok(())
}
