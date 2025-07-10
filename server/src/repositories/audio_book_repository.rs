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
    async fn create(&self, payload: &CreateAudioBook) -> Result<(), ServiceError>;

    async fn find_one(&self, payload: &CreateAudioBook) -> Result<Option<AudioBook>, ServiceError>;

    async fn find_many(&self, payload: &CreateAudioBook) -> Result<Vec<AudioBook>, ServiceError>;

    async fn update(&self, payload: &UpdateAudioBook) -> Result<Option<AudioBook>, ServiceError>;

    async fn delete(&self, identifier: &Uuid) -> Result<(), ServiceError>;

    async fn add_to_playlist(
        &self,
        identifier: &Uuid,
        playlist_identifier: &Uuid,
    ) -> Result<(), ServiceError>;

    async fn remove_from_playlist(&self, identifier: &Uuid) -> Result<(), ServiceError>;

    async fn mark_favourite(&self, identifier: &Uuid) -> Result<(), ServiceError>;

    async fn unmark_favourite(&self, identifier: &Uuid) -> Result<(), ServiceError>;
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
