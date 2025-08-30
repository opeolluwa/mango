use std::path::Path;

use crate::AERS_FILE_UPLOAD_PATH;
use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, CreateAudioBookRequest, UpdateBookRequest, UploadAssetRequest,
};
use crate::adapters::file::SaveFile;
use crate::adapters::jwt::Claims;
use crate::adapters::pagination::{PaginatedResponse, PaginationParams};
use crate::entities::audio_book::AudioBookEntity;
use crate::errors::repository_error::RepositoryError;
use crate::errors::service_error::ServiceError;
use crate::events::channels::EventChannel;
use crate::events::message::{ConvertDocument, DocumentConverted};
use crate::events::producer::EventPrducer;
use crate::repositories::audio_book_repository::{AudioBookRepository, AudioBookRepositoryExt};
use aers_utils::generate_file_name;
use axum_typed_multipart::{FieldData, TypedMultipart};
use regex::Regex;
use sqlx::Postgres;
use sqlx::pool::Pool;
use tempfile::NamedTempFile;
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
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

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
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<AudioBookEntity>, ServiceError>> + Send;

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

    fn unmark_favourite(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn fetch_favourites(
        &self,
        user_identifier: &Uuid,
        pagination_params: &PaginationParams,
    ) -> impl std::future::Future<
        Output = Result<PaginatedResponse<Vec<AudioBookEntity>>, ServiceError>,
    > + Send;

    fn save_file_to_disk(
        &self,
        document: FieldData<NamedTempFile>,
    ) -> Result<SaveFile, ServiceError>;

    fn processs_file_converted(
        &self,
        message: &DocumentConverted,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn fetch_many(
        &self,
        user_identifier: &Uuid,
        pagination_params: &PaginationParams,
    ) -> impl std::future::Future<
        Output = Result<PaginatedResponse<Vec<AudioBookEntity>>, ServiceError>,
    > + Send;
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
    ) -> Result<(), ServiceError> {
        let SaveFile {
            file_name,
            file_path,
        } = self.save_file_to_disk(document)?;

        let user_identifier = claim.user_identifier;
        let payload = ConvertDocument {
            playlist_identifier,
            file_name,
            user_identifier,
            file_path,
        };

        tokio::task::spawn(async move {
            let channel = EventChannel::ConvertDocumentToAudio;
            let message = payload;
            if let Err(err) = EventPrducer::new(&channel, message).send().await {
                log::error!("failed to publish message to {channel} due to {err}");
            }
        });
        Ok(())
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
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<AudioBookEntity>, ServiceError> {
        self.audio_book_repository
            .update(&request, book_identifier, user_identifier)
            .await
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

    async fn unmark_favourite(
        &self,
        book_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.audio_book_repository
            .unmark_favourite(book_identifier, user_identifier)
            .await?;

        Ok(())
    }

    async fn fetch_favourites(
        &self,
        user_identifier: &Uuid,
        pagination_params: &PaginationParams,
    ) -> Result<PaginatedResponse<Vec<AudioBookEntity>>, ServiceError> {
        let records = self
            .audio_book_repository
            .fetch_favourites(user_identifier, pagination_params)
            .await?;

        let response = PaginatedResponse {
            records,
            page: pagination_params.page.unwrap_or_default(),
            per_page: pagination_params.per_page.unwrap_or_default(),
            total_count: 5,
            total_pages: 5, //TODO: use complex query to read all at onece
        };

        Ok(response)
    }

    fn save_file_to_disk(
        &self,
        document: FieldData<NamedTempFile>,
    ) -> Result<SaveFile, ServiceError> {
        // 1. Prepare the file name
        let original_file_name = &document
            .metadata
            .file_name
            .clone()
            .unwrap_or_else(generate_file_name);

        // Normalize file name: replace whitespace with hyphens
        let sanitized_file_name = Regex::new(r"\s+")
            .unwrap()
            .replace_all(original_file_name, "-")
            .to_string();

        // 2. Save the PDF to disk
        let timestamp = chrono::Local::now().timestamp();
        let temp_dir = Path::new(AERS_FILE_UPLOAD_PATH);
        let pdf_path = temp_dir.join(format!("{timestamp}_{sanitized_file_name}.pdf"));
        if let Err(err) = document.contents.persist(&pdf_path) {
            log::error!("Failed to persist file: {err}");
            return Err(ServiceError::OperationFailed);
        };

        Ok(SaveFile {
            file_name: sanitized_file_name,
            file_path: pdf_path,
        })
    }

    async fn processs_file_converted(
        &self,
        message: &DocumentConverted,
    ) -> Result<(), ServiceError> {
        self.audio_book_repository
            .create(
                &CreateAudioBookRequest {
                    file_name: message.file_name.to_string(),
                    src: message.url.to_string(),
                    playlist_identifier: message.playlist_identifier,
                },
                &message.user_identifier,
            )
            .await?;
        Ok(())
    }
    async fn fetch_many(
        &self,
        user_identifier: &Uuid,
        pagination_params: &PaginationParams,
    ) -> Result<PaginatedResponse<Vec<AudioBookEntity>>, ServiceError> {
        self.audio_book_repository
            .find_many(user_identifier, pagination_params)
            .await
    }
}
