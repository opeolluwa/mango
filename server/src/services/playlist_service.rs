use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

use crate::entities::playlist::Playlist;
use crate::{
    adapters::playlist::CreatePlaylistRequest, errors::service_error::ServiceError,
    repositories::playlist_repository::PlaylistRepository,
    repositories::playlist_repository::PlaylistRepositoryExt,
};

#[derive(Clone)]
pub struct PlaylistService {
    playlist_repository: PlaylistRepository,
}

impl PlaylistService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            playlist_repository: PlaylistRepository::init(pool),
        }
    }
}

pub trait PlaylistServiceExt {
    fn create_new_playlist(
        &self,
        request: &CreatePlaylistRequest,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Uuid, ServiceError>>;
    fn fetch_one(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Playlist>, ServiceError>> + Send;

    fn delete_playlist(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn update_playlist(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl PlaylistServiceExt for PlaylistService {
    async fn create_new_playlist(
        &self,
        request: &CreatePlaylistRequest,
        user_identifier: &Uuid,
    ) -> Result<Uuid, ServiceError> {
        self.playlist_repository
            .create_new(request, user_identifier)
            .await
            .map_err(ServiceError::from)
    }
    async fn fetch_one(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<Playlist>, ServiceError> {
        self.playlist_repository
            .fetch_one(playlist_identifier, user_identifier)
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_playlist(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.playlist_repository
            .delete(playlist_identifier, user_identifier)
            .await
            .map_err(ServiceError::from)
    }

    async fn update_playlist(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.playlist_repository
            .delete(playlist_identifier, user_identifier)
            .await
            .map_err(ServiceError::from)
    }
}
