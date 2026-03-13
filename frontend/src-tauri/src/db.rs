use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::errors::AppError;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, AppError> {
        let db_path = get_db_path()?;

        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)
            .map_err(|e| AppError::Database(format!("无法打开数据库: {}", e)))?;

        // 启用 WAL 模式和外键约束
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA foreign_keys=ON;
             PRAGMA busy_timeout=5000;"
        )?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        db.init_schema()?;
        db.init_seed_data()?;

        Ok(db)
    }

    fn init_schema(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute_batch(
            "
            -- 用户表
            CREATE TABLE IF NOT EXISTS sys_user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                email TEXT,
                role TEXT NOT NULL DEFAULT 'user',
                avatar TEXT,
                is_enabled INTEGER NOT NULL DEFAULT 1,
                create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                update_time TEXT DEFAULT (datetime('now', 'localtime'))
            );

            CREATE INDEX IF NOT EXISTS idx_user_username ON sys_user(username);
            CREATE INDEX IF NOT EXISTS idx_user_role ON sys_user(role);

            -- 文件分类表
            CREATE TABLE IF NOT EXISTS file_category (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                parent_id INTEGER,
                sort_order INTEGER NOT NULL DEFAULT 0,
                is_enabled INTEGER NOT NULL DEFAULT 1,
                create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                update_time TEXT DEFAULT (datetime('now', 'localtime')),
                created_by INTEGER,
                FOREIGN KEY (parent_id) REFERENCES file_category(id) ON DELETE SET NULL,
                FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL
            );

            CREATE INDEX IF NOT EXISTS idx_category_parent ON file_category(parent_id);
            CREATE INDEX IF NOT EXISTS idx_category_sort ON file_category(sort_order);

            -- 文件标签表
            CREATE TABLE IF NOT EXISTS file_tag (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                color TEXT DEFAULT '#409EFF',
                description TEXT,
                create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                created_by INTEGER,
                FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL
            );

            -- 政策文件表
            CREATE TABLE IF NOT EXISTS policy_file (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                original_name TEXT NOT NULL,
                file_name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                file_type TEXT NOT NULL,
                mime_type TEXT,
                description TEXT,
                document_number TEXT,
                issue_date TEXT,
                effective_date TEXT,
                issuing_authority TEXT,
                download_count INTEGER NOT NULL DEFAULT 0,
                view_count INTEGER NOT NULL DEFAULT 0,
                is_public INTEGER NOT NULL DEFAULT 1,
                is_enabled INTEGER NOT NULL DEFAULT 1,
                create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                update_time TEXT DEFAULT (datetime('now', 'localtime')),
                created_by INTEGER NOT NULL,
                updated_by INTEGER,
                category_id INTEGER,
                FOREIGN KEY (category_id) REFERENCES file_category(id) ON DELETE SET NULL,
                FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE RESTRICT,
                FOREIGN KEY (updated_by) REFERENCES sys_user(id) ON DELETE SET NULL
            );

            CREATE INDEX IF NOT EXISTS idx_file_title ON policy_file(title);
            CREATE INDEX IF NOT EXISTS idx_file_type ON policy_file(file_type);
            CREATE INDEX IF NOT EXISTS idx_file_category ON policy_file(category_id);
            CREATE INDEX IF NOT EXISTS idx_file_created_by ON policy_file(created_by);
            CREATE INDEX IF NOT EXISTS idx_file_create_time ON policy_file(create_time);
            CREATE INDEX IF NOT EXISTS idx_file_enabled ON policy_file(is_enabled);

            -- 文件标签关联表
            CREATE TABLE IF NOT EXISTS policy_file_tag (
                file_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                PRIMARY KEY (file_id, tag_id),
                FOREIGN KEY (file_id) REFERENCES policy_file(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES file_tag(id) ON DELETE CASCADE
            );

            -- update_time 触发器
            CREATE TRIGGER IF NOT EXISTS trg_user_update
            AFTER UPDATE ON sys_user
            BEGIN
                UPDATE sys_user SET update_time = datetime('now', 'localtime') WHERE id = NEW.id;
            END;

            CREATE TRIGGER IF NOT EXISTS trg_category_update
            AFTER UPDATE ON file_category
            BEGIN
                UPDATE file_category SET update_time = datetime('now', 'localtime') WHERE id = NEW.id;
            END;

            CREATE TRIGGER IF NOT EXISTS trg_file_update
            AFTER UPDATE ON policy_file
            BEGIN
                UPDATE policy_file SET update_time = datetime('now', 'localtime') WHERE id = NEW.id;
            END;
            "
        )?;

        Ok(())
    }

    fn init_seed_data(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        // 检查是否已有 admin 用户
        let admin_exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM sys_user WHERE username = 'admin'",
            [],
            |row| row.get(0),
        )?;

        if !admin_exists {
            // 创建默认用户 (bcrypt 加密)
            let admin_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)?;
            let user_hash = bcrypt::hash("user123", bcrypt::DEFAULT_COST)?;
            let demo_hash = bcrypt::hash("demo123", bcrypt::DEFAULT_COST)?;

            conn.execute(
                "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, ?4)",
                params!["admin", admin_hash, "admin@dochub.com", "admin"],
            )?;

            conn.execute(
                "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, ?4)",
                params!["user", user_hash, "user@dochub.com", "user"],
            )?;

            conn.execute(
                "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, ?4)",
                params!["demo", demo_hash, "demo@dochub.com", "user"],
            )?;

            // 创建默认分类
            conn.execute(
                "INSERT OR IGNORE INTO file_category (name, description, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
                params!["政策法规", "政策法规类文档", 1, 1],
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO file_category (name, description, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
                params!["通知公告", "通知公告类文档", 2, 1],
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO file_category (name, description, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
                params!["规范性文件", "规范性文件类文档", 3, 1],
            )?;

            log::info!("种子数据初始化完成");
        }

        Ok(())
    }
}

/// 获取数据库文件路径: ~/Documents/DocHub/data/dochub.db
fn get_db_path() -> Result<PathBuf, AppError> {
    let doc_dir = dirs::document_dir()
        .ok_or_else(|| AppError::Internal("无法获取文档目录".to_string()))?;
    Ok(doc_dir.join("DocHub").join("data").join("dochub.db"))
}

/// 获取上传文件存储根目录: ~/Documents/DocHub/uploads/
pub fn get_uploads_dir() -> Result<PathBuf, AppError> {
    let doc_dir = dirs::document_dir()
        .ok_or_else(|| AppError::Internal("无法获取文档目录".to_string()))?;
    let uploads = doc_dir.join("DocHub").join("uploads");
    std::fs::create_dir_all(&uploads)?;
    Ok(uploads)
}
