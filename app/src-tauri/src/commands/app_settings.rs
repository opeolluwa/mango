use tauri::{Runtime, State};

use crate::database::AppPersonalization;
use crate::database::AppSettings;
use crate::database::ModelTrait;
use crate::error::CommandError;
use crate::error::DbError;
use crate::state::AppState;

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
    _: tauri::Window<R>,
) -> Result<AppSettings, CommandError> {
    let pool = state.db.clone();

    let result = sqlx::query_as::<_, AppSettings>(r#"SELECT * FROM app_settings"#)
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            log::error!("{err}");
            DbError::QueryFailed
        })?;

    Ok(result)
}
