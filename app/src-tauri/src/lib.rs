// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::state::{AppState};
use lazy_static::lazy_static;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::sync::{Arc, Mutex};
use tauri::{ Manager, State};
use tauri_plugin_sql::{Migration, MigrationKind};

mod adapters;
mod commands;
mod database;
mod error;
mod models;
mod state;

lazy_static! {
    pub static ref MODEL_CONFIG_FILE: &'static str = "resources/en_US-libritts_r-medium.onnx.json";
    pub static ref MEDIA_DIR: String = {
        let Some(os_audio_dir) = dirs::audio_dir() else {
            todo!()
        };

        let os_audio_dir = os_audio_dir.as_path().to_str().unwrap();
        let audify_folder = "audify";

        let media_dir = format!("{os_audio_dir}/{audify_folder}");
        let _ = std::fs::create_dir(&media_dir);
        media_dir.clone()
    };
    pub static ref DATABASE_FILE: &'static str = "sqlite:eckko.db";
    // pub static ref DATABASE_FILE: &'static str = "sample.db";

}

pub const LAME_SIDECAR: &str = "lame";
pub const DATABASE_PATH: &str = "eckko.db";
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_default_tables",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181420_create_default_tables.sql"),
        },
        Migration {
            version: 2,
            description: "add_time_stamps_to_playlist_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181622_add_timestanp_to_playlist.sql"),
        },
        Migration {
            version: 3,
            description: "change_is_loved_to_boolean",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181730_change_is_loved_to_boolean.sql"),
        },
        Migration {
            version: 4,
            description: "create_app_settings_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181908_create_app_settings_table.sql"),
        },
        Migration {
            version: 5,
            description: "create_app_personalization_table",
            kind: MigrationKind::Up,
            sql: include_str!(
                "../migrations/20250812181958_create_app_personalization_table_table.sql"
            ),
        },
        Migration {
            version: 6,
            description: "create_cached_user_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812182051_create_cached_user_table.sql"),
        },
        Migration {
            version: 7,
            description: "remove_user_details_from_app_personalization",
            kind: MigrationKind::Up,
            sql: include_str!(
                "../migrations/20250812182303_remove_user_from_app_personalization.sql"
            ),
        },
        Migration {
            version: 8,
            description: "create_app_settings_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812182430_create_app_settings_if_not_exist.sql"),
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap();
            std::fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join(DATABASE_PATH);
            println!("db is here {:#?}", db_path);
            let app_state_result = tauri::async_runtime::block_on(async {
                let connection_options = SqliteConnectOptions::new()
                    .filename(db_path)
                    .create_if_missing(true);

                let pool = SqlitePool::connect_with(connection_options)
                    .await
                    .map_err(|e| e.to_string())?;

                    let state = AppState {
                    db: Arc::new(pool),
                  
                }; 
                Ok(state)
            });

            match app_state_result {
                Ok(app_state) => {
                    app.manage(app_state);
                    Ok(())
                }
                Err(e) => {
                    log::error!("{}", e);
                    Err(e)
                }
            }
        })
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(&DATABASE_FILE, migrations)
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
