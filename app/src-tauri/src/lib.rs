// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::state::AppState;
use dirs;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tauri_plugin_sql::{Migration, MigrationKind};

// mod commands;

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
    pub static ref DATABASE_FILE: &'static str = "sqlite:echo.db";
}

pub const LAME_SIDECAR: &str = "lame";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_default_tables",
            sql: r#"
--- AUDIO BOOKS TABLE ---
CREATE TABLE IF NOT EXISTS audio_books
(
    identifier TEXT PRIMARY KEY NOT NULL,
    title      TEXT,
    created_at TEXT,
    updated_at TEXT,
    is_loved   TINYINT DEFAULT 0
);

--- PLAYLIST TABLE ---
CREATE TABLE IF NOT EXISTS playlist
(
    identifier  TEXT PRIMARY KEY NOT NULL,
    name        TEXT,
    description TEXT
);

--- HISTORY TABLE ---
CREATE TABLE IF NOT EXISTS history
(
    identifier            TEXT PRIMARY KEY NOT NULL,
    audio_book_identifier TEXT,
    FOREIGN KEY (audio_book_identifier) REFERENCES audio_books(identifier) ON DELETE CASCADE
);

SELECT name
FROM sqlite_master
WHERE type = 'table';
        "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_time_stamps_to_playlist_table",
            kind: MigrationKind::Up,
            sql: r#"
-- Add timestamp columns to playlist table--
ALTER TABLE playlist ADD COLUMN created_at TEXT;
ALTER TABLE playlist ADD COLUMN updated_at TEXT;

-- Update existing rows with current timestamp--
UPDATE playlist
SET created_at = CURRENT_TIMESTAMP,
    updated_at = CURRENT_TIMESTAMP
WHERE created_at IS NULL;

-- Make the columns NOT NULL for future inserts--
PRAGMA foreign_keys=off;

CREATE TABLE playlist_new
(
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
    ];
    tauri::Builder::default()
        .manage(Arc::new(AppState {
            current_audio_book: Mutex::new(None),
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(&DATABASE_FILE, migrations)
                .build(),
        )
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(desktop)]
            let _ = app
                .handle()
                .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // commands::synthesize_audio,
            // commands::read_library,
            // commands::play_audio_book,
            // commands::pause_audio_book,
            // commands::set_audio_book_volume,
            // commands::seek_audio_book_to_position,
            // commands::set_audio_book_playback_speed,
            // commands::resume_playing_audio_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
