use std::path::Path;

use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, CreateAudioBookRequest, UpdateBookRequest, UploadAssetRequest,
};
use crate::adapters::jwt::Claims;
use crate::adapters::pagination::{PaginatedResponse, PaginationParams};
use crate::entities::audio_book::AudioBookEntity;
use crate::errors::repository_error::RepositoryError;
use crate::errors::service_error::ServiceError;
use crate::repositories::audio_book_repository::{AudioBookRepository, AudioBookRepositoryExt};
use crate::{AERS_EXPORT_PATH, AERS_FILE_UPLOAD_PATH};
use aers_audify::Audify;
use aers_imagekit_client::ImagekitClient;
use aers_utils::{extract_env, generate_file_name};
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
        // tokio::task::spawn(async move {
        let file_name = document
            .metadata
            .file_name
            .clone()
            .unwrap_or(generate_file_name());

        let temp_dir = Path::new(AERS_FILE_UPLOAD_PATH);
        let file_path = temp_dir.join(format!(
            "{time_stamp}_{file_name}.pdf",
            time_stamp = chrono::Local::now().timestamp()
        ));

        // create file object
        if let Err(err) = document.contents.persist(&file_path) {
            log::error!("error processing file due to {}", err);
            return Err(ServiceError::OperationFailed);
        }

        let config_path = "resources/models/en_US-libritts_r-medium.onnx.json";
        let audify_client = Audify::new(config_path);

        let pdf_path = file_path.to_str().ok_or(ServiceError::OperationFailed)?;

        let audio_output = format!("{}/{}.wav", AERS_EXPORT_PATH, file_name);
        audify_client
            .synthesize_pdf(pdf_path, &audio_output)
            .map_err(|err| {
                log::error!("failed to process document due to {} ", err);
                ServiceError::OperationFailed
            })?;

        let private_key = extract_env::<String>("IMAGEKIT_PRIVATE_KEY").unwrap();
        let public_key = extract_env::<String>("IMAGEKIT_PUBLIC_KEY").unwrap();

        let imagekit_upload_response = ImagekitClient::new(&public_key, &private_key)
            .map_err(|err| {
                log::error!("error creating client due to {}", err);
                ServiceError::OperationFailed
            })?
            .upload_file(&file_path, &file_name)
            .await
            .map_err(|err| {
                log::error!("error creating client due to {}", err);
                ServiceError::OperationFailed
            })?;

        let request = CreateAudioBookRequest {
            file_name: file_name.to_owned(),
            src: imagekit_upload_response.url,
            playlist_identifier,
        };

        let book_identifier = self
            .audio_book_repository
            .create(&request, &claim.user_identifier)
            .await?;

        Ok(book_identifier)
        // });
        // todo!()
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
        let data = self
            .audio_book_repository
            .fetch_favourites(user_identifier, pagination_params)
            .await?;

        let response = PaginatedResponse {
            data,
            page: pagination_params.page.unwrap_or_default(),
            per_page: pagination_params.per_page.unwrap_or_default(),
            total_count: 5,
            total_pages: 5, //TODO: use complex query to read all at onece
        };

        Ok(response)
    }
}
