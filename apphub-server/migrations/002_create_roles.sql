-- 创建角色表
CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(255),
    permissions TEXT,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_roles_name ON roles(name);

-- 插入默认角色
INSERT OR IGNORE INTO roles (id, name, description, status) VALUES
(1, 'super_admin', '超级管理员', 1);
INSERT OR IGNORE INTO roles (id, name, description, status) VALUES
(2, 'admin', '管理员', 1);
INSERT OR IGNORE INTO roles (id, name, description, status) VALUES
(3, 'user', '普通用户', 1);

-- 插入默认管理员（需要在 roles 表创建后执行, 密码默认为admin）
INSERT OR IGNORE INTO users (username, password_hash, nickname, role_id, status)
VALUES ('admin', '$2b$12$EVFw3ad/LwEFSkqEAEoQ/.dDMSutvdrP5ADd1sIspk1gUdtRg6xWS', '超级管理员', 1, 1);