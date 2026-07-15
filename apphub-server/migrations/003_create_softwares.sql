-- 创建软件表
CREATE TABLE IF NOT EXISTS softwares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    version VARCHAR(50) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_size INTEGER NOT NULL DEFAULT 0,
    file_path VARCHAR(500) NOT NULL,
    file_hash VARCHAR(64),
    icon VARCHAR(255),
    category_id INTEGER NOT NULL DEFAULT 1,
    status SMALLINT NOT NULL DEFAULT 1,
    download_count INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL
);

-- 创建索引
CREATE INDEX idx_softwares_name ON softwares(name);
CREATE INDEX idx_softwares_category ON softwares(category_id);
CREATE INDEX idx_softwares_status ON softwares(status);
