use tauri::State;

use crate::db::Database;
use crate::models::{ApiResponse, FileCategory, CreateCategoryRequest, UpdateCategoryRequest, CategoryStats};
use crate::services::auth_service::AuthService;
use crate::services::category_service::CategoryService;

#[tauri::command]
pub fn get_categories(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
) -> Result<ApiResponse<Vec<FileCategory>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match CategoryService::get_all_categories(&db) {
        Ok(categories) => Ok(ApiResponse::success(categories)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_root_categories(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
) -> Result<ApiResponse<Vec<FileCategory>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match CategoryService::get_root_categories(&db) {
        Ok(categories) => Ok(ApiResponse::success(categories)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_child_categories(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    parent_id: i64,
) -> Result<ApiResponse<Vec<FileCategory>>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match CategoryService::get_child_categories(&db, parent_id) {
        Ok(categories) => Ok(ApiResponse::success(categories)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn create_category(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    name: String,
    description: Option<String>,
    parent_id: Option<i64>,
    sort_order: Option<i32>,
) -> Result<ApiResponse<FileCategory>, String> {
    let user_id = auth.validate_token(&token).map_err(|e| e.to_string())?;

    let req = CreateCategoryRequest {
        name,
        description,
        parent_id,
        sort_order: sort_order.unwrap_or(0),
    };

    match CategoryService::create_category(&db, &req, user_id) {
        Ok(cat) => Ok(ApiResponse::success(cat)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn update_category(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    parent_id: Option<i64>,
    sort_order: Option<i32>,
    enabled: Option<bool>,
) -> Result<ApiResponse<()>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let req = UpdateCategoryRequest {
        name,
        description,
        parent_id,
        sort_order,
        enabled,
    };

    match CategoryService::update_category(&db, id, &req) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn delete_category(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match CategoryService::delete_category(&db, id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn move_category(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
    new_parent_id: Option<i64>,
) -> Result<ApiResponse<()>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match CategoryService::move_category(&db, id, new_parent_id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_category_stats(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<CategoryStats>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match CategoryService::get_category_stats(&db, id) {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(e) => Err(e.to_string()),
    }
}
