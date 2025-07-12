use std::path::Path;

use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, CreateAudioBookRequest, UpdateBookRequest, UpdateBookResponse,
    UploadAssetRequest,
};
use crate::adapters::jwt::Claims;
use crate::entities::audio_book::AudioBookEntity;
use crate::errors::repository_error::RepositoryError;
use crate::errors::service_error::ServiceError;
use crate::repositories::audio_book_repository::{AudioBookRepository, AudioBookRepositoryExt};
use aers_imagekit_client::{ImagekitClient, ImagekitUploadResponse};
use aers_utils::generate_file_name;
use axum_typed_multipart::TypedMultipart;
use sqlx::Postgres;
use sqlx::pool::Pool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AudioBooksService {
    audio_book_repository: AudioBookRepository,
}

pub trait AudioBooksServiceExt {
    fn create_new(
        &self,
        request: TypedMultipart<UploadAssetRequest>,
        claim: &Claims,
    ) -> impl std::future::Future<Output = Result<Uuid, ServiceError>> + Send;

    fn fetch_one(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<AudioBookEntity, ServiceError>> + Send;

    fn add_to_playlist(
        &self,
        request: &AddBookToPlaylistRequest,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn remove_from_playlist(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn update_book(
        &self,
        request: UpdateBookRequest,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<UpdateBookResponse, ServiceError>> + Send;

    fn delete_book(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn mark_favourite(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AudioBooksService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            audio_book_repository: AudioBookRepository::init(pool),
        }
    }
}

impl AudioBooksServiceExt for AudioBooksService {
    async fn create_new(
        &self,
        TypedMultipart(UploadAssetRequest {
            document,
            playlist_identifier,
        }): TypedMultipart<UploadAssetRequest>,
        claim: &Claims,
    ) -> Result<Uuid, ServiceError> {
        let file_name = document
            .metadata
            .file_name
            .clone()
            .unwrap_or(generate_file_name());
        let file_path = Path::new("/tmp")
            .join(chrono::Local::now().timestamp().to_string())
            .join(&file_name);

        if let Err(err) = document.contents.persist(&file_path) {
            log::error!("error processing file due to {}", err.to_string());
            return Err(ServiceError::OperationFailed);
        }

        let Ok(ImagekitUploadResponse { url }) = ImagekitClient::new().upload(&file_path);
        let request = CreateAudioBookRequest {
            file_name: file_name.to_owned(),
            src: url,
            playlist_identifier,
        };

        let book_identifier = self
            .audio_book_repository
            .create(&request, &claim.user_identifier)
            .await?;

        Ok(book_identifier)
    }

    async fn fetch_one(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<AudioBookEntity, ServiceError> {
        let book = self
            .audio_book_repository
            .find_one(book_identifier, user_identifier)
            .await?
            .ok_or(RepositoryError::RecordNotFound)?;

        Ok(book)
    }

    async fn add_to_playlist(
        &self,
        request: &AddBookToPlaylistRequest,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.audio_book_repository
            .add_to_playlist(
                &request.book_identifier,
                &request.playlist_identifier,
                user_identifier,
            )
            .await?;

        Ok(())
    }

    async fn remove_from_playlist(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.audio_book_repository
            .remove_from_playlist(book_identifier, user_identifier)
            .await?;
        Ok(())
    }

    async fn update_book(
        &self,
        request: UpdateBookRequest,
        user_identifier: &Uuid,
    ) -> Result<UpdateBookResponse, ServiceError> {
        todo!()
    }

    async fn delete_book(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.audio_book_repository
            .delete(book_identifier, user_identifier)
            .await?;
        Ok(())
    }

    async fn mark_favourite(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.audio_book_repository
            .mark_favourite(book_identifier, user_identifier)
            .await?;

        Ok(())
    }
}
