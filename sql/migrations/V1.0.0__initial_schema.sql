-- ==========================================
-- 数据库版本: V1.0.0
-- 创建时间: 2024-01-01
-- 描述: 初始数据库结构创建
-- ==========================================

-- 创建版本管理表
CREATE TABLE IF NOT EXISTS schema_version (
    id INT AUTO_INCREMENT PRIMARY KEY,
    version VARCHAR(20) NOT NULL UNIQUE,
    description TEXT,
    script_name VARCHAR(255),
    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    execution_time_ms INT,
    success BOOLEAN DEFAULT TRUE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 记录当前版本
INSERT INTO schema_version (version, description, script_name) 
VALUES ('1.0.0', '初始数据库结构创建', 'V1.0.0__initial_schema.sql');

-- 执行主要的数据库创建脚本
-- (这里可以包含 schema.sql 的内容，或者通过外部脚本执行)

SELECT 'Schema version 1.0.0 applied successfully' as result;
