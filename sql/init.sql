-- ==========================================
-- Vue3 + SpringBoot 文件管理系统数据库初始化脚本
-- 版本: 1.0.0
-- 创建时间: 2024-01-01
-- 说明: 完整的数据库初始化，包含表结构、初始数据、视图、函数等
-- ==========================================

-- 设置SQL模式和字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;
SET sql_mode = 'STRICT_TRANS_TABLES,NO_ZERO_DATE,NO_ZERO_IN_DATE,ERROR_FOR_DIVISION_BY_ZERO';

-- 创建数据库
CREATE DATABASE IF NOT EXISTS document_center_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE document_center_db;

-- 开始事务
START TRANSACTION;

-- 删除已存在的表（按依赖关系顺序）
DROP TABLE IF EXISTS policy_file_tag;
DROP TABLE IF EXISTS policy_file;
DROP TABLE IF EXISTS file_tag;
DROP TABLE IF EXISTS file_category;
DROP TABLE IF EXISTS sys_user;

-- 用户表
CREATE TABLE sys_user (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password VARCHAR(255) NOT NULL COMMENT '密码',
    email VARCHAR(100) COMMENT '邮箱',
    role VARCHAR(20) NOT NULL DEFAULT 'user' COMMENT '角色',
    avatar VARCHAR(200) COMMENT '头像',
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否启用',
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',

    INDEX idx_username (username),
    INDEX idx_role (role),
    INDEX idx_create_time (create_time)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';

-- 文件分类表
CREATE TABLE file_category (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL COMMENT '分类名称',
    description VARCHAR(500) COMMENT '分类描述',
    parent_id BIGINT COMMENT '父分类ID',
    sort_order INT NOT NULL DEFAULT 0 COMMENT '排序',
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否启用',
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    created_by BIGINT COMMENT '创建者ID',

    INDEX idx_parent_id (parent_id),
    INDEX idx_sort_order (sort_order),
    INDEX idx_enabled (is_enabled),
    INDEX idx_created_by (created_by)
    -- 注意：父分类外键约束在插入数据后添加，避免循环引用问题
    -- FOREIGN KEY (parent_id) REFERENCES file_category(id) ON DELETE SET NULL,
    -- FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文件分类表';

-- 文件标签表
CREATE TABLE file_tag (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE COMMENT '标签名称',
    color VARCHAR(7) DEFAULT '#409EFF' COMMENT '标签颜色',
    description VARCHAR(200) COMMENT '标签描述',
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    created_by BIGINT COMMENT '创建者ID',

    INDEX idx_name (name),
    INDEX idx_created_by (created_by)
    -- 外键约束在数据插入后添加
    -- FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文件标签表';

-- 政策文件表
CREATE TABLE policy_file (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(200) NOT NULL COMMENT '文件标题',
    original_name VARCHAR(255) NOT NULL COMMENT '原始文件名',
    file_name VARCHAR(255) NOT NULL COMMENT '存储文件名',
    file_path VARCHAR(500) NOT NULL COMMENT '文件路径',
    file_size BIGINT NOT NULL COMMENT '文件大小（字节）',
    file_type VARCHAR(50) NOT NULL COMMENT '文件类型（扩展名）',
    mime_type VARCHAR(100) COMMENT 'MIME类型',
    description TEXT COMMENT '文件描述',
    document_number VARCHAR(100) COMMENT '文件编号',
    issue_date DATETIME COMMENT '发布日期',
    effective_date DATETIME COMMENT '生效日期',
    issuing_authority VARCHAR(200) COMMENT '发布机关',
    download_count INT NOT NULL DEFAULT 0 COMMENT '下载次数',
    view_count INT NOT NULL DEFAULT 0 COMMENT '查看次数',
    is_public BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否公开',
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否启用',
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    created_by BIGINT NOT NULL COMMENT '创建者ID',
    updated_by BIGINT COMMENT '更新者ID',
    category_id BIGINT COMMENT '分类ID',

    INDEX idx_title (title),
    INDEX idx_file_type (file_type),
    INDEX idx_category_id (category_id),
    INDEX idx_created_by (created_by),
    INDEX idx_create_time (create_time),
    INDEX idx_issue_date (issue_date),
    INDEX idx_document_number (document_number),
    INDEX idx_is_public (is_public),
    INDEX idx_is_enabled (is_enabled),
    INDEX idx_issuing_authority (issuing_authority),
    INDEX idx_download_count (download_count),
    INDEX idx_view_count (view_count),
    INDEX idx_category_public_enabled (category_id, is_public, is_enabled),
    INDEX idx_created_time_public (create_time, is_public, is_enabled),
    INDEX idx_file_type_size (file_type, file_size),
    FULLTEXT idx_title_desc (title, description)
    -- 外键约束在数据插入后添加
    -- FOREIGN KEY (category_id) REFERENCES file_category(id) ON DELETE SET NULL,
    -- FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE RESTRICT,
    -- FOREIGN KEY (updated_by) REFERENCES sys_user(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='政策文件表';

-- 文件标签关联表
CREATE TABLE policy_file_tag (
    file_id BIGINT NOT NULL COMMENT '文件ID',
    tag_id BIGINT NOT NULL COMMENT '标签ID',
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '关联时间',

    PRIMARY KEY (file_id, tag_id),
    INDEX idx_file_id (file_id),
    INDEX idx_tag_id (tag_id)
    -- 外键约束在数据插入后添加
    -- FOREIGN KEY (file_id) REFERENCES policy_file(id) ON DELETE CASCADE,
    -- FOREIGN KEY (tag_id) REFERENCES file_tag(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文件标签关联表';

-- ==========================================
-- 插入初始数据
-- ==========================================

-- 插入初始用户（使用明文密码，仅用于开发演示）
-- 注意：生产环境请使用加密密码
INSERT INTO sys_user (username, password, email, role, is_enabled) VALUES
('admin', 'admin123', 'admin@example.com', 'admin', TRUE),
('user', 'user123', 'user@example.com', 'user', TRUE),
('demo', 'demo123', 'demo@example.com', 'user', TRUE);

-- 插入默认分类
INSERT INTO file_category (name, description, sort_order, created_by) VALUES
('政策法规', '国家和地方政策法规文件', 1, 1),
('通知公告', '各类通知和公告文件', 2, 1),
('规范性文件', '行业规范和标准文件', 3, 1),
('工作文件', '日常工作相关文件', 4, 1),
('技术文档', '技术规范和操作手册', 5, 1),
('培训资料', '培训课件和学习资料', 6, 1);

-- 插入子分类
INSERT INTO file_category (name, description, parent_id, sort_order, created_by) VALUES
('国家政策', '国家级政策文件', 1, 1, 1),
('地方政策', '地方政府政策文件', 1, 2, 1),
('行业政策', '行业相关政策文件', 1, 3, 1),
('部门通知', '各部门发布的通知', 2, 1, 1),
('公开公告', '面向公众的公告', 2, 2, 1),
('紧急通知', '紧急事务通知', 2, 3, 1),
('国家标准', '国家标准规范', 3, 1, 1),
('行业标准', '行业标准规范', 3, 2, 1),
('企业标准', '企业内部标准', 3, 3, 1),
('会议纪要', '会议记录和纪要', 4, 1, 1),
('报告总结', '工作报告和总结', 4, 2, 1),
('计划方案', '工作计划和实施方案', 4, 3, 1);

-- 插入默认标签
INSERT INTO file_tag (name, color, description, created_by) VALUES
('重要', '#F56C6C', '重要文件标签', 1),
('紧急', '#E6A23C', '紧急文件标签', 1),
('常规', '#409EFF', '常规文件标签', 1),
('归档', '#909399', '已归档文件标签', 1),
('草案', '#67C23A', '草案文件标签', 1),
('机密', '#722ED1', '机密文件标签', 1),
('公开', '#52C41A', '公开文件标签', 1),
('临时', '#FA8C16', '临时文件标签', 1),
('废止', '#8C8C8C', '已废止文件标签', 1),
('最新', '#1890FF', '最新文件标签', 1);

-- ==========================================
-- 创建视图
-- ==========================================

-- 文件统计视图
CREATE VIEW v_file_statistics AS
SELECT
    c.id as category_id,
    c.name as category_name,
    COUNT(f.id) as file_count,
    COALESCE(SUM(f.file_size), 0) as total_size,
    COALESCE(SUM(f.download_count), 0) as total_downloads,
    COALESCE(SUM(f.view_count), 0) as total_views
FROM file_category c
LEFT JOIN policy_file f ON c.id = f.category_id AND f.is_enabled = TRUE
WHERE c.is_enabled = TRUE
GROUP BY c.id, c.name;

-- 热门文件视图
CREATE VIEW v_popular_files AS
SELECT
    f.*,
    c.name as category_name,
    u.username as creator_name,
    (f.view_count * 0.7 + f.download_count * 0.3) as popularity_score
FROM policy_file f
LEFT JOIN file_category c ON f.category_id = c.id
LEFT JOIN sys_user u ON f.created_by = u.id
WHERE f.is_enabled = TRUE AND f.is_public = TRUE
ORDER BY popularity_score DESC;

-- 用户文件统计视图
CREATE VIEW v_user_file_stats AS
SELECT
    u.id as user_id,
    u.username,
    COUNT(f.id) as file_count,
    COALESCE(SUM(f.file_size), 0) as total_size,
    COALESCE(SUM(f.download_count), 0) as total_downloads,
    COALESCE(SUM(f.view_count), 0) as total_views,
    MAX(f.create_time) as last_upload_time
FROM sys_user u
LEFT JOIN policy_file f ON u.id = f.created_by AND f.is_enabled = TRUE
GROUP BY u.id, u.username;

-- ==========================================
-- 创建存储过程和函数
-- ==========================================

-- 格式化文件大小的函数
DELIMITER //
CREATE FUNCTION format_file_size(size_bytes BIGINT)
RETURNS VARCHAR(20)
READS SQL DATA
DETERMINISTIC
BEGIN
    DECLARE result VARCHAR(20);

    IF size_bytes < 1024 THEN
        SET result = CONCAT(size_bytes, ' B');
    ELSEIF size_bytes < 1024 * 1024 THEN
        SET result = CONCAT(ROUND(size_bytes / 1024, 1), ' KB');
    ELSEIF size_bytes < 1024 * 1024 * 1024 THEN
        SET result = CONCAT(ROUND(size_bytes / (1024 * 1024), 1), ' MB');
    ELSE
        SET result = CONCAT(ROUND(size_bytes / (1024 * 1024 * 1024), 1), ' GB');
    END IF;

    RETURN result;
END //
DELIMITER ;

-- 获取分类路径的函数
DELIMITER //
CREATE FUNCTION get_category_path(category_id BIGINT)
RETURNS TEXT
READS SQL DATA
DETERMINISTIC
BEGIN
    DECLARE path TEXT DEFAULT '';
    DECLARE current_id BIGINT DEFAULT category_id;
    DECLARE current_name VARCHAR(100);
    DECLARE parent_id BIGINT;
    DECLARE done INT DEFAULT FALSE;

    WHILE current_id IS NOT NULL AND NOT done DO
        SELECT name, parent_id INTO current_name, parent_id
        FROM file_category
        WHERE id = current_id;

        IF current_name IS NOT NULL THEN
            IF path = '' THEN
                SET path = current_name;
            ELSE
                SET path = CONCAT(current_name, ' > ', path);
            END IF;
            SET current_id = parent_id;
        ELSE
            SET done = TRUE;
        END IF;
    END WHILE;

    RETURN path;
END //
DELIMITER ;

-- 清理无效文件的存储过程
DELIMITER //
CREATE PROCEDURE clean_invalid_files()
BEGIN
    DECLARE done INT DEFAULT FALSE;
    DECLARE file_id BIGINT;
    DECLARE file_path VARCHAR(500);

    -- 声明游标
    DECLARE file_cursor CURSOR FOR
        SELECT id, file_path FROM policy_file WHERE is_enabled = TRUE;

    DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

    START TRANSACTION;

    OPEN file_cursor;

    read_loop: LOOP
        FETCH file_cursor INTO file_id, file_path;
        IF done THEN
            LEAVE read_loop;
        END IF;

        -- 这里可以添加文件存在性检查逻辑
        -- 如果文件不存在，可以标记为已删除
        -- UPDATE policy_file SET is_enabled = FALSE WHERE id = file_id;

    END LOOP;

    CLOSE file_cursor;

    COMMIT;
END //
DELIMITER ;

-- 更新文件统计的存储过程
DELIMITER //
CREATE PROCEDURE update_file_statistics()
BEGIN
    -- 更新分类文件数量（可以定期执行）
    UPDATE file_category c
    SET description = CONCAT(
        COALESCE(SUBSTRING_INDEX(description, '(', 1), description),
        '(',
        (SELECT COUNT(*) FROM policy_file f WHERE f.category_id = c.id AND f.is_enabled = TRUE),
        '个文件)'
    )
    WHERE c.is_enabled = TRUE;
END //
DELIMITER ;

-- ==========================================
-- 插入示例数据（可选）
-- ==========================================

-- 插入一些示例文件（仅用于演示）
INSERT INTO policy_file (
    title, original_name, file_name, file_path, file_size, file_type, mime_type,
    description, document_number, issue_date, effective_date, issuing_authority,
    is_public, category_id, created_by
) VALUES
(
    '系统使用手册', 'system_manual.pdf', 'uuid_system_manual.pdf',
    '2024/01/01/uuid_system_manual.pdf', 2048576, 'pdf', 'application/pdf',
    '系统操作使用手册，包含详细的功能说明和操作步骤', 'DOC-2024-001',
    '2024-01-01 00:00:00', '2024-01-01 00:00:00', '技术部',
    TRUE, 5, 1  -- 技术文档分类
),
(
    '数据备份规范', 'backup_standard.docx', 'uuid_backup_standard.docx',
    '2024/01/01/uuid_backup_standard.docx', 1024768, 'docx', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    '数据备份的标准操作规范，确保数据安全', 'STD-2024-001',
    '2024-01-01 00:00:00', '2024-01-01 00:00:00', '运维部',
    TRUE, 3, 1  -- 规范性文件分类
),
(
    '培训计划表', 'training_plan.xlsx', 'uuid_training_plan.xlsx',
    '2024/01/01/uuid_training_plan.xlsx', 512384, 'xlsx', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    '2024年度员工培训计划安排表', 'PLAN-2024-001',
    '2024-01-01 00:00:00', '2024-01-01 00:00:00', '人事部',
    TRUE, 6, 2  -- 培训资料分类
);

-- 为示例文件添加标签
INSERT INTO policy_file_tag (file_id, tag_id) VALUES
(1, 1), (1, 7),  -- 系统使用手册：重要、公开
(2, 1), (2, 3),  -- 数据备份规范：重要、常规
(3, 3), (3, 7);  -- 培训计划表：常规、公开

-- ==========================================
-- 添加外键约束（在数据插入完成后）
-- ==========================================

-- 添加文件分类表的外键约束
ALTER TABLE file_category
ADD CONSTRAINT fk_category_creator
FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL;

-- 添加文件标签表的外键约束
ALTER TABLE file_tag
ADD CONSTRAINT fk_tag_creator
FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL;

-- 添加政策文件表的外键约束
ALTER TABLE policy_file
ADD CONSTRAINT fk_file_category
FOREIGN KEY (category_id) REFERENCES file_category(id) ON DELETE SET NULL;

ALTER TABLE policy_file
ADD CONSTRAINT fk_file_creator
FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE RESTRICT;

ALTER TABLE policy_file
ADD CONSTRAINT fk_file_updater
FOREIGN KEY (updated_by) REFERENCES sys_user(id) ON DELETE SET NULL;

-- 添加文件标签关联表的外键约束
ALTER TABLE policy_file_tag
ADD CONSTRAINT fk_file_tag_file
FOREIGN KEY (file_id) REFERENCES policy_file(id) ON DELETE CASCADE;

ALTER TABLE policy_file_tag
ADD CONSTRAINT fk_file_tag_tag
FOREIGN KEY (tag_id) REFERENCES file_tag(id) ON DELETE CASCADE;

-- ==========================================
-- 性能优化和维护
-- ==========================================

-- 分析表以优化查询性能
ANALYZE TABLE sys_user;
ANALYZE TABLE file_category;
ANALYZE TABLE file_tag;
ANALYZE TABLE policy_file;
ANALYZE TABLE policy_file_tag;

-- 复合索引已在表创建时添加，无需重复创建

-- ==========================================
-- 数据完整性检查
-- ==========================================

-- 检查数据完整性的查询（可以定期执行）
-- 检查孤立的文件（没有创建者的文件）
-- SELECT * FROM policy_file WHERE created_by NOT IN (SELECT id FROM sys_user);

-- 检查孤立的分类（父分类不存在的分类）
-- SELECT * FROM file_category WHERE parent_id IS NOT NULL AND parent_id NOT IN (SELECT id FROM file_category);

-- 检查文件标签关联的完整性
-- SELECT * FROM policy_file_tag WHERE file_id NOT IN (SELECT id FROM policy_file);
-- SELECT * FROM policy_file_tag WHERE tag_id NOT IN (SELECT id FROM file_tag);

-- ==========================================
-- 权限设置（根据需要调整）
-- ==========================================

-- 创建只读用户（用于报表查询等）
-- CREATE USER 'readonly'@'%' IDENTIFIED BY 'readonly_password';
-- GRANT SELECT ON document_center_db.* TO 'readonly'@'%';

-- 创建应用用户（用于应用程序连接）
-- CREATE USER 'app_user'@'%' IDENTIFIED BY 'app_password';
-- GRANT SELECT, INSERT, UPDATE, DELETE ON document_center_db.* TO 'app_user'@'%';

-- 刷新权限
-- FLUSH PRIVILEGES;

-- ==========================================
-- 数据验证和完整性检查
-- ==========================================

-- 检查数据完整性-- 创建存储过程来执行初始化检查
DELIMITER //
CREATE PROCEDURE check_database_initialization()
BEGIN
    -- 禁用外键检查
    SET FOREIGN_KEY_CHECKS = 0;
    
    -- 开始事务
    START TRANSACTION;
    
    -- 初始化错误计数器
    SET @error_count = 0;
    
    -- 检查用户数据
    SELECT COUNT(*) INTO @user_count FROM sys_user;
    IF @user_count = 0 THEN
        SET @error_count = @error_count + 1;
        SELECT 'ERROR: 没有用户数据' as error_message;
    END IF;
    
    -- 检查分类数据
    SELECT COUNT(*) INTO @category_count FROM file_category;
    IF @category_count = 0 THEN
        SET @error_count = @error_count + 1;
        SELECT 'ERROR: 没有分类数据' as error_message;
    END IF;
    
    -- 检查标签数据
    SELECT COUNT(*) INTO @tag_count FROM file_tag;
    IF @tag_count = 0 THEN
        SET @error_count = @error_count + 1;
        SELECT 'ERROR: 没有标签数据' as error_message;
    END IF;
    
    -- 初始化完成提示
    SELECT
        CASE
            WHEN @error_count = 0 THEN 'Database initialization completed successfully!'
            ELSE CONCAT('Database initialization completed with ', @error_count, ' errors!')
        END as message,
        (SELECT COUNT(*) FROM sys_user) as user_count,
        (SELECT COUNT(*) FROM file_category) as category_count,
        (SELECT COUNT(*) FROM file_tag) as tag_count,
        (SELECT COUNT(*) FROM policy_file) as file_count,
        NOW() as initialization_time;
    
    -- 提交事务
    COMMIT;
    
    -- 恢复外键检查
    SET FOREIGN_KEY_CHECKS = 1;
    
    -- 最终提示
    SELECT '🎉 数据库初始化完成！' as final_message;
END //
DELIMITER ;

-- 执行存储过程
CALL check_database_initialization();

-- 如果需要，可以删除存储过程
-- DROP PROCEDURE IF EXISTS check_database_initialization;