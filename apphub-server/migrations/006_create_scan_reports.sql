-- 创建扫描报告表
CREATE TABLE IF NOT EXISTS scan_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id VARCHAR(64) NOT NULL,
    scan_time DATETIME NOT NULL,
    total_processes INTEGER NOT NULL,
    blacklisted_count INTEGER NOT NULL DEFAULT 0,
    process_list TEXT,
    blacklisted_processes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_scan_reports_client_id ON scan_reports(client_id);
CREATE INDEX idx_scan_reports_scan_time ON scan_reports(scan_time);
CREATE INDEX idx_scan_reports_created_at ON scan_reports(created_at);
