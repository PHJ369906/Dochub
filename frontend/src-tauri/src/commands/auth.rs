use tauri::State;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::{ApiResponse, LoginRequest, LoginResponse, RegisterRequest, User, PageResult, UserListParams};
use crate::services::auth_service::AuthService;

#[tauri::command]
pub fn login(
    db: State<Database>,
    auth: State<AuthService>,
    username: String,
    password: String,
) -> Result<ApiResponse<LoginResponse>, String> {
    match auth.login(&db, &username, &password) {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn register(
    db: State<Database>,
    auth: State<AuthService>,
    username: String,
    password: String,
    email: Option<String>,
) -> Result<ApiResponse<User>, String> {
    match auth.register(&db, &username, &password, email.as_deref()) {
        Ok(user) => Ok(ApiResponse::success(user)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn logout(auth: State<AuthService>, token: String) -> Result<ApiResponse<()>, String> {
    match auth.logout(&token) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_current_user(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
) -> Result<ApiResponse<User>, String> {
    match auth.get_current_user(&db, &token) {
        Ok(user) => Ok(ApiResponse::success(user)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn check_login(auth: State<AuthService>, token: String) -> Result<ApiResponse<bool>, String> {
    Ok(ApiResponse::success(auth.check_login(&token)))
}

#[tauri::command]
pub fn get_user_list(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    current: Option<i64>,
    size: Option<i64>,
    username: Option<String>,
    role: Option<String>,
) -> Result<ApiResponse<PageResult<User>>, String> {
    // 验证权限
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let params = UserListParams { current, size, username, role };
    match auth.get_user_list(&db, &params) {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_user_detail(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<User>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match auth.get_user_detail(&db, id) {
        Ok(user) => Ok(ApiResponse::success(user)),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn toggle_user_status(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<String>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    match auth.toggle_user_status(&db, id) {
        Ok(msg) => Ok(ApiResponse::success(msg)),
        Err(e) => Err(e.to_string()),
    }
}
