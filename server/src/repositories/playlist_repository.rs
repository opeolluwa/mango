use crate::adapters::pagination::PaginationParams;
use crate::adapters::playlist::{CreatePlaylistRequest, UpdatePlaylistRequest};
use crate::entities::playlist::Playlist;
use crate::errors::repository_error::RepositoryError;
use sqlx::Postgres;
use sqlx::pool::Pool;
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct PlaylistRepository {
    pool: Arc<Pool<Postgres>>,
}

impl PlaylistRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait PlaylistRepositoryExt {
    fn create_new(
        &self,
        request: &CreatePlaylistRequest,
        user_identifiee: &Uuid,
    ) -> impl std::future::Future<Output = Result<Uuid, RepositoryError>> + Send;
    fn update(
        &self,
        request: &UpdatePlaylistRequest,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
    fn delete(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
    fn fetch_one(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Playlist>, RepositoryError>> + Send;
    fn fetch_many(
        &self,
        pagination: &PaginationParams,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Playlist>, RepositoryError>> + Send;
}

impl PlaylistRepositoryExt for PlaylistRepository {
    async fn create_new(
        &self,
        request: &CreatePlaylistRequest,
        user_identifier: &Uuid,
    ) -> Result<Uuid, RepositoryError> {
        let playlist_identifier = Uuid::new_v4();

        sqlx::query(r#"INSERT INTO playlists (identifier, name, description, user_identifier) VALUES ($1, $2, $3, $4)"#).bind(Uuid::new_v4()) 
        .bind(&playlist_identifier)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&user_identifier).execute(self.pool.as_ref()).await.map_err(|err|RepositoryError::SqlxError(err))?;
        Ok(playlist_identifier)
    }

    async fn update(
        &self,
        request: &UpdatePlaylistRequest,

        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), RepositoryError> {
        let Some(playlist) = self.fetch_one(playlist_identifier, user_identifier).await? else {
            return Err(RepositoryError::RecordNotFound);
        };

        sqlx::query(r#"UPDATE playlists SET name = $1, description = $2 WHERE identifier = $3 AND user_identifier = $4"#).bind(&request.name.clone().unwrap_or(playlist.name)).bind(request.description.clone().unwrap_or(playlist.description)).bind(user_identifier).execute(self.pool.as_ref()).await.map_err(|err|RepositoryError::SqlxError(err))?;
        Ok(())
    }

    async fn delete(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), RepositoryError> {
        let Some(_) = self
            .fetch_one(playlist_identifier, user_identifier)
            .await
            .ok()
        else {
            return Err(RepositoryError::RecordNotFound);
        };
        sqlx::query("DELETE playlists WHERE identifier = $1 AND user_identifier = $2")
            .bind(playlist_identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::SqlxError(err))?;
        Ok(())
    }

    async fn fetch_one(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<Playlist>, RepositoryError> {
        let playlist = sqlx::query_as::<_, Playlist>(
            "SELECT * FROM playlists WHERE identifier = $1 AND user_identifier = $2",
        )
        .bind(playlist_identifier)
        .bind(user_identifier)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|err| RepositoryError::SqlxError(err))
        .ok();

        Ok(playlist)
    }

    async fn fetch_many(
        &self,
        pagination: &PaginationParams,
        user_identifier: &Uuid,
    ) -> Result<Vec<Playlist>, RepositoryError> {
        let limit = pagination.page.unwrap_or_default() - 1;
        let offset = pagination.per_page.unwrap_or_default();

        let playlists = sqlx::query_as::<_, Playlist>(
            "SELECT * FROM playlists WHERE user_identifier = $1 LIMIT $2 OFFSET $3",
        )
        .bind(user_identifier)
        .bind(limit.to_string())
        .bind(offset.to_string())
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|err| RepositoryError::SqlxError(err))?;

        Ok(playlists)
    }
}
