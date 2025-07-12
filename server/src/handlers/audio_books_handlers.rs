use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum_typed_multipart::TypedMultipart;
use uuid::Uuid;

use crate::adapters::api_response::{ApiResponse, ApiResponseBuilder};
use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, DeleteBookRequest, DeleteBookResponse, MarkFavouriteRequest,
    MarkFavouriteResponse, UpdateBookRequest, UpdateBookResponse, UploadAssetRequest,
};
use crate::adapters::jwt::Claims;
use crate::entities::audio_book::AudioBookEntity;
use crate::errors::service_error::ServiceError;
use crate::middlewares::validator::ValidatedRequest;
use crate::services::audio_book_service::{AudioBooksService, AudioBooksServiceExt};

pub async fn create_new_book(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    request: TypedMultipart<UploadAssetRequest>,
) -> Result<ApiResponse<AudioBookEntity>, ApiResponse<()>> {
    let book_identifier = audio_book_service.create_new(request, &claims).await?;
    let book = audio_book_service
        .fetch_one(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(book)
        .status_code(StatusCode::CREATED)
        .message("Book created succesfully")
        .build())
}

pub async fn fetch_book(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<AudioBookEntity>, ServiceError> {
    let book = audio_book_service
        .fetch_one(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(book)
        .status_code(StatusCode::OK)
        .message("Book created succesfully")
        .build())
}

pub async fn add_to_playlist(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<AddBookToPlaylistRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    audio_book_service
        .add_to_playlist(&request, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .status_code(StatusCode::OK)
        .message("Book successfully added to playlist")
        .build())
}

pub async fn remove_from_playlist(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    audio_book_service
        .remove_from_playlist(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .status_code(StatusCode::OK)
        .message("Book successfully removed to playlist")
        .build())
}

pub async fn update_book(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<UpdateBookRequest>,
) -> Result<ApiResponse<UpdateBookResponse>, ServiceError> {
    todo!()
}

pub async fn delete_book(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<DeleteBookRequest>,
) -> Result<ApiResponse<DeleteBookResponse>, ServiceError> {
    todo!()
}

pub async fn mark_favourite(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<MarkFavouriteRequest>,
) -> Result<ApiResponse<MarkFavouriteResponse>, ServiceError> {
    todo!()
}
