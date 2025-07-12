use sqlx::Postgres;
use sqlx::pool::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    adapters::{
        audio_books::{AudioBook, CreateAudioBookRequest, UpdateAudioBook},
        pagination::PaginationParams,
    },
    entities::audio_book::AudioBookEntity,
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
        payload: &CreateAudioBookRequest,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Uuid, ServiceError>> + Send;

    fn find_one(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<AudioBookEntity>, ServiceError>> + Send;

    fn find_many(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> impl std::future::Future<Output = Result<Vec<AudioBookEntity>, ServiceError>> + Send;

    fn update(
        &self,
        payload: &UpdateAudioBook,
    ) -> impl std::future::Future<Output = Result<Option<AudioBook>, ServiceError>> + Send;

    fn delete(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn add_to_playlist(
        &self,
        identifier: &Uuid,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn remove_from_playlist(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn mark_favourite(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn unmark_favourite(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AudioBookRepositoryExt for AudioBookRepository {
    async fn create(
        &self,
        payload: &CreateAudioBookRequest,
        user_identifier: &Uuid,
    ) -> Result<Uuid, ServiceError> {
        let identifier = Uuid::new_v4();

        sqlx::query(r#"INSERT INTO audio_books(identifier, name, src, user_identifier, playlist_identifier) VALUES($1, $2, $3, $4, $5) "#)
            .bind(&identifier)
            .bind(&payload.file_name)
            .bind(&payload.src)
            .bind(&user_identifier)
            .bind(&payload.playlist_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(ServiceError::from)?;

        Ok(identifier)
    }

    async fn find_one(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<AudioBookEntity>, ServiceError> {
        let audio_book = sqlx::query_as::<_, AudioBookEntity>(
            r#"SELECT * audio_books WHERE identifier = $1 AND user_identifier =$2"#,
        )
        .bind(book_identifier)
        .bind(user_identifier)
        .fetch_one(self.pool.as_ref())
        .await
        .ok();

        Ok(audio_book)
    }

    async fn find_many(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> Result<Vec<AudioBookEntity>, ServiceError> {
        sqlx::query_as::<_, AudioBookEntity>(
            r#"SELECT * audio_books WHERE user_identifier = $1 LIMIT $2 OFFSET $3 SORT BY created_at"#,
        )
        .bind(user_identifier)
        .bind(pagination.per_page.unwrap_or_default().to_string())
        .bind(pagination.page.unwrap_or_default().to_string())
        .fetch_all(self.pool.as_ref())
        .await.map_err(ServiceError::from)
    }

    async fn update(&self, _payload: &UpdateAudioBook) -> Result<Option<AudioBook>, ServiceError> {
        todo!()
    }

    async fn delete(&self, identifier: &Uuid, user_identifier: &Uuid) -> Result<(), ServiceError> {
        sqlx::query(r#"DELETE FROM audio_books WHERE identifier =$1 AND user_identifier =$2"#)
            .bind(identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

    async fn add_to_playlist(
        &self,
        identifier: &Uuid,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        sqlx::query(r#"UPDATE audio_books  SET playlist_identifier =$1 WHERE identifier =$2 AND user_identifier =$2"#)
        .bind(playlist_identifier)
            .bind(identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

    async fn remove_from_playlist(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        sqlx::query(r#"UPDATE audio_books  SET playlist_identifier =null WHERE identifier =$2 AND user_identifier =$2"#)
            .bind(identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

    async fn mark_favourite(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        sqlx::query(r#"UPDATE audio_books  SET starred = true WHERE identifier =$1 AND user_identifier =$2"#)
            .bind(identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

    async fn unmark_favourite(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        sqlx::query(r#"UPDATE audio_books SET starred = false WHERE identifier =$1 AND user_identifier =$2"#)
            .bind(identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }
}
