-- 创建黑名单表
CREATE TABLE IF NOT EXISTS blacklists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    process_name VARCHAR(255) NOT NULL,
    description TEXT,
    risk_level SMALLINT NOT NULL DEFAULT 1,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL,
    version INTEGER NOT NULL DEFAULT 1
);

-- 创建索引
CREATE INDEX idx_blacklists_process_name ON blacklists(process_name);
CREATE INDEX idx_blacklists_status ON blacklists(status);
CREATE INDEX idx_blacklists_version ON blacklists(version);
