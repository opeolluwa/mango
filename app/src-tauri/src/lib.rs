use crate::database::initialize_database;
use crate::migrations::load_migrations;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::utils::initialize_app_directories;
use std::sync::Arc;
use tauri::Manager;

mod adapters;
mod commands;
mod config;
mod database;
mod error;
mod migrations;
mod models;
mod state;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_stronghold::Builder::new(|_pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle();
            let app_config = config::AppConfig::default();
            let app_data_dir = app.path().app_data_dir().unwrap();

            initialize_app_directories(&app_data_dir, &app_config).unwrap();

            // move owned values into async task
            let app_config_cloned = app_config.clone();
            let app_data_dir_cloned = app_data_dir.clone();
            let app_handle_cloned = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                match initialize_database(&app_data_dir_cloned, &app_config_cloned).await {
                    Ok(state) => {
                        log::info!("AppState initialized: {state:?}");
                        app_handle_cloned.manage(Arc::new(state));
                        Ok(())
                    }
                    Err(e) => {
                        log::error!("Failed to initialize AppState: {e}");
                        Err(e)
                    }
                }
            });

            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    config::AppConfig::default().database_file,
                    load_migrations(),
                )
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::app_personalization::fetch_app_personalization,
            commands::app_personalization::update_app_personalization,
            commands::app_personalization::set_theme,
            commands::cached_user::set_cached_user,
            commands::cached_user::fetch_cached_user,
            commands::app_settings::fetch_app_settings,
            commands::app_settings::initalize_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
