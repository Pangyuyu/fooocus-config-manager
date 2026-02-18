mod database;
mod commands;

use database::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            let app_data_dir = app_handle.path().app_data_dir()
                .expect("Failed to get app data dir");
            let db = Database::new(app_data_dir)
                .expect("Failed to initialize database");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_all_presets,
            commands::get_preset_by_id,
            commands::create_preset,
            commands::update_preset,
            commands::delete_preset,
            commands::search_presets,
            commands::get_all_tags,
            commands::create_tag,
            commands::delete_tag,
            commands::toggle_favorite,
            commands::increment_use_count,
            commands::get_all_models,
            commands::get_models_by_type,
            commands::get_model_by_id,
            commands::create_model,
            commands::update_model,
            commands::delete_model,
            commands::search_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
