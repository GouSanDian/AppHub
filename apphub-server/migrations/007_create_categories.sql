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
-- 添加更多常用软件分类
INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('系统工具', '系统实用工具', 5, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('网络工具', '网络相关工具', 6, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('安全软件', '安全防护软件', 7, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('多媒体', '音视频播放和编辑', 8, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('图形图像', '图像处理和查看', 9, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('通讯软件', '即时通讯和邮件', 10, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('教育学习', '教育和学习工具', 11, 1);

INSERT OR IGNORE INTO categories (name, description, sort_order, status) VALUES
('商业软件', '商业应用', 12, 1);