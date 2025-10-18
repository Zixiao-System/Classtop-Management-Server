-- 添加 CCTV 支持
-- Migration: 004_add_cctv_support

-- CCTV 配置表
CREATE TABLE IF NOT EXISTS cctv_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id UUID NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    camera_id VARCHAR(100) NOT NULL,
    camera_name VARCHAR(255) NOT NULL,
    rtsp_url TEXT,
    recording_enabled BOOLEAN DEFAULT FALSE,
    streaming_enabled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(client_id, camera_id)
);

-- CCTV 事件表
CREATE TABLE IF NOT EXISTS cctv_events (
    id SERIAL PRIMARY KEY,
    camera_config_id UUID NOT NULL REFERENCES cctv_configs(id) ON DELETE CASCADE,
    event_type VARCHAR(50) NOT NULL,  -- start_recording, stop_recording, start_stream, stop_stream, error
    details JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_cctv_configs_client ON cctv_configs(client_id);
CREATE INDEX IF NOT EXISTS idx_cctv_events_camera ON cctv_events(camera_config_id);
CREATE INDEX IF NOT EXISTS idx_cctv_events_time ON cctv_events(created_at);
CREATE INDEX IF NOT EXISTS idx_cctv_events_type ON cctv_events(event_type);

-- 注释
COMMENT ON TABLE cctv_configs IS 'CCTV 摄像头配置表';
COMMENT ON TABLE cctv_events IS 'CCTV 事件日志表';
COMMENT ON COLUMN cctv_configs.camera_id IS '摄像头唯一标识';
COMMENT ON COLUMN cctv_configs.rtsp_url IS 'RTSP 流地址';
COMMENT ON COLUMN cctv_events.event_type IS '事件类型：start_recording, stop_recording, start_stream, stop_stream, error';
COMMENT ON COLUMN cctv_events.details IS '事件详细信息（JSON 格式）';
