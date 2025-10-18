-- 添加 LMS 支持
-- Migration: 003_add_lms_support

-- LMS 实例表
CREATE TABLE IF NOT EXISTS lms_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lms_uuid UUID NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    host VARCHAR(255),
    port INTEGER DEFAULT 8000,
    api_key VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'offline',  -- online, offline, error
    last_heartbeat TIMESTAMPTZ,
    client_count INTEGER DEFAULT 0,
    version VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- LMS 管理的客户端关系表
CREATE TABLE IF NOT EXISTS lms_client_mapping (
    id SERIAL PRIMARY KEY,
    lms_id UUID NOT NULL REFERENCES lms_instances(id) ON DELETE CASCADE,
    client_id UUID NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(lms_id, client_id)
);

-- LMS 心跳日志
CREATE TABLE IF NOT EXISTS lms_heartbeats (
    id SERIAL PRIMARY KEY,
    lms_id UUID NOT NULL REFERENCES lms_instances(id) ON DELETE CASCADE,
    client_count INTEGER,
    received_at TIMESTAMPTZ DEFAULT NOW()
);

-- 为 clients 表添加 lms_id 字段
ALTER TABLE clients ADD COLUMN IF NOT EXISTS lms_id UUID REFERENCES lms_instances(id);

-- 索引
CREATE INDEX IF NOT EXISTS idx_lms_status ON lms_instances(status);
CREATE INDEX IF NOT EXISTS idx_lms_last_heartbeat ON lms_instances(last_heartbeat);
CREATE INDEX IF NOT EXISTS idx_lms_client_mapping_lms ON lms_client_mapping(lms_id);
CREATE INDEX IF NOT EXISTS idx_clients_lms_id ON clients(lms_id);

-- 注释
COMMENT ON TABLE lms_instances IS 'LMS 实例注册表';
COMMENT ON TABLE lms_client_mapping IS 'LMS 与客户端的映射关系';
COMMENT ON TABLE lms_heartbeats IS 'LMS 心跳历史记录';
COMMENT ON COLUMN lms_instances.lms_uuid IS 'LMS 的唯一标识符';
COMMENT ON COLUMN lms_instances.api_key IS 'LMS 的 API 密钥';
COMMENT ON COLUMN lms_instances.status IS 'LMS 状态：online, offline, error';
COMMENT ON COLUMN lms_instances.client_count IS '当前连接的客户端数量';
