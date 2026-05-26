use serde::{Deserialize, Serialize};

// ====== 用户相关 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub avatar: Option<String>,
    pub enabled: bool,
    pub create_time: String,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

// ====== 分类相关 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCategory {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
    pub enabled: bool,
    pub create_time: String,
    pub update_time: Option<String>,
    pub created_by: Option<i64>,
    #[serde(default)]
    pub children: Vec<FileCategory>,
    #[serde(default)]
    pub file_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: Option<i32>,
    pub enabled: Option<bool>,
}

// ====== 文件相关 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyFile {
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
    pub download_count: i32,
    pub view_count: i32,
    pub is_public: bool,
    pub enabled: bool,
    pub create_time: String,
    pub update_time: Option<String>,
    pub created_by: i64,
    pub updated_by: Option<i64>,
    pub category_id: Option<i64>,
    pub category: Option<FileCategoryBrief>,
    pub tags: Option<Vec<FileTag>>,
    pub creator: Option<UserBrief>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCategoryBrief {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBrief {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub create_time: String,
    pub created_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileRequest {
    pub file_path: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub document_number: Option<String>,
    pub issue_date: Option<String>,
    pub effective_date: Option<String>,
    pub issuing_authority: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFileRequest {
    pub title: Option<String>,
    pub original_name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub document_number: Option<String>,
    pub issue_date: Option<String>,
    pub effective_date: Option<String>,
    pub issuing_authority: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSearchParams {
    pub keyword: Option<String>,
    pub category_id: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub issuing_authority: Option<String>,
    pub is_public: Option<bool>,
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub sort: Option<String>,
    pub direction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub records: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
}

// ====== 统一响应 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 200,
            message: "success".to_string(),
            data,
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: i32, message: &str) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.to_string(),
            data: (),
        }
    }
}

// ====== 预览相关 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePreviewInfo {
    pub id: i64,
    pub title: String,
    pub file_name: String,
    pub file_type: String,
    pub preview_type: String,
    pub file_size: i64,
    pub can_preview: bool,
    pub preview_url: String,
    pub download_url: String,
}

// ====== 用户列表查询 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListParams {
    pub current: Option<i64>,
    pub size: Option<i64>,
    pub username: Option<String>,
    pub role: Option<String>,
}

// ====== 分类统计 ======

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryStats {
    pub category_id: i64,
    pub category_name: String,
    pub file_count: i64,
    pub total_size: i64,
}
