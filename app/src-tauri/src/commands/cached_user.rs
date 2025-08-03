use tauri::{Runtime, State};

use crate::adapters::cached_user::CreateCachedUser;
use crate::database::ModelTrait;
use crate::error::CommandError;
use crate::error::DbError;
use crate::models::cached_user::CachedUser;
use crate::state::AppState;

use std::sync::Arc;

// set cached user
#[tauri::command]
pub async fn set_cached_user<R: Runtime>(
    state: State<'_, Arc<AppState>>,
    user: CreateCachedUser,
    _: tauri::Window<R>,
) -> Result<(), CommandError> {
    let pool = state.db.clone();

    CachedUser::from_adapter(user)
        .save(&pool)
        .await
        .map_err(|err| {
            log::error!("{err}");
            DbError::QueryFailed
        })?;

    Ok(())
}

// fetch cached user
#[tauri::command]
pub async fn fetch_cached_user<R: Runtime>(
    state: State<'_, Arc<AppState>>,
    user_identifier: String,
    _: tauri::Window<R>,
) -> Result<Option<CachedUser>, CommandError> {
    let pool = state.db.clone();

    if user_identifier.is_empty() {
        return Err(CommandError::from("User identifier cannot be empty"));
    }

    let user_identifier = uuid::Uuid::parse_str(&user_identifier)
        .map_err(|_| CommandError::from("Invalid UUID format"))?;

    let result =
        sqlx::query_as::<_, CachedUser>(r#"SELECT * FROM cached_user WHERE identifier = ?"#)
            .bind(user_identifier)
            .fetch_optional(&*pool)
            .await
            .map_err(|err| {
                log::error!("{err}");
                DbError::QueryFailed
            })?;

    Ok(result)
}
