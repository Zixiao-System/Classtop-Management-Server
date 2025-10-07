-- Create clients table (注册的客户端设备)
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='clients' AND xtype='U')
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

-- Create courses table (从客户端同步的课程)
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='courses' AND xtype='U')
CREATE TABLE courses (
    id INT IDENTITY(1,1) PRIMARY KEY,
    client_id INT NOT NULL,
    course_id_on_client INT NOT NULL,
    name NVARCHAR(255) NOT NULL,
    teacher NVARCHAR(255),
    color VARCHAR(7),
    note NVARCHAR(MAX),
    synced_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE,
    CONSTRAINT UQ_client_course UNIQUE(client_id, course_id_on_client)
);

-- Create schedule_entries table (从客户端同步的课程表)
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='schedule_entries' AND xtype='U')
CREATE TABLE schedule_entries (
    id INT IDENTITY(1,1) PRIMARY KEY,
    client_id INT NOT NULL,
    entry_id_on_client INT NOT NULL,
    course_id INT NOT NULL,
    day_of_week INT NOT NULL CHECK (day_of_week BETWEEN 1 AND 7),
    start_time VARCHAR(5) NOT NULL,
    end_time VARCHAR(5) NOT NULL,
    weeks NVARCHAR(MAX),
    synced_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE,
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE,
    CONSTRAINT UQ_client_entry UNIQUE(client_id, entry_id_on_client)
);

-- Create settings table (管理服务器配置)
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='settings' AND xtype='U')
CREATE TABLE settings (
    [key] NVARCHAR(255) PRIMARY KEY,
    value NVARCHAR(MAX) NOT NULL
);

-- Create sync_logs table (同步日志)
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='sync_logs' AND xtype='U')
CREATE TABLE sync_logs (
    id INT IDENTITY(1,1) PRIMARY KEY,
    client_id INT NOT NULL,
    sync_type NVARCHAR(50) NOT NULL,  -- 'full', 'incremental'
    status NVARCHAR(50) NOT NULL,     -- 'success', 'failed', 'partial'
    courses_count INT DEFAULT 0,
    entries_count INT DEFAULT 0,
    error_message NVARCHAR(MAX),
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE
);

-- Insert default settings
IF NOT EXISTS (SELECT * FROM settings WHERE [key] = 'server_name')
    INSERT INTO settings ([key], value) VALUES ('server_name', 'ClassTop Management Server');

IF NOT EXISTS (SELECT * FROM settings WHERE [key] = 'auto_sync_interval')
    INSERT INTO settings ([key], value) VALUES ('auto_sync_interval', '300');

IF NOT EXISTS (SELECT * FROM settings WHERE [key] = 'max_clients')
    INSERT INTO settings ([key], value) VALUES ('max_clients', '100');

-- Create indexes
CREATE INDEX idx_courses_client_id ON courses(client_id);
CREATE INDEX idx_schedule_entries_client_id ON schedule_entries(client_id);
CREATE INDEX idx_schedule_entries_course_id ON schedule_entries(course_id);
CREATE INDEX idx_schedule_entries_day_of_week ON schedule_entries(day_of_week);
CREATE INDEX idx_sync_logs_client_id ON sync_logs(client_id);
CREATE INDEX idx_sync_logs_created_at ON sync_logs(created_at);
