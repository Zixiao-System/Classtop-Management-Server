-- Create clients table (注册的客户端设备)
CREATE TABLE IF NOT EXISTS clients (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    api_url VARCHAR(500) NOT NULL,
    api_key VARCHAR(255),
    last_sync TIMESTAMP,
    status VARCHAR(50) DEFAULT 'offline',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create courses table (从客户端同步的课程)
CREATE TABLE IF NOT EXISTS courses (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    course_id_on_client INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    teacher VARCHAR(255),
    color VARCHAR(7),
    note TEXT,
    synced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(client_id, course_id_on_client)
);

-- Create schedule_entries table (从客户端同步的课程表)
CREATE TABLE IF NOT EXISTS schedule_entries (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    entry_id_on_client INTEGER NOT NULL,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    day_of_week INTEGER NOT NULL CHECK (day_of_week BETWEEN 1 AND 7),
    start_time VARCHAR(5) NOT NULL,
    end_time VARCHAR(5) NOT NULL,
    weeks TEXT,
    synced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(client_id, entry_id_on_client)
);

-- Create settings table (管理服务器配置)
CREATE TABLE IF NOT EXISTS settings (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT NOT NULL
);

-- Create sync_logs table (同步日志)
CREATE TABLE IF NOT EXISTS sync_logs (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    sync_type VARCHAR(50) NOT NULL,  -- 'full', 'incremental'
    status VARCHAR(50) NOT NULL,     -- 'success', 'failed', 'partial'
    courses_count INTEGER DEFAULT 0,
    entries_count INTEGER DEFAULT 0,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert default settings
INSERT INTO settings (key, value) VALUES
    ('server_name', 'ClassTop Management Server'),
    ('auto_sync_interval', '300'),  -- 自动同步间隔（秒）
    ('max_clients', '100')  -- 最大客户端数量
ON CONFLICT (key) DO NOTHING;

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_courses_client_id ON courses(client_id);
CREATE INDEX IF NOT EXISTS idx_schedule_entries_client_id ON schedule_entries(client_id);
CREATE INDEX IF NOT EXISTS idx_schedule_entries_course_id ON schedule_entries(course_id);
CREATE INDEX IF NOT EXISTS idx_schedule_entries_day_of_week ON schedule_entries(day_of_week);
CREATE INDEX IF NOT EXISTS idx_sync_logs_client_id ON sync_logs(client_id);
CREATE INDEX IF NOT EXISTS idx_sync_logs_created_at ON sync_logs(created_at);
