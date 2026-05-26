use tauri::State;

use crate::db::Database;
use crate::models::{ApiResponse, PolicyFile, UploadFileRequest, UpdateFileRequest, FileSearchParams, PageResult, FileTag};
use crate::services::auth_service::AuthService;
use crate::services::file_service::FileService;

#[tauri::command]
pub fn upload_file(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    file_path: String,
    title: Option<String>,
    description: Option<String>,
    category_id: Option<i64>,
    tags: Option<Vec<String>>,
    document_number: Option<String>,
    issue_date: Option<String>,
    effective_date: Option<String>,
    issuing_authority: Option<String>,
    is_public: Option<bool>,
) -> Result<ApiResponse<PolicyFile>, String> {
    let user_id = auth.validate_token(&token).map_err(|e| e.to_string())?;

    let req = UploadFileRequest {
        file_path,
        title,
        description,
        category_id,
        tags,
        document_number,
        issue_date,
        effective_date,
        issuing_authority,
        is_public,
    };

    match FileService::upload_file(&db, &req, user_id) {
        Ok(file) => Ok(ApiResponse::success(file)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn batch_upload_files(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    file_paths: Vec<String>,
    category_id: Option<i64>,
) -> Result<ApiResponse<Vec<PolicyFile>>, String> {
    let user_id = auth.validate_token(&token).map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for path in file_paths {
        let req = UploadFileRequest {
            file_path: path,
            title: None,
            description: None,
            category_id,
            tags: None,
            document_number: None,
            issue_date: None,
            effective_date: None,
            issuing_authority: None,
            is_public: Some(true),
        };
        match FileService::upload_file(&db, &req, user_id) {
            Ok(file) => results.push(file),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(ApiResponse::success(results))
}

#[tauri::command]
pub fn get_files(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    keyword: Option<String>,
    category_id: Option<i64>,
    tags: Option<Vec<String>>,
    start_date: Option<String>,
    end_date: Option<String>,
    issuing_authority: Option<String>,
    is_public: Option<bool>,
    page: Option<i64>,
    size: Option<i64>,
    sort: Option<String>,
    direction: Option<String>,
) -> Result<ApiResponse<PageResult<PolicyFile>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let params = FileSearchParams {
        keyword,
        category_id,
        tags,
        start_date,
        end_date,
        issuing_authority,
        is_public,
        page,
        size,
        sort,
        direction,
    };

    match FileService::get_files(&db, &params) {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_file_by_id(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<PolicyFile>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_file_by_id(&db, id) {
        Ok(file) => Ok(ApiResponse::success(file)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn update_file(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
    title: Option<String>,
    original_name: Option<String>,
    description: Option<String>,
    category_id: Option<i64>,
    tags: Option<Vec<String>>,
    document_number: Option<String>,
    issue_date: Option<String>,
    effective_date: Option<String>,
    issuing_authority: Option<String>,
    is_public: Option<bool>,
) -> Result<ApiResponse<()>, String> {
    let user_id = auth.validate_token(&token).map_err(|e| e.to_string())?;

    let req = UpdateFileRequest {
        title,
        original_name,
        description,
        category_id,
        tags,
        document_number,
        issue_date,
        effective_date,
        issuing_authority,
        is_public,
    };

    match FileService::update_file(&db, id, &req, user_id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn delete_file(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::delete_file(&db, id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_file_path(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<String>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_file_path(&db, id) {
        Ok(path) => Ok(ApiResponse::success(path)),
        Err(e) => Err(e.to_string()),
    }
}

/// 用系统默认应用打开文件
#[tauri::command]
pub fn open_file_with_system(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let path = FileService::get_file_path(&db, id).map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("无法打开文件: {}", e))?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/c", "start", "", &path])
        .spawn()
        .map_err(|e| format!("无法打开文件: {}", e))?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("无法打开文件: {}", e))?;

    Ok(ApiResponse::success(()))
}

/// 打开文件所在目录
#[tauri::command]
pub fn open_file_folder(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let path = FileService::get_file_path(&db, id).map_err(|e| e.to_string())?;
    let parent = std::path::Path::new(&path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(path.clone());

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&parent)
        .spawn()
        .map_err(|e| format!("无法打开目录: {}", e))?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&parent)
        .spawn()
        .map_err(|e| format!("无法打开目录: {}", e))?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&parent)
        .spawn()
        .map_err(|e| format!("无法打开目录: {}", e))?;

    Ok(ApiResponse::success(()))
}

#[tauri::command]
pub fn get_popular_files(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
) -> Result<ApiResponse<Vec<PolicyFile>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_popular_files(&db) {
        Ok(files) => Ok(ApiResponse::success(files)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_latest_files(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
) -> Result<ApiResponse<Vec<PolicyFile>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_latest_files(&db) {
        Ok(files) => Ok(ApiResponse::success(files)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_my_files(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    page: Option<i64>,
    size: Option<i64>,
) -> Result<ApiResponse<PageResult<PolicyFile>>, String> {
    let user_id = auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_my_files(&db, user_id, page.unwrap_or(0), size.unwrap_or(10)) {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_all_tags(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
) -> Result<ApiResponse<Vec<FileTag>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_all_tags(&db) {
        Ok(tags) => Ok(ApiResponse::success(tags)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_file_content(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<String>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match FileService::get_file_content(&db, id) {
        Ok(content) => Ok(ApiResponse::success(content)),
        Err(e) => Err(e.to_string()),
    }
}
