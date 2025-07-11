use sqlx::Postgres;
use sqlx::pool::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    adapters::audio_books::{AudioBook, CreateAudioBook, UpdateAudioBook},
    errors::common_service_error::ServiceError,
};

#[derive(Clone)]
pub struct AudioBookRepository {
    pool: Arc<Pool<Postgres>>,
}

impl AudioBookRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait AudioBookRepositoryExt {
    fn create(
        &self,
        payload: &CreateAudioBook,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn find_one(
        &self,
        payload: &CreateAudioBook,
    ) -> impl std::future::Future<Output = Result<Option<AudioBook>, ServiceError>> + Send;

    fn find_many(
        &self,
        payload: &CreateAudioBook,
    ) -> impl std::future::Future<Output = Result<Vec<AudioBook>, ServiceError>> + Send;

    fn update(
        &self,
        payload: &UpdateAudioBook,
    ) -> impl std::future::Future<Output = Result<Option<AudioBook>, ServiceError>> + Send;

    fn delete(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn add_to_playlist(
        &self,
        identifier: &Uuid,
        playlist_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn remove_from_playlist(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn mark_favourite(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn unmark_favourite(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AudioBookRepositoryExt for AudioBookRepository {
    async fn create(&self, payload: &CreateAudioBook) -> Result<(), ServiceError> {
        todo!()
    }

    async fn find_one(&self, payload: &CreateAudioBook) -> Result<Option<AudioBook>, ServiceError> {
        todo!()
    }

    async fn find_many(&self, payload: &CreateAudioBook) -> Result<Vec<AudioBook>, ServiceError> {
        todo!()
    }

    async fn update(&self, payload: &UpdateAudioBook) -> Result<Option<AudioBook>, ServiceError> {
        todo!()
    }

    async fn delete(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    async fn add_to_playlist(
        &self,
        identifier: &Uuid,
        playlist_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        todo!()
    }

    async fn remove_from_playlist(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    async fn mark_favourite(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    async fn unmark_favourite(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        todo!()
    }
}
