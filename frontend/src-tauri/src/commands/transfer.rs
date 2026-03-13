use tauri::State;

use crate::db::Database;
use crate::models::ApiResponse;
use crate::services::auth_service::AuthService;
use crate::services::transfer_service::{
    ExportResult, ImportPreview, ImportResult, TransferService,
};

#[tauri::command]
pub fn export_data(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    file_ids: Vec<i64>,
    category_ids: Vec<i64>,
    export_path: String,
) -> Result<ApiResponse<ExportResult>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match TransferService::export_data(&db, file_ids, category_ids, export_path) {
        Ok(result) => Ok(ApiResponse::success(result)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn preview_import(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    import_path: String,
) -> Result<ApiResponse<ImportPreview>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match TransferService::preview_import(&db, import_path) {
        Ok(preview) => Ok(ApiResponse::success(preview)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn import_data(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    import_path: String,
    strategy: String,
) -> Result<ApiResponse<ImportResult>, String> {
    let user_id = auth.validate_token(&token).map_err(|e| e.to_string())?;

    match TransferService::import_data(&db, import_path, strategy, user_id) {
        Ok(result) => Ok(ApiResponse::success(result)),
        Err(e) => Err(e.to_string()),
    }
}
