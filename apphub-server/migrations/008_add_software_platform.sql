-- 为 softwares 表添加 platform 字段
-- 存储格式：逗号分隔的平台列表，如 "mac,linux,windows"
ALTER TABLE softwares ADD COLUMN platform VARCHAR(100) NOT NULL DEFAULT 'mac,linux,windows';

-- 为 platform 创建索引（用于按平台筛选）
CREATE INDEX idx_softwares_platform ON softwares(platform);
