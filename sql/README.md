# 数据库配置说明

## 文件说明

### 1. init.sql
- **用途**: 完整的数据库初始化脚本
- **包含**: 表结构 + 初始数据 + 示例数据 + 视图 + 函数 + 存储过程
- **适用**: 开发环境、演示环境
- **执行方式**: 
  ```bash
  mysql -u root -p < sql/init.sql
  ```

### 2. schema.sql
- **用途**: 仅包含数据库结构
- **包含**: 表结构 + 视图 + 函数（不包含数据）
- **适用**: 生产环境
- **执行方式**:
  ```bash
  mysql -u root -p vue_springboot_db < sql/schema.sql
  ```

## 数据库结构

### 核心表

#### 1. sys_user (用户表)
- 存储系统用户信息
- 支持角色管理 (admin/user)
- 包含用户状态控制

#### 2. file_category (文件分类表)
- 支持无限级分类结构
- 树形结构设计
- 包含排序和状态控制

#### 3. file_tag (文件标签表)
- 文件标签管理
- 支持颜色标识
- 多对多关联设计

#### 4. policy_file (政策文件表)
- 核心文件信息存储
- 包含文件元数据
- 支持统计和权限控制

#### 5. policy_file_tag (文件标签关联表)
- 文件与标签的多对多关联
- 记录关联时间

### 视图

#### 1. v_file_statistics
- 文件统计信息
- 按分类汇总文件数量、大小、下载量等

#### 2. v_popular_files
- 热门文件排行
- 基于查看次数和下载次数计算热度

#### 3. v_user_file_stats
- 用户文件统计
- 每个用户的文件上传情况

### 函数

#### 1. format_file_size(size_bytes)
- 格式化文件大小显示
- 自动转换为 B/KB/MB/GB

#### 2. get_category_path(category_id)
- 获取分类的完整路径
- 返回如 "政策法规 > 国家政策" 的路径字符串

### 存储过程

#### 1. clean_invalid_files()
- 清理无效文件记录
- 可定期执行维护数据完整性

#### 2. update_file_statistics()
- 更新文件统计信息
- 可定期执行保持统计准确性

## 索引优化

### 单列索引
- 主键索引 (自动创建)
- 唯一索引 (username, tag_name)
- 普通索引 (常用查询字段)

### 复合索引
- `idx_category_public_enabled`: 分类+公开状态+启用状态
- `idx_created_time_public`: 创建时间+公开状态+启用状态
- `idx_file_type_size`: 文件类型+文件大小

### 全文索引
- `idx_title_desc`: 文件标题和描述的全文搜索

## 数据完整性

### 外键约束
- 确保数据引用完整性
- 支持级联删除和设置NULL

### 检查约束
- 文件大小必须大于0
- 状态字段限制为有效值

## 性能优化建议

### 1. 定期维护
```sql
-- 分析表统计信息
ANALYZE TABLE policy_file;

-- 优化表结构
OPTIMIZE TABLE policy_file;
```

### 2. 监控慢查询
```sql
-- 开启慢查询日志
SET GLOBAL slow_query_log = 'ON';
SET GLOBAL long_query_time = 2;
```

### 3. 分区建议
对于大量数据，可考虑按时间分区：
```sql
-- 按年份分区示例
ALTER TABLE policy_file 
PARTITION BY RANGE (YEAR(create_time)) (
    PARTITION p2023 VALUES LESS THAN (2024),
    PARTITION p2024 VALUES LESS THAN (2025),
    PARTITION p_future VALUES LESS THAN MAXVALUE
);
```

## 备份策略

### 1. 结构备份
```bash
mysqldump -u root -p --no-data vue_springboot_db > schema_backup.sql
```

### 2. 数据备份
```bash
mysqldump -u root -p vue_springboot_db > full_backup.sql
```

### 3. 增量备份
启用二进制日志进行增量备份：
```sql
-- 在 my.cnf 中配置
log-bin=mysql-bin
expire_logs_days=7
```

## 安全配置

### 1. 用户权限
```sql
-- 创建应用专用用户
CREATE USER 'app_user'@'localhost' IDENTIFIED BY 'strong_password';
GRANT SELECT, INSERT, UPDATE, DELETE ON vue_springboot_db.* TO 'app_user'@'localhost';

-- 创建只读用户
CREATE USER 'readonly'@'localhost' IDENTIFIED BY 'readonly_password';
GRANT SELECT ON vue_springboot_db.* TO 'readonly'@'localhost';
```

### 2. 连接限制
```sql
-- 限制连接数
ALTER USER 'app_user'@'localhost' WITH MAX_CONNECTIONS_PER_HOUR 100;
```

## 监控指标

### 1. 关键指标
- 文件总数和总大小
- 用户活跃度
- 热门文件排行
- 存储空间使用情况

### 2. 监控查询
```sql
-- 系统概览
SELECT 
    (SELECT COUNT(*) FROM sys_user WHERE is_enabled = TRUE) as active_users,
    (SELECT COUNT(*) FROM policy_file WHERE is_enabled = TRUE) as total_files,
    (SELECT SUM(file_size) FROM policy_file WHERE is_enabled = TRUE) as total_size,
    (SELECT COUNT(*) FROM file_category WHERE is_enabled = TRUE) as categories;

-- 今日统计
SELECT 
    COUNT(*) as files_uploaded_today,
    SUM(file_size) as size_uploaded_today
FROM policy_file 
WHERE DATE(create_time) = CURDATE();
```

## 故障排除

### 1. 常见问题
- 外键约束错误: 检查关联数据是否存在
- 字符集问题: 确保使用 utf8mb4
- 索引失效: 重建索引或分析表

### 2. 诊断查询
```sql
-- 检查表状态
SHOW TABLE STATUS LIKE 'policy_file';

-- 检查索引使用情况
SHOW INDEX FROM policy_file;

-- 检查外键约束
SELECT * FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE 
WHERE REFERENCED_TABLE_NAME = 'sys_user';
```
