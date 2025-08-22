// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::state::AppState;
use anyhow::{Context, Result};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};
use tracing::{error, info, warn};

mod adapters;
mod commands;
mod database;
mod error;
mod models;
mod state;

/// Application configuration constants
pub struct AppConfig {
    pub model_config_file: &'static str,
    pub media_dir_name: &'static str,
    pub database_file: &'static str,
    pub database_path: &'static str,
    pub lame_sidecar: &'static str,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model_config_file: "resources/en_US-libritts_r-medium.onnx.json",
            media_dir_name: "audify",
            database_file: "sqlite:eckko.db",
            database_path: "eckko.db",
            lame_sidecar: "lame",
        }
    }
}

/// Initialize the media directory in the user's audio folder
fn initialize_media_directory(config: &AppConfig) -> Result<PathBuf> {
    let audio_dir = dirs::audio_dir()
        .context("Failed to find system audio directory")?;
    
    let media_dir = audio_dir.join(config.media_dir_name);
    
    std::fs::create_dir_all(&media_dir)
        .with_context(|| format!("Failed to create media directory: {}", media_dir.display()))?;
    
    info!("Media directory initialized at: {}", media_dir.display());
    Ok(media_dir)
}

/// Load database migrations from embedded SQL files
fn load_migrations() -> Vec<Migration> {
    vec![
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
            description: "create_app_settings_table_if_not_exists",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812182430_create_app_settings_if_not_exist.sql"),
        },
    ]
}

/// Initialize the database connection and state
async fn initialize_database(app_data_dir: &PathBuf, config: &AppConfig) -> Result<AppState> {
    let db_path = app_data_dir.join(config.database_path);
    info!("Initializing database at: {}", db_path.display());

    let connection_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .pragma("journal_mode", "WAL") // Enable WAL mode for better performance
        .pragma("synchronous", "NORMAL") // Balanced durability vs performance
        .pragma("cache_size", "1000") // Set cache size
        .pragma("temp_store", "memory"); // Use memory for temp storage

    let pool = SqlitePool::connect_with(connection_options)
        .await
        .with_context(|| format!("Failed to connect to database at {}", db_path.display()))?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .context("Failed to test database connection")?;

    info!("Database connection established successfully");
    
    Ok(AppState { db: Arc::new(pool) })
}

/// Setup function for the Tauri application
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::default();

    // Initialize media directory (non-blocking)
    if let Err(e) = initialize_media_directory(&config) {
        warn!("Failed to initialize media directory: {}", e);
        // Continue execution as this might not be critical
    }

    // Setup app data directory
    let app_data_dir = app.path().app_data_dir()
        .context("Failed to get app data directory")?;
    
    std::fs::create_dir_all(&app_data_dir)
        .with_context(|| format!("Failed to create app data directory: {}", app_data_dir.display()))?;

    info!("App data directory: {}", app_data_dir.display());

    // Initialize database synchronously in setup
    let app_state = tauri::async_runtime::block_on(async {
        initialize_database(&app_data_dir, &config).await
    })?;

    // Store the app state
    app.manage(Arc::new(app_state));
    
    info!("Application setup completed successfully");
    Ok(())
}

/// Initialize logging
fn initialize_logging() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging first
    if let Err(e) = initialize_logging() {
        eprintln!("Failed to initialize logging: {}", e);
    }

    let config = AppConfig::default();
    let migrations = load_migrations();

    let builder_result = tauri::Builder::default()
        // Core plugins
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_pinia::init())
        
        // SQL plugin with migrations
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(config.database_file, migrations)
                .build()
        )
        
        // Stronghold plugin with proper error handling
        .plugin(
            tauri_plugin_stronghold::Builder::new(|_password| {
                // TODO: Implement proper password handling
                warn!("Stronghold password callback not implemented");
                vec![]
            }).build()
        )
        
        // Application setup
        .setup(|app| {
            if let Err(e) = setup_app(app) {
                error!("Application setup failed: {}", e);
                return Err(e);
            }
            Ok(())
        })
        
        // Command handlers
        .invoke_handler(tauri::generate_handler![
            commands::app_personalization::fetch_app_personalization,
            commands::app_personalization::update_app_personalization,
            commands::app_personalization::set_theme,
            commands::cached_user::set_cached_user,
            commands::cached_user::fetch_cached_user,
            commands::app_settings::fetch_app_settings,
            commands::app_settings::initalize_app_settings,
        ])
        
        // Build and run
        .build(tauri::generate_context!()).expect("build");

    match builder_result {
        Ok(app) => {
            info!("Starting Tauri application");
            if let Err(e) = app.run(|_app, event| {
                match event {
                    tauri::RunEvent::ExitRequested { api, .. } => {
                        info!("Application exit requested");
                        api.prevent_exit();
                    }
                    tauri::RunEvent::WindowEvent { event, .. } => {
                        match event {
                            tauri::WindowEvent::CloseRequested { .. } => {
                                info!("Window close requested");
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }) {
                error!("Error running Tauri application: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to build Tauri application: {}", e);
            std::process::exit(1);
        }
    }
}