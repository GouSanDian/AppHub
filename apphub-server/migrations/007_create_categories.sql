-- 创建分类表
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    description VARCHAR(255),
    sort_order INTEGER NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_categories_status ON categories(status);

-- 插入默认分类
INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('办公', '办公软件', 1, 1);
INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('开发', '开发工具', 2, 1);
INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('工具', '系统工具', 3, 1);
INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('娱乐', '娱乐软件', 4, 1);
