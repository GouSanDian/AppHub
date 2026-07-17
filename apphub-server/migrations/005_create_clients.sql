-- 创建客户端表
CREATE TABLE IF NOT EXISTS clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id VARCHAR(64) NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    device_name VARCHAR(100) NOT NULL,
    os_type VARCHAR(20) NOT NULL,
    os_version VARCHAR(50) NOT NULL,
    mac_address VARCHAR(17) NOT NULL,
    ip_address VARCHAR(45),
    last_heartbeat_at DATETIME,
    status SMALLINT NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_clients_client_id ON clients(client_id);
CREATE INDEX idx_clients_user_id ON clients(user_id);
CREATE INDEX idx_clients_status ON clients(status);
