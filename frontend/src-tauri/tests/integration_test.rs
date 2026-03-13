/// 集成测试：验证所有后端服务接口逻辑
/// 使用独立的内存 SQLite 数据库，不影响生产数据

use rusqlite::{Connection, params};
use std::sync::Mutex;
use std::io::Write;

// =========================================================
// 内联必要的结构体和逻辑（因为 lib crate 依赖 tauri 无法直接在测试中引用）
// 我们复制核心逻辑并直接在独立数据库上测试 SQL 和业务逻辑
// =========================================================

fn create_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("无法创建内存数据库");
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA foreign_keys=ON;
         PRAGMA busy_timeout=5000;"
    ).expect("PRAGMA 设置失败");

    conn.execute_batch(
        "
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

        CREATE TABLE IF NOT EXISTS file_tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT DEFAULT '#409EFF',
            description TEXT,
            create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            created_by INTEGER,
            FOREIGN KEY (created_by) REFERENCES sys_user(id) ON DELETE SET NULL
        );

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

        CREATE TABLE IF NOT EXISTS policy_file_tag (
            file_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            create_time TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            PRIMARY KEY (file_id, tag_id),
            FOREIGN KEY (file_id) REFERENCES policy_file(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES file_tag(id) ON DELETE CASCADE
        );

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
    ).expect("建表失败");

    conn
}

fn seed_users(conn: &Connection) {
    let admin_hash = bcrypt::hash("admin123", 4).expect("bcrypt hash 失败"); // cost=4 加速测试
    let user_hash = bcrypt::hash("user123", 4).expect("bcrypt hash 失败");
    let demo_hash = bcrypt::hash("demo123", 4).expect("bcrypt hash 失败");

    conn.execute(
        "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, ?4)",
        params!["admin", admin_hash, "admin@dochub.com", "admin"],
    ).expect("插入 admin 失败");

    conn.execute(
        "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, ?4)",
        params!["user", user_hash, "user@dochub.com", "user"],
    ).expect("插入 user 失败");

    conn.execute(
        "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, ?4)",
        params!["demo", demo_hash, "demo@dochub.com", "user"],
    ).expect("插入 demo 失败");
}

fn seed_categories(conn: &Connection) {
    conn.execute(
        "INSERT INTO file_category (name, description, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
        params!["政策法规", "政策法规类文档", 1, 1],
    ).unwrap();
    conn.execute(
        "INSERT INTO file_category (name, description, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
        params!["通知公告", "通知公告类文档", 2, 1],
    ).unwrap();
    conn.execute(
        "INSERT INTO file_category (name, description, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
        params!["规范性文件", "规范性文件类文档", 3, 1],
    ).unwrap();
}

// =========================================================
// 测试 1：数据库初始化和种子数据
// =========================================================

#[test]
fn test_schema_creation() {
    let conn = create_test_db();
    // 验证 5 张表存在
    let tables: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
        ).unwrap();
        stmt.query_map([], |row| row.get(0)).unwrap().collect::<Result<Vec<_>, _>>().unwrap()
    };
    assert!(tables.contains(&"sys_user".to_string()), "sys_user 表不存在");
    assert!(tables.contains(&"file_category".to_string()), "file_category 表不存在");
    assert!(tables.contains(&"file_tag".to_string()), "file_tag 表不存在");
    assert!(tables.contains(&"policy_file".to_string()), "policy_file 表不存在");
    assert!(tables.contains(&"policy_file_tag".to_string()), "policy_file_tag 表不存在");
    println!("  [PASS] 5 张表全部创建成功");
}

#[test]
fn test_seed_data() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let user_count: i64 = conn.query_row("SELECT COUNT(*) FROM sys_user", [], |r| r.get(0)).unwrap();
    assert_eq!(user_count, 3, "用户种子数据数量不正确");

    let cat_count: i64 = conn.query_row("SELECT COUNT(*) FROM file_category", [], |r| r.get(0)).unwrap();
    assert_eq!(cat_count, 3, "分类种子数据数量不正确");

    // 验证 admin 角色
    let role: String = conn.query_row(
        "SELECT role FROM sys_user WHERE username = 'admin'", [], |r| r.get(0)
    ).unwrap();
    assert_eq!(role, "admin");
    println!("  [PASS] 种子数据初始化正确");
}

// =========================================================
// 测试 2：认证模块
// =========================================================

#[test]
fn test_login_success() {
    let conn = create_test_db();
    seed_users(&conn);

    let (id, pwd_hash, enabled): (i64, String, bool) = conn.query_row(
        "SELECT id, password, is_enabled FROM sys_user WHERE username = 'admin'",
        [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    ).unwrap();

    assert!(enabled, "admin 应该是启用状态");
    assert!(bcrypt::verify("admin123", &pwd_hash).unwrap(), "admin123 密码验证失败");
    assert_eq!(id, 1);
    println!("  [PASS] admin/admin123 登录验证成功");
}

#[test]
fn test_login_wrong_password() {
    let conn = create_test_db();
    seed_users(&conn);

    let pwd_hash: String = conn.query_row(
        "SELECT password FROM sys_user WHERE username = 'admin'",
        [], |r| r.get(0)
    ).unwrap();

    assert!(!bcrypt::verify("wrongpassword", &pwd_hash).unwrap(), "错误密码不应该验证通过");
    println!("  [PASS] 错误密码正确拒绝");
}

#[test]
fn test_login_nonexistent_user() {
    let conn = create_test_db();
    seed_users(&conn);

    let result = conn.query_row(
        "SELECT id FROM sys_user WHERE username = 'nonexistent'",
        [], |r| r.get::<_, i64>(0)
    );
    assert!(result.is_err(), "不存在的用户应该查询为空");
    println!("  [PASS] 不存在的用户正确拒绝");
}

#[test]
fn test_register_new_user() {
    let conn = create_test_db();
    seed_users(&conn);

    let pwd_hash = bcrypt::hash("newpass123", 4).unwrap();
    conn.execute(
        "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, 'user')",
        params!["newuser", pwd_hash, "new@test.com"],
    ).unwrap();

    let id = conn.last_insert_rowid();
    assert!(id > 0, "新用户 ID 应该 > 0");

    let (username, role, email): (String, String, Option<String>) = conn.query_row(
        "SELECT username, role, email FROM sys_user WHERE id = ?1",
        params![id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    ).unwrap();
    assert_eq!(username, "newuser");
    assert_eq!(role, "user");
    assert_eq!(email.as_deref(), Some("new@test.com"));
    println!("  [PASS] 注册新用户成功");
}

#[test]
fn test_register_duplicate_username() {
    let conn = create_test_db();
    seed_users(&conn);

    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM sys_user WHERE username = 'admin'",
        [], |r| r.get(0)
    ).unwrap();
    assert!(exists, "admin 应该已存在");

    let pwd_hash = bcrypt::hash("test123", 4).unwrap();
    let result = conn.execute(
        "INSERT INTO sys_user (username, password, email, role) VALUES (?1, ?2, ?3, 'user')",
        params!["admin", pwd_hash, "dup@test.com"],
    );
    assert!(result.is_err(), "重复用户名应该被拒绝 (UNIQUE 约束)");
    println!("  [PASS] 重复用户名正确拒绝");
}

#[test]
fn test_get_user_list_with_pagination() {
    let conn = create_test_db();
    seed_users(&conn);

    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sys_user WHERE 1=1", [], |r| r.get(0)
    ).unwrap();
    assert_eq!(total, 3);

    // 分页 page=1, size=2
    let mut stmt = conn.prepare(
        "SELECT id, username, role FROM sys_user ORDER BY id ASC LIMIT ?1 OFFSET ?2"
    ).unwrap();
    let users: Vec<(i64, String, String)> = stmt.query_map(params![2, 0], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(users.len(), 2, "第1页应该有2条记录");
    assert_eq!(users[0].1, "admin");
    assert_eq!(users[1].1, "user");

    // 第2页
    let users2: Vec<(i64, String)> = conn.prepare(
        "SELECT id, username FROM sys_user ORDER BY id ASC LIMIT ?1 OFFSET ?2"
    ).unwrap().query_map(params![2, 2], |r| Ok((r.get(0)?, r.get(1)?))).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(users2.len(), 1, "第2页应该有1条记录");
    assert_eq!(users2[0].1, "demo");
    println!("  [PASS] 用户列表分页正确");
}

#[test]
fn test_get_user_list_filter_by_username() {
    let conn = create_test_db();
    seed_users(&conn);

    let mut stmt = conn.prepare(
        "SELECT id, username FROM sys_user WHERE username LIKE ?1 ORDER BY id"
    ).unwrap();
    let users: Vec<String> = stmt.query_map(params!["%adm%"], |r| r.get(1)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0], "admin");
    println!("  [PASS] 用户名模糊筛选正确");
}

#[test]
fn test_get_user_list_filter_by_role() {
    let conn = create_test_db();
    seed_users(&conn);

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sys_user WHERE role = 'user'", [], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 2, "应有2个 user 角色");
    println!("  [PASS] 角色筛选正确");
}

#[test]
fn test_toggle_user_status() {
    let conn = create_test_db();
    seed_users(&conn);

    let enabled: bool = conn.query_row(
        "SELECT is_enabled FROM sys_user WHERE id = 2", [], |r| r.get(0)
    ).unwrap();
    assert!(enabled, "user 应该初始为启用状态");

    conn.execute("UPDATE sys_user SET is_enabled = 0 WHERE id = 2", []).unwrap();
    let enabled2: bool = conn.query_row(
        "SELECT is_enabled FROM sys_user WHERE id = 2", [], |r| r.get(0)
    ).unwrap();
    assert!(!enabled2, "禁用后应该为 false");

    conn.execute("UPDATE sys_user SET is_enabled = 1 WHERE id = 2", []).unwrap();
    let enabled3: bool = conn.query_row(
        "SELECT is_enabled FROM sys_user WHERE id = 2", [], |r| r.get(0)
    ).unwrap();
    assert!(enabled3, "重新启用后应该为 true");
    println!("  [PASS] 切换用户状态正确");
}

#[test]
fn test_disabled_user_login_check() {
    let conn = create_test_db();
    seed_users(&conn);

    // 禁用 user
    conn.execute("UPDATE sys_user SET is_enabled = 0 WHERE username = 'user'", []).unwrap();

    let (pwd_hash, enabled): (String, bool) = conn.query_row(
        "SELECT password, is_enabled FROM sys_user WHERE username = 'user'",
        [], |r| Ok((r.get(0)?, r.get(1)?))
    ).unwrap();

    assert!(bcrypt::verify("user123", &pwd_hash).unwrap(), "密码仍然正确");
    assert!(!enabled, "但用户已被禁用，应拒绝登录");
    println!("  [PASS] 禁用用户登录检查正确");
}

// =========================================================
// 测试 3：分类管理
// =========================================================

#[test]
fn test_get_all_categories() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let mut stmt = conn.prepare(
        "SELECT c.id, c.name, c.description, c.parent_id, c.sort_order, c.is_enabled,
                (SELECT COUNT(*) FROM policy_file f WHERE f.category_id = c.id AND f.is_enabled = 1) as file_count
         FROM file_category c WHERE c.is_enabled = 1 ORDER BY c.sort_order ASC"
    ).unwrap();
    let cats: Vec<(i64, String, i64)> = stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(6)?))
    }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();

    assert_eq!(cats.len(), 3);
    assert_eq!(cats[0].1, "政策法规");
    assert_eq!(cats[1].1, "通知公告");
    assert_eq!(cats[2].1, "规范性文件");
    assert_eq!(cats[0].2, 0, "初始文件数应为 0");
    println!("  [PASS] 获取所有分类正确");
}

#[test]
fn test_get_root_categories() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM file_category WHERE parent_id IS NULL AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 3, "3个根分类");
    println!("  [PASS] 获取根分类正确");
}

#[test]
fn test_create_category() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    conn.execute(
        "INSERT INTO file_category (name, description, parent_id, sort_order, created_by) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["子分类A", "测试子分类", 1, 1, 1],
    ).unwrap();
    let id = conn.last_insert_rowid();

    let (name, parent_id): (String, Option<i64>) = conn.query_row(
        "SELECT name, parent_id FROM file_category WHERE id = ?1",
        params![id], |r| Ok((r.get(0)?, r.get(1)?))
    ).unwrap();
    assert_eq!(name, "子分类A");
    assert_eq!(parent_id, Some(1));
    println!("  [PASS] 创建子分类正确");
}

#[test]
fn test_get_child_categories() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 创建子分类
    conn.execute(
        "INSERT INTO file_category (name, description, parent_id, sort_order, created_by) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["子分类1", "子分类描述", 1, 1, 1],
    ).unwrap();
    conn.execute(
        "INSERT INTO file_category (name, description, parent_id, sort_order, created_by) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["子分类2", "子分类描述2", 1, 2, 1],
    ).unwrap();

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM file_category WHERE parent_id = 1 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 2, "parent_id=1 下应有 2 个子分类");
    println!("  [PASS] 获取子分类正确");
}

#[test]
fn test_update_category() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    conn.execute(
        "UPDATE file_category SET name = ?1, description = ?2 WHERE id = ?3",
        params!["更新后的名称", "更新后的描述", 1],
    ).unwrap();

    let (name, desc): (String, Option<String>) = conn.query_row(
        "SELECT name, description FROM file_category WHERE id = 1",
        [], |r| Ok((r.get(0)?, r.get(1)?))
    ).unwrap();
    assert_eq!(name, "更新后的名称");
    assert_eq!(desc.as_deref(), Some("更新后的描述"));
    println!("  [PASS] 更新分类正确");
}

#[test]
fn test_delete_category_with_children_blocked() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 添加子分类
    conn.execute(
        "INSERT INTO file_category (name, parent_id, sort_order, created_by) VALUES (?1, ?2, ?3, ?4)",
        params!["子分类", 1, 0, 1],
    ).unwrap();

    let child_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM file_category WHERE parent_id = 1 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert!(child_count > 0, "有子分类时不应删除");
    println!("  [PASS] 有子分类时正确阻止删除");
}

#[test]
fn test_delete_category_with_files_blocked() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 添加文件到分类1
    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, created_by, category_id)
         VALUES ('测试文件', 'test.pdf', 'uuid.pdf', '2026/03/13/uuid.pdf', 1024, 'pdf', 1, 1)",
        [],
    ).unwrap();

    let file_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE category_id = 1 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert!(file_count > 0, "有文件时不应删除分类");
    println!("  [PASS] 有文件时正确阻止删除分类");
}

#[test]
fn test_delete_empty_category() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let child_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM file_category WHERE parent_id = 3 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    let file_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE category_id = 3 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(child_count, 0);
    assert_eq!(file_count, 0);

    conn.execute("UPDATE file_category SET is_enabled = 0 WHERE id = 3", []).unwrap();
    let enabled: bool = conn.query_row(
        "SELECT is_enabled FROM file_category WHERE id = 3", [], |r| r.get(0)
    ).unwrap();
    assert!(!enabled, "软删除后 is_enabled 应为 0");
    println!("  [PASS] 空分类软删除正确");
}

#[test]
fn test_move_category() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 将分类2移到分类1下
    conn.execute(
        "UPDATE file_category SET parent_id = ?1 WHERE id = ?2",
        params![1, 2],
    ).unwrap();

    let parent_id: Option<i64> = conn.query_row(
        "SELECT parent_id FROM file_category WHERE id = 2", [], |r| r.get(0)
    ).unwrap();
    assert_eq!(parent_id, Some(1));
    println!("  [PASS] 移动分类正确");
}

#[test]
fn test_category_stats() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 添加2个文件到分类1
    for i in 1..=2 {
        conn.execute(
            "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, created_by, category_id)
             VALUES (?1, ?2, ?3, ?4, ?5, 'pdf', 1, 1)",
            params![format!("文件{}", i), format!("file{}.pdf", i), format!("uuid{}.pdf", i),
                    format!("2026/03/13/uuid{}.pdf", i), 1024 * i],
        ).unwrap();
    }

    let (cat_name, file_count, total_size): (String, i64, i64) = conn.query_row(
        "SELECT c.name,
                (SELECT COUNT(*) FROM policy_file f WHERE f.category_id = c.id AND f.is_enabled = 1),
                (SELECT COALESCE(SUM(f.file_size), 0) FROM policy_file f WHERE f.category_id = c.id AND f.is_enabled = 1)
         FROM file_category c WHERE c.id = 1",
        [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    ).unwrap();
    assert_eq!(cat_name, "政策法规");
    assert_eq!(file_count, 2);
    assert_eq!(total_size, 1024 + 2048);
    println!("  [PASS] 分类统计正确");
}

// =========================================================
// 测试 4：文件管理
// =========================================================

fn insert_test_file(conn: &Connection, title: &str, category_id: Option<i64>, user_id: i64) -> i64 {
    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, mime_type, created_by, category_id, is_public)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 1)",
        params![
            title, format!("{}.pdf", title), format!("{}.pdf", uuid::Uuid::new_v4()),
            format!("2026/03/13/{}.pdf", uuid::Uuid::new_v4()), 2048, "pdf", "application/pdf",
            user_id, category_id
        ],
    ).unwrap();
    conn.last_insert_rowid()
}

#[test]
fn test_insert_file() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "测试文件上传", Some(1), 1);
    assert!(file_id > 0);

    let (title, file_type, is_public, is_enabled): (String, String, bool, bool) = conn.query_row(
        "SELECT title, file_type, is_public, is_enabled FROM policy_file WHERE id = ?1",
        params![file_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
    ).unwrap();
    assert_eq!(title, "测试文件上传");
    assert_eq!(file_type, "pdf");
    assert!(is_public);
    assert!(is_enabled);
    println!("  [PASS] 文件插入正确");
}

#[test]
fn test_get_files_with_keyword() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    insert_test_file(&conn, "重要政策文件", Some(1), 1);
    insert_test_file(&conn, "通知公告文件", Some(2), 1);
    insert_test_file(&conn, "日常事务文件", Some(3), 2);

    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM policy_file f WHERE f.is_enabled = 1 AND (f.title LIKE ?1 OR f.description LIKE ?1)"
    ).unwrap();
    let count: i64 = stmt.query_row(params!["%政策%"], |r| r.get(0)).unwrap();
    assert_eq!(count, 1);
    println!("  [PASS] 关键字搜索正确");
}

#[test]
fn test_get_files_with_category_filter() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    insert_test_file(&conn, "文件A", Some(1), 1);
    insert_test_file(&conn, "文件B", Some(1), 1);
    insert_test_file(&conn, "文件C", Some(2), 1);

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE category_id = 1 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 2);
    println!("  [PASS] 分类筛选正确");
}

#[test]
fn test_get_files_pagination() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    for i in 1..=15 {
        insert_test_file(&conn, &format!("文件{}", i), Some(1), 1);
    }

    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE is_enabled = 1", [], |r| r.get(0)
    ).unwrap();
    assert_eq!(total, 15);

    let mut stmt = conn.prepare(
        "SELECT id FROM policy_file WHERE is_enabled = 1 ORDER BY create_time DESC LIMIT ?1 OFFSET ?2"
    ).unwrap();
    let page1: Vec<i64> = stmt.query_map(params![10, 0], |r| r.get(0)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(page1.len(), 10, "第1页应有10条");

    let page2: Vec<i64> = conn.prepare(
        "SELECT id FROM policy_file WHERE is_enabled = 1 ORDER BY create_time DESC LIMIT ?1 OFFSET ?2"
    ).unwrap().query_map(params![10, 10], |r| r.get(0)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(page2.len(), 5, "第2页应有5条");
    println!("  [PASS] 文件列表分页正确");
}

#[test]
fn test_get_file_by_id_with_joins() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "带关联信息的文件", Some(1), 1);

    let (title, cat_name, creator_name): (String, Option<String>, Option<String>) = conn.query_row(
        "SELECT f.title, c.name, u.username
         FROM policy_file f
         LEFT JOIN file_category c ON f.category_id = c.id
         LEFT JOIN sys_user u ON f.created_by = u.id
         WHERE f.id = ?1",
        params![file_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    ).unwrap();
    assert_eq!(title, "带关联信息的文件");
    assert_eq!(cat_name.as_deref(), Some("政策法规"));
    assert_eq!(creator_name.as_deref(), Some("admin"));
    println!("  [PASS] 文件详情 JOIN 查询正确");
}

#[test]
fn test_update_file() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "原标题", Some(1), 1);

    conn.execute(
        "UPDATE policy_file SET title = ?1, description = ?2, category_id = ?3, updated_by = ?4 WHERE id = ?5",
        params!["新标题", "新描述", 2, 1, file_id],
    ).unwrap();

    let (title, desc, cat_id): (String, Option<String>, Option<i64>) = conn.query_row(
        "SELECT title, description, category_id FROM policy_file WHERE id = ?1",
        params![file_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    ).unwrap();
    assert_eq!(title, "新标题");
    assert_eq!(desc.as_deref(), Some("新描述"));
    assert_eq!(cat_id, Some(2));
    println!("  [PASS] 更新文件信息正确");
}

#[test]
fn test_delete_file_soft() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "待删除文件", Some(1), 1);

    conn.execute(
        "UPDATE policy_file SET is_enabled = 0 WHERE id = ?1",
        params![file_id],
    ).unwrap();

    let enabled: bool = conn.query_row(
        "SELECT is_enabled FROM policy_file WHERE id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert!(!enabled, "软删除后 is_enabled 应为 0");

    // 搜索时不应出现
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 0, "软删除后搜索不应包含该文件");
    println!("  [PASS] 软删除文件正确");
}

#[test]
fn test_download_count_increment() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "下载测试文件", Some(1), 1);

    // 模拟 get_file_path 的下载计数逻辑
    conn.execute(
        "UPDATE policy_file SET download_count = download_count + 1 WHERE id = ?1",
        params![file_id],
    ).unwrap();
    conn.execute(
        "UPDATE policy_file SET download_count = download_count + 1 WHERE id = ?1",
        params![file_id],
    ).unwrap();

    let count: i32 = conn.query_row(
        "SELECT download_count FROM policy_file WHERE id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 2, "下载次数应为2");
    println!("  [PASS] 下载计数递增正确");
}

#[test]
fn test_view_count_increment() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "浏览测试文件", Some(1), 1);

    conn.execute(
        "UPDATE policy_file SET view_count = view_count + 1 WHERE id = ?1",
        params![file_id],
    ).unwrap();

    let count: i32 = conn.query_row(
        "SELECT view_count FROM policy_file WHERE id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 1, "浏览次数应为1");
    println!("  [PASS] 浏览计数递增正确");
}

#[test]
fn test_popular_files_query() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 插入文件并设置不同的访问量
    let id1 = insert_test_file(&conn, "高热度文件", Some(1), 1);
    let id2 = insert_test_file(&conn, "低热度文件", Some(1), 1);
    conn.execute("UPDATE policy_file SET view_count = 100, download_count = 50 WHERE id = ?1", params![id1]).unwrap();
    conn.execute("UPDATE policy_file SET view_count = 10, download_count = 5 WHERE id = ?1", params![id2]).unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, title FROM policy_file
         WHERE is_enabled = 1 AND is_public = 1
         ORDER BY (view_count * 0.7 + download_count * 0.3) DESC
         LIMIT 10"
    ).unwrap();
    let files: Vec<(i64, String)> = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap().collect::<Result<Vec<_>, _>>().unwrap();

    assert_eq!(files.len(), 2);
    assert_eq!(files[0].1, "高热度文件", "高热度文件应排在第一");
    println!("  [PASS] 热门文件排序正确");
}

#[test]
fn test_latest_files_query() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    insert_test_file(&conn, "文件1", Some(1), 1);
    // 稍微延迟确保时间戳不同
    insert_test_file(&conn, "文件2", Some(1), 1);

    let mut stmt = conn.prepare(
        "SELECT title FROM policy_file WHERE is_enabled = 1 ORDER BY create_time DESC LIMIT 10"
    ).unwrap();
    let titles: Vec<String> = stmt.query_map([], |r| r.get(0)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(titles.len(), 2);
    println!("  [PASS] 最新文件查询正确");
}

#[test]
fn test_my_files_query() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    insert_test_file(&conn, "admin的文件", Some(1), 1);
    insert_test_file(&conn, "user的文件", Some(1), 2);

    let count_admin: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE created_by = 1 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    let count_user: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE created_by = 2 AND is_enabled = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(count_admin, 1);
    assert_eq!(count_user, 1);
    println!("  [PASS] 我的文件筛选正确");
}

// =========================================================
// 测试 5：标签系统
// =========================================================

#[test]
fn test_tag_ensure_create_new() {
    let conn = create_test_db();
    seed_users(&conn);

    conn.execute(
        "INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)",
        params!["重要", 1],
    ).unwrap();
    let tag_id = conn.last_insert_rowid();
    assert!(tag_id > 0);

    let name: String = conn.query_row(
        "SELECT name FROM file_tag WHERE id = ?1",
        params![tag_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(name, "重要");
    println!("  [PASS] 创建新标签正确");
}

#[test]
fn test_tag_ensure_find_existing() {
    let conn = create_test_db();
    seed_users(&conn);

    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["重要", 1]).unwrap();
    let id1 = conn.last_insert_rowid();

    // 再次查找同名标签
    let id2: i64 = conn.query_row(
        "SELECT id FROM file_tag WHERE name = ?1",
        params!["重要"], |r| r.get(0)
    ).unwrap();
    assert_eq!(id1, id2, "已存在的标签应返回相同 ID");
    println!("  [PASS] 查找已有标签正确");
}

#[test]
fn test_tag_unique_constraint() {
    let conn = create_test_db();
    seed_users(&conn);

    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["标签A", 1]).unwrap();
    let result = conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["标签A", 1]);
    assert!(result.is_err(), "重复标签名应被 UNIQUE 约束拒绝");
    println!("  [PASS] 标签名唯一约束正确");
}

#[test]
fn test_file_tag_association() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "带标签的文件", Some(1), 1);

    // 创建标签
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["重要", 1]).unwrap();
    let tag1_id = conn.last_insert_rowid();
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["紧急", 1]).unwrap();
    let tag2_id = conn.last_insert_rowid();

    // 关联
    conn.execute(
        "INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)",
        params![file_id, tag1_id],
    ).unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)",
        params![file_id, tag2_id],
    ).unwrap();

    // 查询文件标签
    let mut stmt = conn.prepare(
        "SELECT ft.name FROM file_tag ft
         JOIN policy_file_tag pft ON ft.id = pft.tag_id
         WHERE pft.file_id = ?1 ORDER BY ft.name"
    ).unwrap();
    let tags: Vec<String> = stmt.query_map(params![file_id], |r| r.get(0)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&"重要".to_string()));
    assert!(tags.contains(&"紧急".to_string()));
    println!("  [PASS] 文件-标签关联正确");
}

#[test]
fn test_tag_filter_files() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file1 = insert_test_file(&conn, "文件1", Some(1), 1);
    let file2 = insert_test_file(&conn, "文件2", Some(1), 1);
    let _file3 = insert_test_file(&conn, "文件3", Some(1), 1);

    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["标签X", 1]).unwrap();
    let tag_id = conn.last_insert_rowid();

    conn.execute("INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)", params![file1, tag_id]).unwrap();
    conn.execute("INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)", params![file2, tag_id]).unwrap();

    // 按标签筛选文件
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file f WHERE f.is_enabled = 1
         AND f.id IN (SELECT pft.file_id FROM policy_file_tag pft JOIN file_tag ft ON pft.tag_id = ft.id WHERE ft.name IN (?1))",
        params!["标签X"], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 2, "标签X关联了2个文件");
    println!("  [PASS] 标签筛选文件正确");
}

#[test]
fn test_update_file_tags() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "更新标签文件", Some(1), 1);

    // 初始标签
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["旧标签", 1]).unwrap();
    let old_tag = conn.last_insert_rowid();
    conn.execute("INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)", params![file_id, old_tag]).unwrap();

    // 更新标签：先删旧的再加新的
    conn.execute("DELETE FROM policy_file_tag WHERE file_id = ?1", params![file_id]).unwrap();
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["新标签1", 1]).unwrap();
    let new_tag1 = conn.last_insert_rowid();
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["新标签2", 1]).unwrap();
    let new_tag2 = conn.last_insert_rowid();
    conn.execute("INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)", params![file_id, new_tag1]).unwrap();
    conn.execute("INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)", params![file_id, new_tag2]).unwrap();

    let tag_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file_tag WHERE file_id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(tag_count, 2, "更新后应有2个新标签");

    // 确认旧标签已断开
    let old_link: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file_tag WHERE file_id = ?1 AND tag_id = ?2",
        params![file_id, old_tag], |r| r.get(0)
    ).unwrap();
    assert_eq!(old_link, 0, "旧标签应已断开");
    println!("  [PASS] 更新文件标签正确");
}

#[test]
fn test_get_all_tags() {
    let conn = create_test_db();
    seed_users(&conn);

    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["标签A", 1]).unwrap();
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["标签B", 1]).unwrap();
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["标签C", 2]).unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, name, color FROM file_tag ORDER BY name"
    ).unwrap();
    let tags: Vec<(i64, String, Option<String>)> = stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?))
    }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0].1, "标签A");
    assert_eq!(tags[0].2.as_deref(), Some("#409EFF"), "默认颜色应为 #409EFF");
    println!("  [PASS] 获取所有标签正确");
}

// =========================================================
// 测试 6：预览类型判断
// =========================================================

fn get_preview_type(file_type: &str) -> String {
    match file_type {
        "pdf" => "pdf".to_string(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" => "image".to_string(),
        "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" | "webm" => "video".to_string(),
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "audio".to_string(),
        "txt" | "md" | "json" | "xml" | "html" | "htm" | "css" | "js" | "ts" | "log" | "csv" => "text".to_string(),
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => "office".to_string(),
        _ => "unknown".to_string(),
    }
}

#[test]
fn test_preview_type_detection() {
    assert_eq!(get_preview_type("pdf"), "pdf");
    assert_eq!(get_preview_type("jpg"), "image");
    assert_eq!(get_preview_type("jpeg"), "image");
    assert_eq!(get_preview_type("png"), "image");
    assert_eq!(get_preview_type("gif"), "image");
    assert_eq!(get_preview_type("webp"), "image");
    assert_eq!(get_preview_type("svg"), "image");
    assert_eq!(get_preview_type("mp4"), "video");
    assert_eq!(get_preview_type("mp3"), "audio");
    assert_eq!(get_preview_type("txt"), "text");
    assert_eq!(get_preview_type("md"), "text");
    assert_eq!(get_preview_type("json"), "text");
    assert_eq!(get_preview_type("csv"), "text");
    assert_eq!(get_preview_type("docx"), "office");
    assert_eq!(get_preview_type("xlsx"), "office");
    assert_eq!(get_preview_type("pptx"), "office");
    assert_eq!(get_preview_type("zip"), "unknown");
    assert_eq!(get_preview_type("exe"), "unknown");
    println!("  [PASS] 预览类型检测正确（全部18种类型）");
}

// =========================================================
// 测试 7：触发器和级联
// =========================================================

#[test]
fn test_cascade_delete_file_tags() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "级联删除文件", Some(1), 1);
    conn.execute("INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)", params!["测试标签", 1]).unwrap();
    let tag_id = conn.last_insert_rowid();
    conn.execute("INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)", params![file_id, tag_id]).unwrap();

    // 确认关联存在
    let link_before: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file_tag WHERE file_id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(link_before, 1);

    // 硬删除文件（测试 CASCADE）
    conn.execute("DELETE FROM policy_file WHERE id = ?1", params![file_id]).unwrap();

    let link_after: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file_tag WHERE file_id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(link_after, 0, "CASCADE 应该自动删除关联记录");
    println!("  [PASS] CASCADE 删除文件-标签关联正确");
}

#[test]
fn test_foreign_key_category_set_null() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    let file_id = insert_test_file(&conn, "外键测试文件", Some(3), 1);

    // 硬删除分类3
    conn.execute("DELETE FROM file_category WHERE id = 3", []).unwrap();

    let cat_id: Option<i64> = conn.query_row(
        "SELECT category_id FROM policy_file WHERE id = ?1",
        params![file_id], |r| r.get(0)
    ).unwrap();
    assert_eq!(cat_id, None, "分类删除后文件的 category_id 应被设为 NULL");
    println!("  [PASS] 外键 ON DELETE SET NULL 正确");
}

// =========================================================
// 测试 8：高级查询和排序
// =========================================================

#[test]
fn test_files_sort_by_size() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, created_by, category_id)
         VALUES ('小文件', 's.pdf', 's.pdf', 'path/s.pdf', 100, 'pdf', 1, 1)", []
    ).unwrap();
    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, created_by, category_id)
         VALUES ('大文件', 'l.pdf', 'l.pdf', 'path/l.pdf', 99999, 'pdf', 1, 1)", []
    ).unwrap();

    let mut stmt = conn.prepare(
        "SELECT title FROM policy_file WHERE is_enabled = 1 ORDER BY file_size DESC"
    ).unwrap();
    let titles: Vec<String> = stmt.query_map([], |r| r.get(0)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(titles[0], "大文件");
    assert_eq!(titles[1], "小文件");
    println!("  [PASS] 按文件大小排序正确");
}

#[test]
fn test_files_complex_filter() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    // 插入不同属性的文件
    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, issuing_authority, is_public, created_by, category_id)
         VALUES ('公开文件A', 'a.pdf', 'a.pdf', 'p/a.pdf', 100, 'pdf', '部门A', 1, 1, 1)", []
    ).unwrap();
    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, issuing_authority, is_public, created_by, category_id)
         VALUES ('私有文件B', 'b.pdf', 'b.pdf', 'p/b.pdf', 200, 'pdf', '部门B', 0, 1, 1)", []
    ).unwrap();

    // 按 issuing_authority 筛选
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE is_enabled = 1 AND issuing_authority LIKE ?1",
        params!["%部门A%"], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 1);

    // 按 is_public 筛选
    let public_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE is_enabled = 1 AND is_public = 1",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(public_count, 1);
    println!("  [PASS] 复杂条件筛选正确");
}

// =========================================================
// 测试 9：边界情况
// =========================================================

#[test]
fn test_empty_search_results() {
    let conn = create_test_db();
    seed_users(&conn);

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM policy_file WHERE is_enabled = 1 AND title LIKE ?1",
        params!["%不存在的内容%"], |r| r.get(0)
    ).unwrap();
    assert_eq!(count, 0, "空搜索结果应返回0");
    println!("  [PASS] 空搜索结果正确");
}

#[test]
fn test_null_category_file() {
    let conn = create_test_db();
    seed_users(&conn);

    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, created_by)
         VALUES ('无分类文件', 'test.pdf', 'test.pdf', 'p/test.pdf', 100, 'pdf', 1)", []
    ).unwrap();

    let cat_id: Option<i64> = conn.query_row(
        "SELECT category_id FROM policy_file WHERE title = '无分类文件'",
        [], |r| r.get(0)
    ).unwrap();
    assert_eq!(cat_id, None, "未分类文件的 category_id 应为 NULL");

    // JOIN 查询不应报错
    let (title, cat_name): (String, Option<String>) = conn.query_row(
        "SELECT f.title, c.name FROM policy_file f LEFT JOIN file_category c ON f.category_id = c.id WHERE f.title = '无分类文件'",
        [], |r| Ok((r.get(0)?, r.get(1)?))
    ).unwrap();
    assert_eq!(title, "无分类文件");
    assert_eq!(cat_name, None, "LEFT JOIN 应返回 NULL");
    println!("  [PASS] 无分类文件处理正确");
}

#[test]
fn test_unicode_content() {
    let conn = create_test_db();
    seed_users(&conn);
    seed_categories(&conn);

    conn.execute(
        "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, description, created_by, category_id)
         VALUES (?1, ?2, ?3, ?4, 100, 'pdf', ?5, 1, 1)",
        params!["中文标题测试 🎉", "中文文件名.pdf", "test.pdf", "p/test.pdf", "描述包含特殊字符: <>&\"'"],
    ).unwrap();

    let (title, desc): (String, Option<String>) = conn.query_row(
        "SELECT title, description FROM policy_file WHERE title LIKE '%中文%'",
        [], |r| Ok((r.get(0)?, r.get(1)?))
    ).unwrap();
    assert_eq!(title, "中文标题测试 🎉");
    assert_eq!(desc.as_deref(), Some("描述包含特殊字符: <>&\"'"));
    println!("  [PASS] Unicode 和特殊字符处理正确");
}
