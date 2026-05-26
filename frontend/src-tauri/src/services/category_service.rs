use rusqlite::params;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::{FileCategory, CreateCategoryRequest, UpdateCategoryRequest, CategoryStats};

pub struct CategoryService;

impl CategoryService {
    pub fn get_all_categories(db: &Database) -> Result<Vec<FileCategory>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "WITH RECURSIVE descendants(root_id, descendant_id) AS (\
                SELECT id, id FROM file_category WHERE is_enabled = 1 \
                UNION ALL \
                SELECT d.root_id, c.id FROM file_category c \
                JOIN descendants d ON c.parent_id = d.descendant_id \
                WHERE c.is_enabled = 1\
             ) \
             SELECT c.id, c.name, c.description, c.parent_id, c.sort_order, c.is_enabled,
                    c.create_time, c.update_time, c.created_by,
                    (SELECT COUNT(*) FROM policy_file f
                     WHERE f.is_enabled = 1
                       AND f.category_id IN (SELECT descendant_id FROM descendants WHERE root_id = c.id)
                    ) as file_count
             FROM file_category c
             WHERE c.is_enabled = 1
             ORDER BY c.sort_order ASC, c.id ASC"
        )?;

        let categories = stmt.query_map([], |row| {
            Ok(FileCategory {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
                enabled: row.get(5)?,
                create_time: row.get(6)?,
                update_time: row.get(7)?,
                created_by: row.get(8)?,
                children: Vec::new(),
                file_count: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn get_root_categories(db: &Database) -> Result<Vec<FileCategory>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "WITH RECURSIVE descendants(root_id, descendant_id) AS (\
                SELECT id, id FROM file_category WHERE is_enabled = 1 \
                UNION ALL \
                SELECT d.root_id, c.id FROM file_category c \
                JOIN descendants d ON c.parent_id = d.descendant_id \
                WHERE c.is_enabled = 1\
             ) \
             SELECT c.id, c.name, c.description, c.parent_id, c.sort_order, c.is_enabled,
                    c.create_time, c.update_time, c.created_by,
                    (SELECT COUNT(*) FROM policy_file f
                     WHERE f.is_enabled = 1
                       AND f.category_id IN (SELECT descendant_id FROM descendants WHERE root_id = c.id)
                    ) as file_count
             FROM file_category c
             WHERE c.parent_id IS NULL AND c.is_enabled = 1
             ORDER BY c.sort_order ASC"
        )?;

        let categories = stmt.query_map([], |row| {
            Ok(FileCategory {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
                enabled: row.get(5)?,
                create_time: row.get(6)?,
                update_time: row.get(7)?,
                created_by: row.get(8)?,
                children: Vec::new(),
                file_count: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn get_child_categories(db: &Database, parent_id: i64) -> Result<Vec<FileCategory>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "WITH RECURSIVE descendants(root_id, descendant_id) AS (\
                SELECT id, id FROM file_category WHERE is_enabled = 1 \
                UNION ALL \
                SELECT d.root_id, c.id FROM file_category c \
                JOIN descendants d ON c.parent_id = d.descendant_id \
                WHERE c.is_enabled = 1\
             ) \
             SELECT c.id, c.name, c.description, c.parent_id, c.sort_order, c.is_enabled,
                    c.create_time, c.update_time, c.created_by,
                    (SELECT COUNT(*) FROM policy_file f
                     WHERE f.is_enabled = 1
                       AND f.category_id IN (SELECT descendant_id FROM descendants WHERE root_id = c.id)
                    ) as file_count
             FROM file_category c
             WHERE c.parent_id = ?1 AND c.is_enabled = 1
             ORDER BY c.sort_order ASC"
        )?;

        let categories = stmt.query_map(params![parent_id], |row| {
            Ok(FileCategory {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
                enabled: row.get(5)?,
                create_time: row.get(6)?,
                update_time: row.get(7)?,
                created_by: row.get(8)?,
                children: Vec::new(),
                file_count: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn create_category(db: &Database, req: &CreateCategoryRequest, user_id: i64) -> Result<FileCategory, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO file_category (name, description, parent_id, sort_order, created_by) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![req.name, req.description, req.parent_id, req.sort_order, user_id],
        )?;

        let id = conn.last_insert_rowid();

        conn.query_row(
            "SELECT id, name, description, parent_id, sort_order, is_enabled, create_time, update_time, created_by
             FROM file_category WHERE id = ?1",
            params![id],
            |row| {
                Ok(FileCategory {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    parent_id: row.get(3)?,
                    sort_order: row.get(4)?,
                    enabled: row.get(5)?,
                    create_time: row.get(6)?,
                    update_time: row.get(7)?,
                    created_by: row.get(8)?,
                    children: Vec::new(),
                    file_count: 0,
                })
            },
        ).map_err(|e| AppError::Database(e.to_string()))
    }

    pub fn update_category(db: &Database, id: i64, req: &UpdateCategoryRequest) -> Result<(), AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut updates = Vec::new();
        let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref name) = req.name {
            bind_values.push(Box::new(name.clone()));
            updates.push(format!("name = ?{}", bind_values.len()));
        }
        if let Some(ref desc) = req.description {
            bind_values.push(Box::new(desc.clone()));
            updates.push(format!("description = ?{}", bind_values.len()));
        }
        if let Some(parent_id) = req.parent_id {
            bind_values.push(Box::new(parent_id));
            updates.push(format!("parent_id = ?{}", bind_values.len()));
        }
        if let Some(sort_order) = req.sort_order {
            bind_values.push(Box::new(sort_order));
            updates.push(format!("sort_order = ?{}", bind_values.len()));
        }
        if let Some(enabled) = req.enabled {
            bind_values.push(Box::new(enabled));
            updates.push(format!("is_enabled = ?{}", bind_values.len()));
        }

        if updates.is_empty() {
            return Ok(());
        }

        bind_values.push(Box::new(id));
        let sql = format!(
            "UPDATE file_category SET {} WHERE id = ?{}",
            updates.join(", "),
            bind_values.len()
        );

        let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
        conn.execute(&sql, refs.as_slice())?;

        Ok(())
    }

    pub fn delete_category(db: &Database, id: i64) -> Result<(), AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        // 检查子分类
        let child_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM file_category WHERE parent_id = ?1 AND is_enabled = 1",
            params![id],
            |row| row.get(0),
        )?;

        if child_count > 0 {
            return Err(AppError::BadRequest("该分类下还有子分类，请先删除子分类".to_string()));
        }

        // 检查文件
        let file_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM policy_file WHERE category_id = ?1 AND is_enabled = 1",
            params![id],
            |row| row.get(0),
        )?;

        if file_count > 0 {
            return Err(AppError::BadRequest("该分类下还有文件，请先移动或删除文件".to_string()));
        }

        // 软删除
        conn.execute(
            "UPDATE file_category SET is_enabled = 0 WHERE id = ?1",
            params![id],
        )?;

        Ok(())
    }

    pub fn move_category(db: &Database, id: i64, new_parent_id: Option<i64>) -> Result<(), AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE file_category SET parent_id = ?1 WHERE id = ?2",
            params![new_parent_id, id],
        )?;

        Ok(())
    }

    pub fn get_category_stats(db: &Database, id: i64) -> Result<CategoryStats, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.query_row(
            "WITH RECURSIVE cat_tree(id) AS (\
                SELECT ?1 \
                UNION ALL \
                SELECT c.id FROM file_category c \
                JOIN cat_tree t ON c.parent_id = t.id \
                WHERE c.is_enabled = 1\
             ) \
             SELECT c.id, c.name,
                    (SELECT COUNT(*) FROM policy_file f
                     WHERE f.is_enabled = 1 AND f.category_id IN (SELECT id FROM cat_tree)),
                    (SELECT COALESCE(SUM(f.file_size), 0) FROM policy_file f
                     WHERE f.is_enabled = 1 AND f.category_id IN (SELECT id FROM cat_tree))
             FROM file_category c WHERE c.id = ?1",
            params![id],
            |row| {
                Ok(CategoryStats {
                    category_id: row.get(0)?,
                    category_name: row.get(1)?,
                    file_count: row.get(2)?,
                    total_size: row.get(3)?,
                })
            },
        ).map_err(|_| AppError::NotFound("分类不存在".to_string()))
    }
}
