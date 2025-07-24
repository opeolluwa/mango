use crate::{database, LAME_SIDECAR, MEDIA_DIR, MODEL_CONFIG_FILE};
// use libaudify::core::Audify;
// use libaudify::error::AudifyError;
use std::path::{Path, PathBuf};
use std::thread;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager};
use tauri::{Runtime, State};

use crate::adapters::AudioSynthesisEvent;
use crate::adapters::CurrentAudioMetadata;
use crate::adapters::Theme;
use crate::adapters::AUDIO_PROCESSING_EVENT;
use crate::adapters::CURRENTLY_PLAYING_EVENT;
use crate::adapters::FINISHED_AUDIO_PROCESSING_EVENT;
use crate::database::AppPersonalization;
use crate::database::AppSettings;
use crate::error::CommandError;
use crate::error::DbError;
use crate::state::AppState;
use crate::database::ModelTrait;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::sync::Arc;

#[tauri::command]
pub async fn initalize_app_settings(state: State<'_, Arc<AppState>>) -> Result<(), CommandError> {
    let pool = state.db.clone();

    sqlx::query(r"INSERT INTO app_settings (app_initialized) VALUES (1)")
        .execute(&*pool)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    let app_personalization = AppPersonalization::new(None, None, None, None, None, None);
    app_personalization.save(&pool).await?;

    Ok(())
}

#[tauri::command]
pub async fn fetch_app_settings<R: Runtime>(
    state: State<'_, Arc<AppState>>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<AppSettings, CommandError> {
    let pool = state.db.clone();

    let result = sqlx::query_as::<_, AppSettings>(r#"SELECT * FROM app_settings"#)
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            log::error!("{}", err);
            DbError::QueryFailed
        })?;

    Ok(result)
}

// fetch app personalization
#[tauri::command]
pub async fn fetch_app_personalization<R: Runtime>(
    state: State<'_, Arc<AppState>>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<AppPersonalization, CommandError> {
    let pool = state.db.clone();

    let result =
        sqlx::query_as::<_, AppPersonalization>(r#"SELECT * FROM app_personalization LIMIT 1"#)
            .fetch_one(&*pool)
            .await
            .map_err(|err| {
                log::error!("{}", err);
                DbError::QueryFailed
            })?;

    Ok(result)
}

// update app personalization
#[tauri::command]
pub async fn update_app_personalization<R: Runtime>(
    state: State<'_, Arc<AppState>>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<AppPersonalization, CommandError> {
    let pool = state.db.clone();

    let result =
        sqlx::query_as::<_, AppPersonalization>(r#"SELECT * FROM app_personalization LIMIT 1"#)
            .fetch_one(&*pool)
            .await
            .map_err(|err| {
                log::error!("{}", err);
                DbError::QueryFailed
            })?;

    Ok(result)
}

// set the theme
#[tauri::command]
pub async fn set_theme<R: Runtime>(
    state: State<'_, Arc<AppState>>,
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    theme: Theme,
) -> Result<AppPersonalization, CommandError> {
    let pool = state.db.clone();

    let result = sqlx::query_as::<_, AppPersonalization>(
        r#"UPDATE app_personalization SET theme = ? WHERE id = 1"#,
    )
    .fetch_one(&*pool)
    .await
    .map_err(|err| {
        log::error!("{}", err);

        DbError::QueryFailed
    })?;

    Ok(result)
}
