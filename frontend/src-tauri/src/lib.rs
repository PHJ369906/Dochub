mod db;
mod errors;
mod models;
mod commands;
mod services;

use db::Database;
use services::auth_service::AuthService;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = Database::new().expect("无法初始化数据库");
    let auth_service = AuthService::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(database)
        .manage(auth_service)
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            commands::auth::login,
            commands::auth::register,
            commands::auth::logout,
            commands::auth::get_current_user,
            commands::auth::check_login,
            commands::auth::get_user_list,
            commands::auth::get_user_detail,
            commands::auth::toggle_user_status,
            // File commands
            commands::files::upload_file,
            commands::files::batch_upload_files,
            commands::files::get_files,
            commands::files::get_file_by_id,
            commands::files::update_file,
            commands::files::delete_file,
            commands::files::get_file_path,
            commands::files::get_popular_files,
            commands::files::get_latest_files,
            commands::files::get_my_files,
            commands::files::get_all_tags,
            commands::files::get_file_content,
            commands::files::open_file_with_system,
            commands::files::open_file_folder,
            // Category commands
            commands::categories::get_categories,
            commands::categories::get_root_categories,
            commands::categories::get_child_categories,
            commands::categories::create_category,
            commands::categories::update_category,
            commands::categories::delete_category,
            commands::categories::move_category,
            commands::categories::get_category_stats,
            // Preview commands
            commands::preview::get_file_preview_info,
            commands::preview::preview_file,
            // Transfer commands
            commands::transfer::export_data,
            commands::transfer::preview_import,
            commands::transfer::import_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
