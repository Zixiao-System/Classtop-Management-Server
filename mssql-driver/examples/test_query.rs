//! Test SQL query execution
//!
//! Usage: RUST_LOG=info cargo run --example test_query

use mssql_driver::{Connection, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Testing SQL Query Execution ===\n");

    // Build configuration
    let config = ConnectionConfig::builder()
        .host("localhost")
        .port(1433)
        .username("sa")
        .password("ClassTop@2024Dev!")
        .database("master")
        .encrypt(false)
        .build()?;

    println!("Connecting to SQL Server...");
    let mut conn = Connection::connect(config).await?;
    println!("✓ Connected\n");

    // Test 1: Simple SELECT
    println!("--- Test 1: SELECT 1 ---");
    match conn.query("SELECT 1 AS num").await {
        Ok(result) => {
            println!("✓ Query executed successfully");
            println!("  Columns: {}", result.columns.len());
            println!("  Rows: {}", result.rows.len());
            println!("  Rows affected: {}", result.rows_affected);

            for (i, col) in result.columns.iter().enumerate() {
                println!("  Column {}: {} ({:?})", i, col.name, col.data_type);
            }
        }
        Err(e) => {
            println!("✗ Query failed: {}", e);
        }
    }
    println!();

    // Test 2: SELECT with multiple columns
    println!("--- Test 2: SELECT multiple columns ---");
    match conn
        .query("SELECT 1 AS num, 'hello' AS msg, 3.14 AS pi")
        .await
    {
        Ok(result) => {
            println!("✓ Query executed successfully");
            println!("  Columns: {}", result.columns.len());
            println!("  Rows: {}", result.rows.len());

            for (i, col) in result.columns.iter().enumerate() {
                println!("  Column {}: {} ({:?})", i, col.name, col.data_type);
            }
        }
        Err(e) => {
            println!("✗ Query failed: {}", e);
        }
    }
    println!();

    // Test 3: SELECT @@VERSION
    println!("--- Test 3: SELECT @@VERSION ---");
    match conn.query("SELECT @@VERSION AS version").await {
        Ok(result) => {
            println!("✓ Query executed successfully");
            println!("  Columns: {}", result.columns.len());
            println!("  Rows: {}", result.rows.len());
        }
        Err(e) => {
            println!("✗ Query failed: {}", e);
        }
    }
    println!();

    // Test 4: CREATE TABLE
    println!("--- Test 4: CREATE TABLE ---");
    match conn
        .query(
            "IF OBJECT_ID('test_table', 'U') IS NOT NULL DROP TABLE test_table; \
             CREATE TABLE test_table (id INT, name NVARCHAR(50))",
        )
        .await
    {
        Ok(result) => {
            println!("✓ Table created successfully");
            println!("  Rows affected: {}", result.rows_affected);
        }
        Err(e) => {
            println!("✗ Failed to create table: {}", e);
        }
    }
    println!();

    // Test 5: INSERT
    println!("--- Test 5: INSERT ---");
    match conn
        .query("INSERT INTO test_table (id, name) VALUES (1, N'Alice'), (2, N'Bob')")
        .await
    {
        Ok(result) => {
            println!("✓ Insert successful");
            println!("  Rows affected: {}", result.rows_affected);
        }
        Err(e) => {
            println!("✗ Insert failed: {}", e);
        }
    }
    println!();

    // Test 6: SELECT from table
    println!("--- Test 6: SELECT from table ---");
    match conn.query("SELECT * FROM test_table").await {
        Ok(result) => {
            println!("✓ Query executed successfully");
            println!("  Columns: {}", result.columns.len());
            for (i, col) in result.columns.iter().enumerate() {
                println!("  Column {}: {} ({:?})", i, col.name, col.data_type);
            }
            println!("  Rows: {}", result.rows.len());
        }
        Err(e) => {
            println!("✗ Query failed: {}", e);
        }
    }
    println!();

    // Test 7: UPDATE
    println!("--- Test 7: UPDATE ---");
    match conn
        .query("UPDATE test_table SET name = N'Alice Updated' WHERE id = 1")
        .await
    {
        Ok(result) => {
            println!("✓ Update successful");
            println!("  Rows affected: {}", result.rows_affected);
        }
        Err(e) => {
            println!("✗ Update failed: {}", e);
        }
    }
    println!();

    // Test 8: DELETE
    println!("--- Test 8: DELETE ---");
    match conn.query("DELETE FROM test_table WHERE id = 2").await {
        Ok(result) => {
            println!("✓ Delete successful");
            println!("  Rows affected: {}", result.rows_affected);
        }
        Err(e) => {
            println!("✗ Delete failed: {}", e);
        }
    }
    println!();

    // Test 9: DROP TABLE
    println!("--- Test 9: DROP TABLE ---");
    match conn.query("DROP TABLE test_table").await {
        Ok(result) => {
            println!("✓ Table dropped successfully");
            println!("  Rows affected: {}", result.rows_affected);
        }
        Err(e) => {
            println!("✗ Failed to drop table: {}", e);
        }
    }
    println!();

    // Test 10: Error handling
    println!("--- Test 10: Error handling (invalid SQL) ---");
    match conn.query("SELECT * FROM nonexistent_table").await {
        Ok(_) => {
            println!("✗ Query should have failed but succeeded");
        }
        Err(e) => {
            println!("✓ Error correctly caught: {}", e);
        }
    }
    println!();

    println!("=== All Tests Complete ===");

    // Close connection
    conn.close().await?;
    println!("✓ Connection closed");

    Ok(())
}
