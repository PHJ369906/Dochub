use rusqlite::params;
use std::path::Path;

use crate::db::{Database, get_uploads_dir};
use crate::errors::AppError;
use crate::models::{PolicyFile, FileTag, UploadFileRequest, UpdateFileRequest, FileSearchParams, PageResult, FileCategoryBrief, UserBrief};

pub struct FileService;

impl FileService {
    pub fn upload_file(db: &Database, req: &UploadFileRequest, user_id: i64) -> Result<PolicyFile, AppError> {
        let source_path = Path::new(&req.file_path);
        if !source_path.exists() {
            return Err(AppError::BadRequest("文件不存在".to_string()));
        }

        let original_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file_ext = source_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let file_size = std::fs::metadata(source_path)?.len() as i64;

        let mime_type = mime_guess::from_path(source_path)
            .first()
            .map(|m| m.to_string());

        // 生成存储路径
        let now = chrono::Local::now();
        let date_path = now.format("%Y/%m/%d").to_string();
        let uuid_name = format!("{}.{}", uuid::Uuid::new_v4(), file_ext);
        let uploads_dir = get_uploads_dir()?;
        let target_dir = uploads_dir.join(&date_path);
        std::fs::create_dir_all(&target_dir)?;
        let target_path = target_dir.join(&uuid_name);

        // 复制文件
        std::fs::copy(source_path, &target_path)?;

        let file_path_str = format!("{}/{}", date_path, uuid_name);
        let title = req.title.clone().unwrap_or_else(|| {
            original_name
                .rsplit_once('.')
                .map(|(name, _)| name.to_string())
                .unwrap_or(original_name.clone())
        });

        let is_public = req.is_public.unwrap_or(true);

        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size, file_type, mime_type,
             description, document_number, issue_date, effective_date, issuing_authority, is_public, created_by, category_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                title, original_name, uuid_name, file_path_str, file_size, file_ext, mime_type,
                req.description, req.document_number, req.issue_date, req.effective_date,
                req.issuing_authority, is_public, user_id, req.category_id
            ],
        )?;

        let file_id = conn.last_insert_rowid();

        // 处理标签
        if let Some(ref tags) = req.tags {
            for tag_name in tags {
                let tag_id = Self::ensure_tag(&conn, tag_name, user_id)?;
                conn.execute(
                    "INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)",
                    params![file_id, tag_id],
                )?;
            }
        }

        drop(conn);
        Self::get_file_by_id(db, file_id)
    }

    pub fn get_files(db: &Database, params: &FileSearchParams) -> Result<PageResult<PolicyFile>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let page = params.page.unwrap_or(0);
        let size = params.size.unwrap_or(10).max(1);
        let offset = page * size;

        let mut conditions = vec!["f.is_enabled = 1".to_string()];
        let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref keyword) = params.keyword {
            if !keyword.is_empty() {
                bind_values.push(Box::new(format!("%{}%", keyword)));
                let idx = bind_values.len();
                conditions.push(format!("(f.title LIKE ?{} OR f.description LIKE ?{} OR f.original_name LIKE ?{})", idx, idx, idx));
            }
        }

        if let Some(category_id) = params.category_id {
            bind_values.push(Box::new(category_id));
            conditions.push(format!("f.category_id = ?{}", bind_values.len()));
        }

        if let Some(ref authority) = params.issuing_authority {
            if !authority.is_empty() {
                bind_values.push(Box::new(format!("%{}%", authority)));
                conditions.push(format!("f.issuing_authority LIKE ?{}", bind_values.len()));
            }
        }

        if let Some(ref start_date) = params.start_date {
            if !start_date.is_empty() {
                bind_values.push(Box::new(start_date.clone()));
                conditions.push(format!("f.create_time >= ?{}", bind_values.len()));
            }
        }

        if let Some(ref end_date) = params.end_date {
            if !end_date.is_empty() {
                bind_values.push(Box::new(end_date.clone()));
                conditions.push(format!("f.create_time <= ?{}", bind_values.len()));
            }
        }

        if let Some(is_public) = params.is_public {
            bind_values.push(Box::new(is_public));
            conditions.push(format!("f.is_public = ?{}", bind_values.len()));
        }

        // 标签筛选
        if let Some(ref tags) = params.tags {
            let tag_names: Vec<&String> = tags.iter().filter(|t| !t.is_empty()).collect();
            if !tag_names.is_empty() {
                let placeholders: Vec<String> = tag_names.iter().enumerate().map(|(i, _)| {
                    let idx = bind_values.len() + i + 1;
                    format!("?{}", idx)
                }).collect();
                for tag_name in &tag_names {
                    bind_values.push(Box::new((*tag_name).clone()));
                }
                conditions.push(format!(
                    "f.id IN (SELECT pft.file_id FROM policy_file_tag pft JOIN file_tag ft ON pft.tag_id = ft.id WHERE ft.name IN ({}))",
                    placeholders.join(",")
                ));
            }
        }

        let where_clause = conditions.join(" AND ");

        let sort_column = match params.sort.as_deref() {
            Some("createTime") => "f.create_time",
            Some("fileSize") => "f.file_size",
            Some("viewCount") => "f.view_count",
            Some("downloadCount") => "f.download_count",
            _ => "f.create_time",
        };
        let sort_dir = match params.direction.as_deref() {
            Some("asc") => "ASC",
            _ => "DESC",
        };

        // 查询总数
        let count_sql = format!("SELECT COUNT(*) FROM policy_file f WHERE {}", where_clause);
        let total: i64 = {
            let mut stmt = conn.prepare(&count_sql)?;
            let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
            stmt.query_row(refs.as_slice(), |row| row.get(0))?
        };

        // 查询列表
        bind_values.push(Box::new(size));
        bind_values.push(Box::new(offset));

        let query_sql = format!(
            "SELECT f.id, f.title, f.original_name, f.file_name, f.file_path, f.file_size,
                    f.file_type, f.mime_type, f.description, f.document_number, f.issue_date,
                    f.effective_date, f.issuing_authority, f.download_count, f.view_count,
                    f.is_public, f.is_enabled, f.create_time, f.update_time, f.created_by,
                    f.updated_by, f.category_id,
                    c.name as category_name,
                    u.username as creator_name
             FROM policy_file f
             LEFT JOIN file_category c ON f.category_id = c.id
             LEFT JOIN sys_user u ON f.created_by = u.id
             WHERE {}
             ORDER BY {} {}
             LIMIT ?{} OFFSET ?{}",
            where_clause,
            sort_column,
            sort_dir,
            bind_values.len() - 1,
            bind_values.len()
        );

        let mut stmt = conn.prepare(&query_sql)?;
        let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();

        let files = stmt.query_map(refs.as_slice(), |row| {
            let category_id: Option<i64> = row.get(21)?;
            let category_name: Option<String> = row.get(22)?;
            let creator_id: i64 = row.get(19)?;
            let creator_name: Option<String> = row.get(23)?;

            Ok(PolicyFile {
                id: row.get(0)?,
                title: row.get(1)?,
                original_name: row.get(2)?,
                file_name: row.get(3)?,
                file_path: row.get(4)?,
                file_size: row.get(5)?,
                file_type: row.get(6)?,
                mime_type: row.get(7)?,
                description: row.get(8)?,
                document_number: row.get(9)?,
                issue_date: row.get(10)?,
                effective_date: row.get(11)?,
                issuing_authority: row.get(12)?,
                download_count: row.get(13)?,
                view_count: row.get(14)?,
                is_public: row.get(15)?,
                enabled: row.get(16)?,
                create_time: row.get(17)?,
                update_time: row.get(18)?,
                created_by: creator_id,
                updated_by: row.get(20)?,
                category_id,
                category: category_id.and_then(|cid| {
                    category_name.map(|cn| FileCategoryBrief { id: cid, name: cn })
                }),
                tags: None, // 后续填充
                creator: creator_name.map(|name| UserBrief { id: creator_id, username: name }),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        // 填充标签
        let mut result_files = files;
        for file in &mut result_files {
            file.tags = Some(Self::get_file_tags(&conn, file.id)?);
        }

        Ok(PageResult {
            records: result_files,
            total,
            page,
            size,
        })
    }

    pub fn get_file_by_id(db: &Database, file_id: i64) -> Result<PolicyFile, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut file = conn.query_row(
            "SELECT f.id, f.title, f.original_name, f.file_name, f.file_path, f.file_size,
                    f.file_type, f.mime_type, f.description, f.document_number, f.issue_date,
                    f.effective_date, f.issuing_authority, f.download_count, f.view_count,
                    f.is_public, f.is_enabled, f.create_time, f.update_time, f.created_by,
                    f.updated_by, f.category_id,
                    c.name as category_name,
                    u.username as creator_name
             FROM policy_file f
             LEFT JOIN file_category c ON f.category_id = c.id
             LEFT JOIN sys_user u ON f.created_by = u.id
             WHERE f.id = ?1",
            params![file_id],
            |row| {
                let category_id: Option<i64> = row.get(21)?;
                let category_name: Option<String> = row.get(22)?;
                let creator_id: i64 = row.get(19)?;
                let creator_name: Option<String> = row.get(23)?;

                Ok(PolicyFile {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    original_name: row.get(2)?,
                    file_name: row.get(3)?,
                    file_path: row.get(4)?,
                    file_size: row.get(5)?,
                    file_type: row.get(6)?,
                    mime_type: row.get(7)?,
                    description: row.get(8)?,
                    document_number: row.get(9)?,
                    issue_date: row.get(10)?,
                    effective_date: row.get(11)?,
                    issuing_authority: row.get(12)?,
                    download_count: row.get(13)?,
                    view_count: row.get(14)?,
                    is_public: row.get(15)?,
                    enabled: row.get(16)?,
                    create_time: row.get(17)?,
                    update_time: row.get(18)?,
                    created_by: creator_id,
                    updated_by: row.get(20)?,
                    category_id,
                    category: category_id.and_then(|cid| {
                        category_name.map(|cn| FileCategoryBrief { id: cid, name: cn })
                    }),
                    tags: None,
                    creator: creator_name.map(|name| UserBrief { id: creator_id, username: name }),
                })
            },
        ).map_err(|_| AppError::NotFound("文件不存在".to_string()))?;

        file.tags = Some(Self::get_file_tags(&conn, file.id)?);
        Ok(file)
    }

    pub fn update_file(db: &Database, file_id: i64, req: &UpdateFileRequest, user_id: i64) -> Result<(), AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut updates = Vec::new();
        let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref title) = req.title {
            bind_values.push(Box::new(title.clone()));
            updates.push(format!("title = ?{}", bind_values.len()));
        }
        if let Some(ref desc) = req.description {
            bind_values.push(Box::new(desc.clone()));
            updates.push(format!("description = ?{}", bind_values.len()));
        }
        if let Some(category_id) = req.category_id {
            bind_values.push(Box::new(category_id));
            updates.push(format!("category_id = ?{}", bind_values.len()));
        }
        if let Some(ref doc_number) = req.document_number {
            bind_values.push(Box::new(doc_number.clone()));
            updates.push(format!("document_number = ?{}", bind_values.len()));
        }
        if let Some(ref issue_date) = req.issue_date {
            bind_values.push(Box::new(issue_date.clone()));
            updates.push(format!("issue_date = ?{}", bind_values.len()));
        }
        if let Some(ref effective_date) = req.effective_date {
            bind_values.push(Box::new(effective_date.clone()));
            updates.push(format!("effective_date = ?{}", bind_values.len()));
        }
        if let Some(ref authority) = req.issuing_authority {
            bind_values.push(Box::new(authority.clone()));
            updates.push(format!("issuing_authority = ?{}", bind_values.len()));
        }
        if let Some(is_public) = req.is_public {
            bind_values.push(Box::new(is_public));
            updates.push(format!("is_public = ?{}", bind_values.len()));
        }

        bind_values.push(Box::new(user_id));
        updates.push(format!("updated_by = ?{}", bind_values.len()));

        bind_values.push(Box::new(file_id));

        let sql = format!(
            "UPDATE policy_file SET {} WHERE id = ?{}",
            updates.join(", "),
            bind_values.len()
        );

        let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
        conn.execute(&sql, refs.as_slice())?;

        // 更新标签
        if let Some(ref tags) = req.tags {
            conn.execute("DELETE FROM policy_file_tag WHERE file_id = ?1", params![file_id])?;
            for tag_name in tags {
                let tag_id = Self::ensure_tag(&conn, tag_name, user_id)?;
                conn.execute(
                    "INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)",
                    params![file_id, tag_id],
                )?;
            }
        }

        Ok(())
    }

    pub fn delete_file(db: &Database, file_id: i64) -> Result<(), AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        // 软删除
        conn.execute(
            "UPDATE policy_file SET is_enabled = 0 WHERE id = ?1",
            params![file_id],
        )?;

        Ok(())
    }

    pub fn get_file_path(db: &Database, file_id: i64) -> Result<String, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let file_path: String = conn.query_row(
            "SELECT file_path FROM policy_file WHERE id = ?1 AND is_enabled = 1",
            params![file_id],
            |row| row.get(0),
        ).map_err(|_| AppError::NotFound("文件不存在".to_string()))?;

        // 增加下载次数
        conn.execute(
            "UPDATE policy_file SET download_count = download_count + 1 WHERE id = ?1",
            params![file_id],
        )?;

        let uploads_dir = get_uploads_dir()?;
        let full_path = uploads_dir.join(&file_path);
        Ok(full_path.to_string_lossy().to_string())
    }

    pub fn get_popular_files(db: &Database) -> Result<Vec<PolicyFile>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT f.id, f.title, f.original_name, f.file_name, f.file_path, f.file_size,
                    f.file_type, f.mime_type, f.description, f.document_number, f.issue_date,
                    f.effective_date, f.issuing_authority, f.download_count, f.view_count,
                    f.is_public, f.is_enabled, f.create_time, f.update_time, f.created_by,
                    f.updated_by, f.category_id,
                    c.name as category_name,
                    u.username as creator_name
             FROM policy_file f
             LEFT JOIN file_category c ON f.category_id = c.id
             LEFT JOIN sys_user u ON f.created_by = u.id
             WHERE f.is_enabled = 1 AND f.is_public = 1
             ORDER BY (f.view_count * 0.7 + f.download_count * 0.3) DESC
             LIMIT 10"
        )?;

        let files = stmt.query_map([], |row| {
            let category_id: Option<i64> = row.get(21)?;
            let category_name: Option<String> = row.get(22)?;
            let creator_id: i64 = row.get(19)?;
            let creator_name: Option<String> = row.get(23)?;

            Ok(PolicyFile {
                id: row.get(0)?,
                title: row.get(1)?,
                original_name: row.get(2)?,
                file_name: row.get(3)?,
                file_path: row.get(4)?,
                file_size: row.get(5)?,
                file_type: row.get(6)?,
                mime_type: row.get(7)?,
                description: row.get(8)?,
                document_number: row.get(9)?,
                issue_date: row.get(10)?,
                effective_date: row.get(11)?,
                issuing_authority: row.get(12)?,
                download_count: row.get(13)?,
                view_count: row.get(14)?,
                is_public: row.get(15)?,
                enabled: row.get(16)?,
                create_time: row.get(17)?,
                update_time: row.get(18)?,
                created_by: creator_id,
                updated_by: row.get(20)?,
                category_id,
                category: category_id.and_then(|cid| {
                    category_name.map(|cn| FileCategoryBrief { id: cid, name: cn })
                }),
                tags: None,
                creator: creator_name.map(|name| UserBrief { id: creator_id, username: name }),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(files)
    }

    pub fn get_latest_files(db: &Database) -> Result<Vec<PolicyFile>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT f.id, f.title, f.original_name, f.file_name, f.file_path, f.file_size,
                    f.file_type, f.mime_type, f.description, f.document_number, f.issue_date,
                    f.effective_date, f.issuing_authority, f.download_count, f.view_count,
                    f.is_public, f.is_enabled, f.create_time, f.update_time, f.created_by,
                    f.updated_by, f.category_id,
                    c.name as category_name,
                    u.username as creator_name
             FROM policy_file f
             LEFT JOIN file_category c ON f.category_id = c.id
             LEFT JOIN sys_user u ON f.created_by = u.id
             WHERE f.is_enabled = 1
             ORDER BY f.create_time DESC
             LIMIT 10"
        )?;

        let files = stmt.query_map([], |row| {
            let category_id: Option<i64> = row.get(21)?;
            let category_name: Option<String> = row.get(22)?;
            let creator_id: i64 = row.get(19)?;
            let creator_name: Option<String> = row.get(23)?;

            Ok(PolicyFile {
                id: row.get(0)?,
                title: row.get(1)?,
                original_name: row.get(2)?,
                file_name: row.get(3)?,
                file_path: row.get(4)?,
                file_size: row.get(5)?,
                file_type: row.get(6)?,
                mime_type: row.get(7)?,
                description: row.get(8)?,
                document_number: row.get(9)?,
                issue_date: row.get(10)?,
                effective_date: row.get(11)?,
                issuing_authority: row.get(12)?,
                download_count: row.get(13)?,
                view_count: row.get(14)?,
                is_public: row.get(15)?,
                enabled: row.get(16)?,
                create_time: row.get(17)?,
                update_time: row.get(18)?,
                created_by: creator_id,
                updated_by: row.get(20)?,
                category_id,
                category: category_id.and_then(|cid| {
                    category_name.map(|cn| FileCategoryBrief { id: cid, name: cn })
                }),
                tags: None,
                creator: creator_name.map(|name| UserBrief { id: creator_id, username: name }),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(files)
    }

    pub fn get_my_files(db: &Database, user_id: i64, page: i64, size: i64) -> Result<PageResult<PolicyFile>, AppError> {
        let params = FileSearchParams {
            keyword: None,
            category_id: None,
            tags: None,
            start_date: None,
            end_date: None,
            issuing_authority: None,
            is_public: None,
            page: Some(page),
            size: Some(size),
            sort: Some("createTime".to_string()),
            direction: Some("desc".to_string()),
        };
        // 简单实现：使用全局查询然后过滤
        // TODO: 优化为直接在 SQL 中过滤 created_by
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let offset = page * size;
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM policy_file WHERE created_by = ?1 AND is_enabled = 1",
            params![user_id],
            |row| row.get(0),
        )?;

        let mut stmt = conn.prepare(
            "SELECT f.id, f.title, f.original_name, f.file_name, f.file_path, f.file_size,
                    f.file_type, f.mime_type, f.description, f.document_number, f.issue_date,
                    f.effective_date, f.issuing_authority, f.download_count, f.view_count,
                    f.is_public, f.is_enabled, f.create_time, f.update_time, f.created_by,
                    f.updated_by, f.category_id,
                    c.name as category_name,
                    u.username as creator_name
             FROM policy_file f
             LEFT JOIN file_category c ON f.category_id = c.id
             LEFT JOIN sys_user u ON f.created_by = u.id
             WHERE f.created_by = ?1 AND f.is_enabled = 1
             ORDER BY f.create_time DESC
             LIMIT ?2 OFFSET ?3"
        )?;

        let files = stmt.query_map(rusqlite::params![user_id, size, offset], |row| {
            let category_id: Option<i64> = row.get(21)?;
            let category_name: Option<String> = row.get(22)?;
            let creator_id: i64 = row.get(19)?;
            let creator_name: Option<String> = row.get(23)?;

            Ok(PolicyFile {
                id: row.get(0)?,
                title: row.get(1)?,
                original_name: row.get(2)?,
                file_name: row.get(3)?,
                file_path: row.get(4)?,
                file_size: row.get(5)?,
                file_type: row.get(6)?,
                mime_type: row.get(7)?,
                description: row.get(8)?,
                document_number: row.get(9)?,
                issue_date: row.get(10)?,
                effective_date: row.get(11)?,
                issuing_authority: row.get(12)?,
                download_count: row.get(13)?,
                view_count: row.get(14)?,
                is_public: row.get(15)?,
                enabled: row.get(16)?,
                create_time: row.get(17)?,
                update_time: row.get(18)?,
                created_by: creator_id,
                updated_by: row.get(20)?,
                category_id,
                category: category_id.and_then(|cid| {
                    category_name.map(|cn| FileCategoryBrief { id: cid, name: cn })
                }),
                tags: None,
                creator: creator_name.map(|name| UserBrief { id: creator_id, username: name }),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(PageResult {
            records: files,
            total,
            page,
            size,
        })
    }

    pub fn get_all_tags(db: &Database) -> Result<Vec<FileTag>, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, color, description, create_time, created_by FROM file_tag ORDER BY name"
        )?;

        let tags = stmt.query_map([], |row| {
            Ok(FileTag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                description: row.get(3)?,
                create_time: row.get(4)?,
                created_by: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    // 获取文件内容用于预览
    pub fn get_file_content(db: &Database, file_id: i64) -> Result<String, AppError> {
        let file = Self::get_file_by_id(db, file_id)?;
        let uploads_dir = get_uploads_dir()?;
        let full_path = uploads_dir.join(&file.file_path);

        if !full_path.exists() {
            return Err(AppError::NotFound("文件不存在".to_string()));
        }

        // 增加阅读次数
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute(
            "UPDATE policy_file SET view_count = view_count + 1 WHERE id = ?1",
            params![file_id],
        )?;
        drop(conn);

        let content = std::fs::read_to_string(&full_path)
            .map_err(|_| AppError::BadRequest("无法读取文件内容（可能不是文本文件）".to_string()))?;

        Ok(content)
    }

    // 获取文件用于预览 (返回 base64)
    pub fn get_file_base64(db: &Database, file_id: i64) -> Result<(String, String), AppError> {
        let file = Self::get_file_by_id(db, file_id)?;
        let uploads_dir = get_uploads_dir()?;
        let full_path = uploads_dir.join(&file.file_path);

        if !full_path.exists() {
            return Err(AppError::NotFound("文件不存在".to_string()));
        }

        // 增加阅读次数
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute(
            "UPDATE policy_file SET view_count = view_count + 1 WHERE id = ?1",
            params![file_id],
        )?;
        drop(conn);

        let bytes = std::fs::read(&full_path)?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
        let mime = file.mime_type.unwrap_or_else(|| "application/octet-stream".to_string());

        Ok((b64, mime))
    }

    fn get_file_tags(conn: &rusqlite::Connection, file_id: i64) -> Result<Vec<FileTag>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT ft.id, ft.name, ft.color, ft.description, ft.create_time, ft.created_by
             FROM file_tag ft
             JOIN policy_file_tag pft ON ft.id = pft.tag_id
             WHERE pft.file_id = ?1"
        )?;

        let tags = stmt.query_map(params![file_id], |row| {
            Ok(FileTag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                description: row.get(3)?,
                create_time: row.get(4)?,
                created_by: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    fn ensure_tag(conn: &rusqlite::Connection, name: &str, user_id: i64) -> Result<i64, AppError> {
        // 尝试查找已有标签
        match conn.query_row(
            "SELECT id FROM file_tag WHERE name = ?1",
            params![name],
            |row| row.get::<_, i64>(0),
        ) {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                conn.execute(
                    "INSERT INTO file_tag (name, created_by) VALUES (?1, ?2)",
                    params![name, user_id],
                )?;
                Ok(conn.last_insert_rowid())
            }
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }
}
