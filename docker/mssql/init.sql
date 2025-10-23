-- ================================================================
-- SQL Server 初始化脚本
-- 用于 ClassTop Management Server 开发环境
-- ================================================================

USE master;
GO

-- 创建测试数据库
IF NOT EXISTS (SELECT name FROM sys.databases WHERE name = N'classtop_dev')
BEGIN
    CREATE DATABASE classtop_dev;
    PRINT 'Database [classtop_dev] created successfully.';
END
ELSE
BEGIN
    PRINT 'Database [classtop_dev] already exists.';
END
GO

USE classtop_dev;
GO

-- 创建测试表（用于验证驱动功能）
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[test_types]') AND type in (N'U'))
BEGIN
    CREATE TABLE test_types (
        id INT IDENTITY(1,1) PRIMARY KEY,
        test_int INT,
        test_bigint BIGINT,
        test_varchar VARCHAR(100),
        test_nvarchar NVARCHAR(100),
        test_datetime DATETIME2,
        test_uuid UNIQUEIDENTIFIER,
        test_bool BIT,
        test_float FLOAT,
        test_decimal DECIMAL(18, 2),
        created_at DATETIME2 DEFAULT GETDATE()
    );
    PRINT 'Table [test_types] created successfully.';
END
GO

-- 插入测试数据
INSERT INTO test_types (test_int, test_bigint, test_varchar, test_nvarchar, test_datetime, test_uuid, test_bool, test_float, test_decimal)
VALUES
    (42, 9223372036854775807, 'hello', N'你好世界', '2024-01-01 12:30:45', NEWID(), 1, 3.14159, 1234.56),
    (-100, -9223372036854775808, 'world', N'こんにちは', '2024-12-31 23:59:59', NEWID(), 0, 2.71828, -999.99);
GO

-- 创建测试存储过程
IF EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[sp_add_numbers]') AND type in (N'P', N'PC'))
    DROP PROCEDURE [dbo].[sp_add_numbers];
GO

CREATE PROCEDURE sp_add_numbers
    @a INT,
    @b INT,
    @result INT OUTPUT
AS
BEGIN
    SET @result = @a + @b;
END
GO

-- 创建视图（测试视图查询）
IF EXISTS (SELECT * FROM sys.views WHERE object_id = OBJECT_ID(N'[dbo].[v_test_summary]'))
    DROP VIEW [dbo].[v_test_summary];
GO

CREATE VIEW v_test_summary AS
SELECT
    COUNT(*) AS total_rows,
    AVG(CAST(test_int AS FLOAT)) AS avg_int,
    MAX(test_datetime) AS latest_datetime
FROM test_types;
GO

-- 显示初始化结果
SELECT
    DB_NAME() AS current_database,
    @@VERSION AS server_version,
    (SELECT COUNT(*) FROM test_types) AS test_data_count;
GO

PRINT '========================================';
PRINT 'SQL Server initialization completed!';
PRINT 'Database: classtop_dev';
PRINT 'Test table: test_types (with sample data)';
PRINT 'Test procedure: sp_add_numbers';
PRINT 'Test view: v_test_summary';
PRINT '========================================';
GO
