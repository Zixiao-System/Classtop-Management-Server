-- Create clients table (注册的客户端设备)
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'clients')
BEGIN
    CREATE TABLE clients (
        id INT IDENTITY(1,1) PRIMARY KEY,
        uuid NVARCHAR(255) UNIQUE NOT NULL,
        name NVARCHAR(255) NOT NULL,
        description NVARCHAR(MAX),
        api_url NVARCHAR(500) NOT NULL,
        api_key NVARCHAR(255),
        last_sync DATETIME2,
        status NVARCHAR(50) DEFAULT 'offline',
        created_at DATETIME2 DEFAULT GETDATE()
    );
END;
GO

-- Create courses table (从客户端同步的课程)
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'courses')
BEGIN
    CREATE TABLE courses (
        id INT IDENTITY(1,1) PRIMARY KEY,
        client_id INT NOT NULL,
        course_id_on_client INT NOT NULL,
        name NVARCHAR(255) NOT NULL,
        teacher NVARCHAR(255),
        color NVARCHAR(7),
        note NVARCHAR(MAX),
        synced_at DATETIME2 DEFAULT GETDATE(),
        CONSTRAINT FK_courses_client FOREIGN KEY (client_id)
            REFERENCES clients(id) ON DELETE CASCADE,
        CONSTRAINT UQ_client_course UNIQUE(client_id, course_id_on_client)
    );
END;
GO

-- Create schedule_entries table (从客户端同步的课程表)
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'schedule_entries')
BEGIN
    CREATE TABLE schedule_entries (
        id INT IDENTITY(1,1) PRIMARY KEY,
        client_id INT NOT NULL,
        entry_id_on_client INT NOT NULL,
        course_id INT NOT NULL,
        day_of_week INT NOT NULL CHECK (day_of_week BETWEEN 1 AND 7),
        start_time NVARCHAR(5) NOT NULL,
        end_time NVARCHAR(5) NOT NULL,
        weeks NVARCHAR(MAX),
        synced_at DATETIME2 DEFAULT GETDATE(),
        CONSTRAINT FK_schedule_client FOREIGN KEY (client_id)
            REFERENCES clients(id) ON DELETE CASCADE,
        CONSTRAINT FK_schedule_course FOREIGN KEY (course_id)
            REFERENCES courses(id) ON DELETE CASCADE,
        CONSTRAINT UQ_client_entry UNIQUE(client_id, entry_id_on_client)
    );
END;
GO

-- Create settings table (管理服务器配置)
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'settings')
BEGIN
    CREATE TABLE settings (
        [key] NVARCHAR(255) PRIMARY KEY,
        value NVARCHAR(MAX) NOT NULL
    );
END;
GO

-- Create sync_logs table (同步日志)
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'sync_logs')
BEGIN
    CREATE TABLE sync_logs (
        id INT IDENTITY(1,1) PRIMARY KEY,
        client_id INT NOT NULL,
        sync_type NVARCHAR(50) NOT NULL,  -- 'full', 'incremental'
        status NVARCHAR(50) NOT NULL,     -- 'success', 'failed', 'partial'
        courses_count INT DEFAULT 0,
        entries_count INT DEFAULT 0,
        error_message NVARCHAR(MAX),
        created_at DATETIME2 DEFAULT GETDATE(),
        CONSTRAINT FK_synclog_client FOREIGN KEY (client_id)
            REFERENCES clients(id) ON DELETE CASCADE
    );
END;
GO

-- Insert default settings (使用 MERGE 避免重复)
MERGE INTO settings AS target
USING (VALUES
    ('server_name', 'ClassTop Management Server'),
    ('auto_sync_interval', '300'),
    ('max_clients', '100')
) AS source ([key], value)
ON target.[key] = source.[key]
WHEN NOT MATCHED THEN
    INSERT ([key], value) VALUES (source.[key], source.value);
GO

-- Create indexes
IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'idx_courses_client_id')
    CREATE INDEX idx_courses_client_id ON courses(client_id);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'idx_schedule_entries_client_id')
    CREATE INDEX idx_schedule_entries_client_id ON schedule_entries(client_id);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'idx_schedule_entries_course_id')
    CREATE INDEX idx_schedule_entries_course_id ON schedule_entries(course_id);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'idx_schedule_entries_day_of_week')
    CREATE INDEX idx_schedule_entries_day_of_week ON schedule_entries(day_of_week);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'idx_sync_logs_client_id')
    CREATE INDEX idx_sync_logs_client_id ON sync_logs(client_id);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'idx_sync_logs_created_at')
    CREATE INDEX idx_sync_logs_created_at ON sync_logs(created_at);
GO
