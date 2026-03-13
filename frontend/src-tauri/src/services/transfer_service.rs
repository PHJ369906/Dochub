use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::{Database, get_uploads_dir};
use crate::errors::AppError;

// ====== 导出数据结构 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportManifest {
    pub version: String,
    pub app_version: String,
    pub export_time: String,
    pub summary: ExportSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSummary {
    pub categories: usize,
    pub files: usize,
    pub tags: usize,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportData {
    pub categories: Vec<ExportCategory>,
    pub files: Vec<ExportFile>,
    pub tags: Vec<ExportTag>,
    pub users: Vec<ExportUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCategory {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
    pub create_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportFile {
    pub id: i64,
    pub title: String,
    pub original_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub file_type: String,
    pub mime_type: Option<String>,
    pub description: Option<String>,
    pub document_number: Option<String>,
    pub issue_date: Option<String>,
    pub effective_date: Option<String>,
    pub issuing_authority: Option<String>,
    pub is_public: bool,
    pub create_time: String,
    pub category_id: Option<i64>,
    pub created_by: i64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportTag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportUser {
    pub id: i64,
    pub username: String,
    pub role: String,
}

// ====== 导入预览 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    pub version: String,
    pub export_time: String,
    pub category_count: usize,
    pub file_count: usize,
    pub tag_count: usize,
    pub total_size: i64,
    pub categories: Vec<ImportPreviewCategory>,
    pub conflict_files: Vec<String>,
    pub conflict_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreviewCategory {
    pub name: String,
    pub file_count: usize,
}

// ====== 导入结果 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub file_path: String,
    pub file_count: usize,
    pub category_count: usize,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub categories_imported: usize,
    pub files_imported: usize,
    pub tags_imported: usize,
    pub files_skipped: usize,
    pub files_overwritten: usize,
    pub errors: Vec<String>,
}

pub struct TransferService;

impl TransferService {
    /// 导出数据为 .dochub 文件
    pub fn export_data(
        db: &Database,
        file_ids: Vec<i64>,
        category_ids: Vec<i64>,
        export_path: String,
    ) -> Result<ExportResult, AppError> {
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        let uploads_dir = get_uploads_dir()?;

        // 1. 收集分类数据
        let mut categories = Vec::new();
        if !category_ids.is_empty() {
            // 收集选中分类及其所有祖先分类
            let all_category_ids = Self::collect_category_with_ancestors(&conn, &category_ids)?;
            for cid in &all_category_ids {
                if let Ok(cat) = conn.query_row(
                    "SELECT id, name, description, parent_id, sort_order, create_time
                     FROM file_category WHERE id = ?1 AND is_enabled = 1",
                    params![cid],
                    |row| {
                        Ok(ExportCategory {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            description: row.get(2)?,
                            parent_id: row.get(3)?,
                            sort_order: row.get(4)?,
                            create_time: row.get(5)?,
                        })
                    },
                ) {
                    categories.push(cat);
                }
            }
        }

        // 2. 收集文件数据
        let mut files = Vec::new();
        let mut all_tag_ids = std::collections::HashSet::new();
        let mut all_user_ids = std::collections::HashSet::new();
        let mut total_size: i64 = 0;

        // 文件 IDs 可能直接指定，也可能通过分类获取
        let mut effective_file_ids = file_ids.clone();

        // 如果选了分类，也获取分类下的文件
        if !category_ids.is_empty() {
            let placeholders: Vec<String> = category_ids.iter().enumerate().map(|(i, _)| format!("?{}", i + 1)).collect();
            let sql = format!(
                "SELECT id FROM policy_file WHERE category_id IN ({}) AND is_enabled = 1",
                placeholders.join(",")
            );
            let mut stmt = conn.prepare(&sql)?;
            let refs: Vec<Box<dyn rusqlite::types::ToSql>> = category_ids.iter().map(|id| Box::new(*id) as Box<dyn rusqlite::types::ToSql>).collect();
            let param_refs: Vec<&dyn rusqlite::types::ToSql> = refs.iter().map(|b| b.as_ref()).collect();
            let cat_file_ids: Vec<i64> = stmt.query_map(param_refs.as_slice(), |row| row.get(0))?
                .collect::<Result<Vec<_>, _>>()?;
            for fid in cat_file_ids {
                if !effective_file_ids.contains(&fid) {
                    effective_file_ids.push(fid);
                }
            }
        }

        for fid in &effective_file_ids {
            let file_result = conn.query_row(
                "SELECT id, title, original_name, file_name, file_path, file_size, file_type,
                        mime_type, description, document_number, issue_date, effective_date,
                        issuing_authority, is_public, create_time, category_id, created_by
                 FROM policy_file WHERE id = ?1 AND is_enabled = 1",
                params![fid],
                |row| {
                    Ok(ExportFile {
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
                        is_public: row.get(13)?,
                        create_time: row.get(14)?,
                        category_id: row.get(15)?,
                        created_by: row.get(16)?,
                        tags: Vec::new(),
                    })
                },
            );

            if let Ok(mut file) = file_result {
                total_size += file.file_size;
                all_user_ids.insert(file.created_by);

                if let Some(cid) = file.category_id {
                    // 确保分类也被包含
                    if categories.iter().all(|c| c.id != cid) {
                        let ancestor_ids = Self::collect_category_with_ancestors(&conn, &[cid])?;
                        for aid in &ancestor_ids {
                            if categories.iter().all(|c| c.id != *aid) {
                                if let Ok(cat) = conn.query_row(
                                    "SELECT id, name, description, parent_id, sort_order, create_time
                                     FROM file_category WHERE id = ?1 AND is_enabled = 1",
                                    params![aid],
                                    |row| {
                                        Ok(ExportCategory {
                                            id: row.get(0)?,
                                            name: row.get(1)?,
                                            description: row.get(2)?,
                                            parent_id: row.get(3)?,
                                            sort_order: row.get(4)?,
                                            create_time: row.get(5)?,
                                        })
                                    },
                                ) {
                                    categories.push(cat);
                                }
                            }
                        }
                    }
                }

                // 获取文件标签
                let mut tag_stmt = conn.prepare(
                    "SELECT ft.id, ft.name FROM file_tag ft
                     JOIN policy_file_tag pft ON ft.id = pft.tag_id
                     WHERE pft.file_id = ?1"
                )?;
                let tag_rows: Vec<(i64, String)> = tag_stmt.query_map(params![fid], |row| {
                    Ok((row.get(0)?, row.get(1)?))
                })?.collect::<Result<Vec<_>, _>>()?;

                for (tid, tname) in tag_rows {
                    all_tag_ids.insert(tid);
                    file.tags.push(tname);
                }

                files.push(file);
            }
        }

        // 3. 收集标签数据
        let mut tags = Vec::new();
        for tid in &all_tag_ids {
            if let Ok(tag) = conn.query_row(
                "SELECT id, name, color, description FROM file_tag WHERE id = ?1",
                params![tid],
                |row| {
                    Ok(ExportTag {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        color: row.get(2)?,
                        description: row.get(3)?,
                    })
                },
            ) {
                tags.push(tag);
            }
        }

        // 4. 收集用户数据
        let mut users = Vec::new();
        for uid in &all_user_ids {
            if let Ok(user) = conn.query_row(
                "SELECT id, username, role FROM sys_user WHERE id = ?1",
                params![uid],
                |row| {
                    Ok(ExportUser {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        role: row.get(2)?,
                    })
                },
            ) {
                users.push(user);
            }
        }

        drop(conn);

        // 5. 构建 manifest
        let manifest = ExportManifest {
            version: "1.0".to_string(),
            app_version: "0.1.0".to_string(),
            export_time: chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            summary: ExportSummary {
                categories: categories.len(),
                files: files.len(),
                tags: tags.len(),
                total_size,
            },
        };

        let data = ExportData {
            categories,
            files: files.clone(),
            tags,
            users,
        };

        // 6. 创建 ZIP 包
        let export_file = std::fs::File::create(&export_path)
            .map_err(|e| AppError::Internal(format!("无法创建导出文件: {}", e)))?;
        let mut zip_writer = zip::ZipWriter::new(export_file);

        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // 写入 manifest.json
        zip_writer.start_file("manifest.json", options)
            .map_err(|e| AppError::Internal(format!("ZIP写入失败: {}", e)))?;
        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| AppError::Internal(format!("序列化失败: {}", e)))?;
        zip_writer.write_all(manifest_json.as_bytes())?;

        // 写入 data.json
        zip_writer.start_file("data.json", options)
            .map_err(|e| AppError::Internal(format!("ZIP写入失败: {}", e)))?;
        let data_json = serde_json::to_string_pretty(&data)
            .map_err(|e| AppError::Internal(format!("序列化失败: {}", e)))?;
        zip_writer.write_all(data_json.as_bytes())?;

        // 写入实际文件
        for file in &files {
            let source_path = uploads_dir.join(&file.file_path);
            if source_path.exists() {
                let zip_path = format!("uploads/{}", file.file_path);
                zip_writer.start_file(&zip_path, options)
                    .map_err(|e| AppError::Internal(format!("ZIP写入失败: {}", e)))?;
                let mut source_file = std::fs::File::open(&source_path)?;
                let mut buffer = Vec::new();
                source_file.read_to_end(&mut buffer)?;
                zip_writer.write_all(&buffer)?;
            }
        }

        zip_writer.finish().map_err(|e| AppError::Internal(format!("ZIP完成失败: {}", e)))?;

        Ok(ExportResult {
            file_path: export_path,
            file_count: files.len(),
            category_count: data.categories.len(),
            total_size,
        })
    }

    /// 预览导入文件内容
    pub fn preview_import(
        db: &Database,
        import_path: String,
    ) -> Result<ImportPreview, AppError> {
        let zip_file = std::fs::File::open(&import_path)
            .map_err(|e| AppError::BadRequest(format!("无法打开文件: {}", e)))?;
        let mut archive = zip::ZipArchive::new(zip_file)
            .map_err(|e| AppError::BadRequest(format!("无效的 .dochub 文件: {}", e)))?;

        // 读取 manifest.json
        let manifest: ExportManifest = {
            let mut entry = archive.by_name("manifest.json")
                .map_err(|_| AppError::BadRequest("文件中缺少 manifest.json".to_string()))?;
            let mut buf = String::new();
            entry.read_to_string(&mut buf)?;
            serde_json::from_str(&buf)
                .map_err(|e| AppError::BadRequest(format!("manifest.json 解析失败: {}", e)))?
        };

        // 读取 data.json
        let data: ExportData = {
            let mut entry = archive.by_name("data.json")
                .map_err(|_| AppError::BadRequest("文件中缺少 data.json".to_string()))?;
            let mut buf = String::new();
            entry.read_to_string(&mut buf)?;
            serde_json::from_str(&buf)
                .map_err(|e| AppError::BadRequest(format!("data.json 解析失败: {}", e)))?
        };

        // 检测冲突
        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut conflict_files = Vec::new();
        for file in &data.files {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) > 0 FROM policy_file WHERE original_name = ?1 AND is_enabled = 1",
                params![file.original_name],
                |row| row.get(0),
            )?;
            if exists {
                conflict_files.push(file.original_name.clone());
            }
        }

        let mut conflict_categories = Vec::new();
        for cat in &data.categories {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) > 0 FROM file_category WHERE name = ?1 AND is_enabled = 1 AND parent_id IS ?2",
                params![cat.name, cat.parent_id],
                |row| row.get(0),
            )?;
            if exists {
                conflict_categories.push(cat.name.clone());
            }
        }

        // 构建分类预览
        let mut cat_file_count: HashMap<i64, usize> = HashMap::new();
        for file in &data.files {
            if let Some(cid) = file.category_id {
                *cat_file_count.entry(cid).or_insert(0) += 1;
            }
        }

        let categories: Vec<ImportPreviewCategory> = data.categories.iter().map(|c| {
            ImportPreviewCategory {
                name: c.name.clone(),
                file_count: cat_file_count.get(&c.id).copied().unwrap_or(0),
            }
        }).collect();

        Ok(ImportPreview {
            version: manifest.version,
            export_time: manifest.export_time,
            category_count: data.categories.len(),
            file_count: data.files.len(),
            tag_count: data.tags.len(),
            total_size: manifest.summary.total_size,
            categories,
            conflict_files,
            conflict_categories,
        })
    }

    /// 执行导入
    pub fn import_data(
        db: &Database,
        import_path: String,
        strategy: String,
        user_id: i64,
    ) -> Result<ImportResult, AppError> {
        let zip_file = std::fs::File::open(&import_path)
            .map_err(|e| AppError::BadRequest(format!("无法打开文件: {}", e)))?;
        let mut archive = zip::ZipArchive::new(zip_file)
            .map_err(|e| AppError::BadRequest(format!("无效的 .dochub 文件: {}", e)))?;

        // 读取 data.json
        let data: ExportData = {
            let mut entry = archive.by_name("data.json")
                .map_err(|_| AppError::BadRequest("文件中缺少 data.json".to_string()))?;
            let mut buf = String::new();
            entry.read_to_string(&mut buf)?;
            serde_json::from_str(&buf)
                .map_err(|e| AppError::BadRequest(format!("data.json 解析失败: {}", e)))?
        };

        let conn = db.conn.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        let uploads_dir = get_uploads_dir()?;

        let mut result = ImportResult {
            categories_imported: 0,
            files_imported: 0,
            tags_imported: 0,
            files_skipped: 0,
            files_overwritten: 0,
            errors: Vec::new(),
        };

        // 1. 导入分类（按层级顺序，先导入无 parent 的）
        let mut category_id_map: HashMap<i64, i64> = HashMap::new(); // old_id -> new_id

        // 先排序：无父的在前
        let mut sorted_categories = data.categories.clone();
        sorted_categories.sort_by_key(|c| c.parent_id.is_some() as u8);

        // 多次遍历确保父分类先被处理
        let mut remaining = sorted_categories.clone();
        let max_iterations = 10;
        for _ in 0..max_iterations {
            if remaining.is_empty() {
                break;
            }
            let mut next_remaining = Vec::new();
            for cat in &remaining {
                // 确定新的 parent_id
                let new_parent_id = match cat.parent_id {
                    None => None,
                    Some(old_pid) => {
                        if let Some(new_pid) = category_id_map.get(&old_pid) {
                            Some(*new_pid)
                        } else {
                            // 父分类还没导入，稍后重试
                            next_remaining.push(cat.clone());
                            continue;
                        }
                    }
                };

                // 检查是否已存在同名同父的分类
                let existing_id: Option<i64> = conn.query_row(
                    "SELECT id FROM file_category WHERE name = ?1 AND is_enabled = 1 AND parent_id IS ?2",
                    params![cat.name, new_parent_id],
                    |row| row.get(0),
                ).ok();

                if let Some(eid) = existing_id {
                    // 已存在，复用
                    category_id_map.insert(cat.id, eid);
                } else {
                    // 创建新分类
                    conn.execute(
                        "INSERT INTO file_category (name, description, parent_id, sort_order, created_by)
                         VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![cat.name, cat.description, new_parent_id, cat.sort_order, user_id],
                    )?;
                    let new_id = conn.last_insert_rowid();
                    category_id_map.insert(cat.id, new_id);
                    result.categories_imported += 1;
                }
            }
            remaining = next_remaining;
        }

        // 2. 导入标签（find-or-create）
        let mut tag_name_map: HashMap<String, i64> = HashMap::new();
        for tag in &data.tags {
            let existing_id: Option<i64> = conn.query_row(
                "SELECT id FROM file_tag WHERE name = ?1",
                params![tag.name],
                |row| row.get(0),
            ).ok();

            if let Some(eid) = existing_id {
                tag_name_map.insert(tag.name.clone(), eid);
            } else {
                conn.execute(
                    "INSERT INTO file_tag (name, color, description, created_by) VALUES (?1, ?2, ?3, ?4)",
                    params![tag.name, tag.color, tag.description, user_id],
                )?;
                let new_id = conn.last_insert_rowid();
                tag_name_map.insert(tag.name.clone(), new_id);
                result.tags_imported += 1;
            }
        }

        // 3. 导入文件
        for file in &data.files {
            // 检查是否已存在同名文件
            let existing_id: Option<i64> = conn.query_row(
                "SELECT id FROM policy_file WHERE original_name = ?1 AND is_enabled = 1",
                params![file.original_name],
                |row| row.get(0),
            ).ok();

            if let Some(eid) = existing_id {
                if strategy == "skip" {
                    result.files_skipped += 1;
                    continue;
                }
                // overwrite: 删除旧记录
                conn.execute(
                    "DELETE FROM policy_file_tag WHERE file_id = ?1",
                    params![eid],
                )?;
                conn.execute(
                    "DELETE FROM policy_file WHERE id = ?1",
                    params![eid],
                )?;
                result.files_overwritten += 1;
            }

            // 映射 category_id
            let new_category_id = file.category_id.and_then(|old_cid| category_id_map.get(&old_cid).copied());

            // 生成新的存储路径
            let now = chrono::Local::now();
            let date_path = now.format("%Y/%m/%d").to_string();
            let file_ext = Path::new(&file.original_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            let uuid_name = format!("{}.{}", uuid::Uuid::new_v4(), file_ext);
            let new_file_path = format!("{}/{}", date_path, uuid_name);

            // 从 ZIP 中解压文件
            let zip_entry_path = format!("uploads/{}", file.file_path);
            let file_extracted = {
                let target_dir = uploads_dir.join(&date_path);
                std::fs::create_dir_all(&target_dir)?;
                let target_path = target_dir.join(&uuid_name);

                match archive.by_name(&zip_entry_path) {
                    Ok(mut entry) => {
                        let mut out_file = std::fs::File::create(&target_path)?;
                        std::io::copy(&mut entry, &mut out_file)?;
                        true
                    }
                    Err(e) => {
                        result.errors.push(format!("文件 {} 解压失败: {}", file.original_name, e));
                        false
                    }
                }
            };

            if !file_extracted {
                continue;
            }

            // 插入文件记录
            conn.execute(
                "INSERT INTO policy_file (title, original_name, file_name, file_path, file_size,
                 file_type, mime_type, description, document_number, issue_date, effective_date,
                 issuing_authority, is_public, created_by, category_id)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                params![
                    file.title, file.original_name, uuid_name, new_file_path, file.file_size,
                    file.file_type, file.mime_type, file.description, file.document_number,
                    file.issue_date, file.effective_date, file.issuing_authority,
                    file.is_public, user_id, new_category_id
                ],
            )?;

            let new_file_id = conn.last_insert_rowid();

            // 关联标签
            for tag_name in &file.tags {
                if let Some(tag_id) = tag_name_map.get(tag_name) {
                    conn.execute(
                        "INSERT OR IGNORE INTO policy_file_tag (file_id, tag_id) VALUES (?1, ?2)",
                        params![new_file_id, tag_id],
                    )?;
                }
            }

            result.files_imported += 1;
        }

        Ok(result)
    }

    /// 收集指定分类及其所有祖先分类的 ID
    fn collect_category_with_ancestors(
        conn: &rusqlite::Connection,
        category_ids: &[i64],
    ) -> Result<Vec<i64>, AppError> {
        let mut all_ids = std::collections::HashSet::new();

        for &cid in category_ids {
            let mut current_id = Some(cid);
            while let Some(id) = current_id {
                if !all_ids.insert(id) {
                    break; // 已处理过
                }
                current_id = conn.query_row(
                    "SELECT parent_id FROM file_category WHERE id = ?1",
                    params![id],
                    |row| row.get::<_, Option<i64>>(0),
                ).unwrap_or(None);
            }
        }

        let mut result: Vec<i64> = all_ids.into_iter().collect();
        result.sort();
        Ok(result)
    }
}
