use tauri::{Runtime, State};

use crate::adapters::theme::Theme;
use crate::error::CommandError;
use crate::error::DbError;
use crate::models::app_personaliation::AppPersonalization;
use crate::state::AppState;
 use std::sync::Mutex;
use std::sync::Arc;

// fetch app personalization
#[tauri::command]
pub async fn fetch_app_personalization<R: Runtime>(
    state: State<'_, Arc<Mutex<AppState>>>,
    _: tauri::Window<R>,
) -> Result<AppPersonalization, CommandError> {
    let pool = state.clone().lock().unwrap().db.clone();

    let result = sqlx::query_as::<_, AppPersonalization>(r#"SELECT * FROM app_personalization"#)
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            log::error!("{err}");
            DbError::QueryFailed
        })?;

    println!("result: {:#?}", result);
    Ok(result)
}

// update app personalization
#[tauri::command]
pub async fn update_app_personalization<R: Runtime>(
   state: State<'_, Arc<Mutex<AppState>>>,
    _: tauri::Window<R>,
) -> Result<AppPersonalization, CommandError> {
    let pool = state.clone().lock().unwrap().db.clone();

    let result = sqlx::query_as::<_, AppPersonalization>(r#"SELECT * FROM app_personalization"#)
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            log::error!("{err}");
            DbError::QueryFailed
        })?;

    Ok(result)
}

// set the theme
#[tauri::command]
pub async fn set_theme<R: Runtime>(
   state: State<'_, Arc<Mutex<AppState>>>,
    theme: Theme,
    _: tauri::Window<R>,
) -> Result<AppPersonalization, CommandError> {
    let pool = state.clone().lock().unwrap().db.clone();

    let result = sqlx::query_as::<_, AppPersonalization>(
        r#"UPDATE app_personalization SET theme = ? WHERE id = 1"#,
    )
    .bind(theme.to_string())
    .fetch_one(&*pool)
    .await
    .map_err(|err| {
        log::error!("{err}");

        DbError::QueryFailed
    })?;

    Ok(result)
}
