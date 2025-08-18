-- ==========================================
-- Vue3 + SpringBoot 文档中心系统数据库结构
-- 版本: 1.0.0
-- 创建时间: 2024-01-01
-- 说明: 仅包含表结构，不包含初始数据
-- ==========================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ==========================================
-- 表结构定义
-- ==========================================

-- 用户表
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `username` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '用户名',
  `password` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '密码',
  `email` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '邮箱',
  `role` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'user' COMMENT '角色',
  `avatar` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '头像',
  `is_enabled` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否启用',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_username` (`username`),
  KEY `idx_username` (`username`),
  KEY `idx_role` (`role`),
  KEY `idx_create_time` (`create_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';

-- 文件分类表
DROP TABLE IF EXISTS `file_category`;
CREATE TABLE `file_category` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `name` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '分类名称',
  `description` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '分类描述',
  `parent_id` bigint DEFAULT NULL COMMENT '父分类ID',
  `sort_order` int NOT NULL DEFAULT '0' COMMENT '排序',
  `is_enabled` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否启用',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `created_by` bigint DEFAULT NULL COMMENT '创建者ID',
  PRIMARY KEY (`id`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_sort_order` (`sort_order`),
  KEY `idx_enabled` (`is_enabled`),
  KEY `idx_created_by` (`created_by`),
  CONSTRAINT `fk_category_parent` FOREIGN KEY (`parent_id`) REFERENCES `file_category` (`id`) ON DELETE SET NULL,
  CONSTRAINT `fk_category_creator` FOREIGN KEY (`created_by`) REFERENCES `sys_user` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文件分类表';

-- 文件标签表
DROP TABLE IF EXISTS `file_tag`;
CREATE TABLE `file_tag` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `name` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '标签名称',
  `color` varchar(7) COLLATE utf8mb4_unicode_ci DEFAULT '#409EFF' COMMENT '标签颜色',
  `description` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '标签描述',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `created_by` bigint DEFAULT NULL COMMENT '创建者ID',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_tag_name` (`name`),
  KEY `idx_name` (`name`),
  KEY `idx_created_by` (`created_by`),
  CONSTRAINT `fk_tag_creator` FOREIGN KEY (`created_by`) REFERENCES `sys_user` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文件标签表';

-- 政策文件表
DROP TABLE IF EXISTS `policy_file`;
CREATE TABLE `policy_file` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `title` varchar(200) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '文件标题',
  `original_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '原始文件名',
  `file_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '存储文件名',
  `file_path` varchar(500) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '文件路径',
  `file_size` bigint NOT NULL COMMENT '文件大小（字节）',
  `file_type` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '文件类型（扩展名）',
  `mime_type` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'MIME类型',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT '文件描述',
  `document_number` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '文件编号',
  `issue_date` datetime DEFAULT NULL COMMENT '发布日期',
  `effective_date` datetime DEFAULT NULL COMMENT '生效日期',
  `issuing_authority` varchar(200) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '发布机关',
  `download_count` int NOT NULL DEFAULT '0' COMMENT '下载次数',
  `view_count` int NOT NULL DEFAULT '0' COMMENT '查看次数',
  `is_public` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否公开',
  `is_enabled` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否启用',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `created_by` bigint NOT NULL COMMENT '创建者ID',
  `updated_by` bigint DEFAULT NULL COMMENT '更新者ID',
  `category_id` bigint DEFAULT NULL COMMENT '分类ID',
  PRIMARY KEY (`id`),
  KEY `idx_title` (`title`),
  KEY `idx_file_type` (`file_type`),
  KEY `idx_category_id` (`category_id`),
  KEY `idx_created_by` (`created_by`),
  KEY `idx_create_time` (`create_time`),
  KEY `idx_issue_date` (`issue_date`),
  KEY `idx_document_number` (`document_number`),
  KEY `idx_is_public` (`is_public`),
  KEY `idx_is_enabled` (`is_enabled`),
  KEY `idx_issuing_authority` (`issuing_authority`),
  KEY `idx_download_count` (`download_count`),
  KEY `idx_view_count` (`view_count`),
  KEY `idx_category_public_enabled` (`category_id`,`is_public`,`is_enabled`),
  KEY `idx_created_time_public` (`create_time`,`is_public`,`is_enabled`),
  KEY `idx_file_type_size` (`file_type`,`file_size`),
  FULLTEXT KEY `idx_title_desc` (`title`,`description`),
  CONSTRAINT `fk_file_category` FOREIGN KEY (`category_id`) REFERENCES `file_category` (`id`) ON DELETE SET NULL,
  CONSTRAINT `fk_file_creator` FOREIGN KEY (`created_by`) REFERENCES `sys_user` (`id`) ON DELETE RESTRICT,
  CONSTRAINT `fk_file_updater` FOREIGN KEY (`updated_by`) REFERENCES `sys_user` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='政策文件表';

-- 文件标签关联表
DROP TABLE IF EXISTS `policy_file_tag`;
CREATE TABLE `policy_file_tag` (
  `file_id` bigint NOT NULL COMMENT '文件ID',
  `tag_id` bigint NOT NULL COMMENT '标签ID',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '关联时间',
  PRIMARY KEY (`file_id`,`tag_id`),
  KEY `idx_file_id` (`file_id`),
  KEY `idx_tag_id` (`tag_id`),
  CONSTRAINT `fk_file_tag_file` FOREIGN KEY (`file_id`) REFERENCES `policy_file` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_file_tag_tag` FOREIGN KEY (`tag_id`) REFERENCES `file_tag` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='文件标签关联表';

-- ==========================================
-- 视图定义
-- ==========================================

-- 文件统计视图
CREATE VIEW `v_file_statistics` AS 
SELECT 
    `c`.`id` AS `category_id`,
    `c`.`name` AS `category_name`,
    COUNT(`f`.`id`) AS `file_count`,
    COALESCE(SUM(`f`.`file_size`), 0) AS `total_size`,
    COALESCE(SUM(`f`.`download_count`), 0) AS `total_downloads`,
    COALESCE(SUM(`f`.`view_count`), 0) AS `total_views`
FROM (`file_category` `c` 
LEFT JOIN `policy_file` `f` ON ((`c`.`id` = `f`.`category_id`) AND (`f`.`is_enabled` = TRUE)))
WHERE (`c`.`is_enabled` = TRUE)
GROUP BY `c`.`id`, `c`.`name`;

-- 热门文件视图
CREATE VIEW `v_popular_files` AS 
SELECT 
    `f`.*,
    `c`.`name` AS `category_name`,
    `u`.`username` AS `creator_name`,
    ((`f`.`view_count` * 0.7) + (`f`.`download_count` * 0.3)) AS `popularity_score`
FROM ((`policy_file` `f` 
LEFT JOIN `file_category` `c` ON (`f`.`category_id` = `c`.`id`)) 
LEFT JOIN `sys_user` `u` ON (`f`.`created_by` = `u`.`id`))
WHERE ((`f`.`is_enabled` = TRUE) AND (`f`.`is_public` = TRUE))
ORDER BY `popularity_score` DESC;

-- 用户文件统计视图
CREATE VIEW `v_user_file_stats` AS 
SELECT 
    `u`.`id` AS `user_id`,
    `u`.`username` AS `username`,
    COUNT(`f`.`id`) AS `file_count`,
    COALESCE(SUM(`f`.`file_size`), 0) AS `total_size`,
    COALESCE(SUM(`f`.`download_count`), 0) AS `total_downloads`,
    COALESCE(SUM(`f`.`view_count`), 0) AS `total_views`,
    MAX(`f`.`create_time`) AS `last_upload_time`
FROM (`sys_user` `u` 
LEFT JOIN `policy_file` `f` ON ((`u`.`id` = `f`.`created_by`) AND (`f`.`is_enabled` = TRUE)))
GROUP BY `u`.`id`, `u`.`username`;

-- ==========================================
-- 函数定义
-- ==========================================

-- 格式化文件大小的函数
DELIMITER //
CREATE FUNCTION `format_file_size`(size_bytes BIGINT) 
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
CREATE FUNCTION `get_category_path`(category_id BIGINT) 
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

SET FOREIGN_KEY_CHECKS = 1;
