// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::state::AppState;
use lazy_static::lazy_static;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};

mod adapters;
mod commands;
mod database;
mod error;
mod state;
mod models;

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
            sql: r#"
-- AUDIO BOOKS TABLE --
CREATE TABLE IF NOT EXISTS audio_books (
    identifier TEXT PRIMARY KEY NOT NULL,
    title      TEXT,
    created_at TEXT,
    updated_at TEXT,
    is_loved   TINYINT DEFAULT 0
);

-- PLAYLIST TABLE --
CREATE TABLE IF NOT EXISTS playlist (
    identifier  TEXT PRIMARY KEY NOT NULL,
    name        TEXT,
    description TEXT
);

-- HISTORY TABLE --
CREATE TABLE IF NOT EXISTS history (
    identifier            TEXT PRIMARY KEY NOT NULL,
    audio_book_identifier TEXT,
    FOREIGN KEY (audio_book_identifier) REFERENCES audio_books(identifier) ON DELETE CASCADE
);

-- VERIFY TABLES CREATED --
SELECT name FROM sqlite_master WHERE type = 'table';
"#,
        },
        Migration {
            version: 2,
            description: "add_time_stamps_to_playlist_table",
            kind: MigrationKind::Up,
            sql: r#"
-- Add timestamp columns to playlist table --
ALTER TABLE playlist ADD COLUMN created_at TEXT;
ALTER TABLE playlist ADD COLUMN updated_at TEXT;

-- Update existing rows with current timestamp --
UPDATE playlist
SET created_at = CURRENT_TIMESTAMP,
    updated_at = CURRENT_TIMESTAMP
WHERE created_at IS NULL;

-- Make the columns NOT NULL for future inserts --
PRAGMA foreign_keys=off;

CREATE TABLE playlist_new (
    identifier  TEXT PRIMARY KEY NOT NULL,
    name        TEXT,
    description TEXT,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

INSERT INTO playlist_new SELECT * FROM playlist;
DROP TABLE playlist;
ALTER TABLE playlist_new RENAME TO playlist;

PRAGMA foreign_keys=on;
"#,
        },
        Migration {
            version: 3,
            description: "change_is_loved_to_boolean",
            kind: MigrationKind::Up,
            sql: r#"
-- Rename original table --
ALTER TABLE audio_books RENAME TO audio_books_old;

-- Create new table with updated schema --
CREATE TABLE IF NOT EXISTS audio_books (
    identifier TEXT PRIMARY KEY NOT NULL,
    title      TEXT,
    author     TEXT,
    created_at TEXT,
    updated_at TEXT,
    is_loved   BOOLEAN DEFAULT 0
);

-- Copy data over --
INSERT INTO audio_books (identifier, title, created_at, updated_at, is_loved)
SELECT identifier, title, created_at, updated_at, is_loved FROM audio_books_old;

-- Drop old table --
DROP TABLE audio_books_old;
"#,
        },
        Migration {
            version: 4,
            description: "create_app_settings_table",
            kind: MigrationKind::Up,
            sql: r#"
-- Create a new table that stores app settings --
CREATE TABLE IF NOT EXISTS app_settings (
    app_initialized BOOLEAN DEFAULT 0
);
"#,
        },
        Migration {
            version: 5,
            description: "create_app_personalization_table",
            kind: MigrationKind::Up,
            sql: r#"
-- Create a new table that stores app personalization --
CREATE TABLE IF NOT EXISTS app_personalization (
    theme           TEXT DEFAULT 'light',
    language        TEXT,
    first_name      TEXT,
    last_name       TEXT,
    email           TEXT,
    preferred_voice TEXT
);
"#,
        },
        Migration {
            version: 6,
            description: "create_cached_user_table",
            kind: MigrationKind::Up,
            sql: r#"
-- Create a new table that stores cached user information --
CREATE TABLE IF NOT EXISTS cached_user (
    identifier TEXT PRIMARY KEY NOT NULL,
    first_name TEXT,
    last_name  TEXT,
    email      TEXT,
    avatar_url TEXT
);
"#,
        },
        Migration {
            version: 7,
            description: "remove_user_details_from_app_personalization",
            kind: MigrationKind::Up,
            sql: r#"
-- Remove user-related fields from app_personalization table --
CREATE TABLE IF NOT EXISTS app_personalization_new (
    theme           TEXT DEFAULT 'light',
    language        TEXT,
    preferred_voice TEXT
);

INSERT INTO app_personalization_new (theme, language, preferred_voice)
SELECT theme, language, preferred_voice FROM app_personalization;

DROP TABLE app_personalization;
ALTER TABLE app_personalization_new RENAME TO app_personalization;
"#,
        },

             Migration {
            version: 8,
            description: "create_app_settings_table",
            kind: MigrationKind::Up,
            sql: r#"
            CREATE TABLE app_settings(app_initialized)"#
             }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap();
            std::fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join(DATABASE_PATH);

            let app_state_result = tauri::async_runtime::block_on(async {
                let connection_options = SqliteConnectOptions::new()
                    .filename(db_path)
                    .create_if_missing(true);

                let pool = SqlitePool::connect_with(connection_options)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(AppState { db: Arc::new(pool) })
            });

            match app_state_result {
                Ok(app_state) => {
                    app.manage(Arc::new(app_state));
                    Ok(())
                }
                Err(e) => Err(e),
            }
        })
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(&DATABASE_FILE, migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::app_settings::fetch_app_settings,
            commands::app_personalization::fetch_app_personalization,
            commands::app_personalization::update_app_personalization,
            commands::app_personalization::set_theme,
            commands::cached_user::set_cached_user,
            commands::cached_user::fetch_cached_user,
            commands::app_settings::initalize_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
