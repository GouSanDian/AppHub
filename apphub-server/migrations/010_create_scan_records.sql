-- 创建黑名单扫描记录表
CREATE TABLE IF NOT EXISTS scan_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id VARCHAR(64) NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 0,
    username VARCHAR(50) NOT NULL DEFAULT '',
    process_name VARCHAR(255) NOT NULL,
    risk_level INTEGER NOT NULL DEFAULT 1,
    scan_time DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_scan_records_client_id ON scan_records(client_id);
CREATE INDEX idx_scan_records_username ON scan_records(username);
CREATE INDEX idx_scan_records_scan_time ON scan_records(scan_time);
CREATE INDEX idx_scan_records_created_at ON scan_records(created_at);
