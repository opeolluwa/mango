use axum::extract::State;

use crate::adapters::api_response::ApiResponse;
use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, AddBookToPlaylistResponse, CreateAudioBookRequest,
    CreateAudioBookResponse, DeleteBookRequest, DeleteBookResponse, FetchBookRequest,
    FetchBookResponse, MarkFavouriteRequest, MarkFavouriteResponse, RemoveBookFromPlaylistRequest,
    RemoveBookFromPlaylistResponse, UpdateBookRequest, UpdateBookResponse,
};
use crate::adapters::jwt::Claims;
use crate::errors::common_service_error::ServiceError;
use crate::middlewares::validator::ValidatedRequest;
use crate::services::audio_book_service::{AudioBooksService, AudioBooksServiceExt};

pub async fn fetch_book(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<FetchBookRequest>,
) -> Result<ApiResponse<FetchBookResponse>, ServiceError> {
    todo!()
}

pub async fn create_new_book(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<CreateAudioBookRequest>,
) -> Result<ApiResponse<CreateAudioBookResponse>, ServiceError> {
    audio_book_service
        .create_new(&request, &claim.identifier)
        .await?;
    todo!()
}

pub async fn add_to_playlist(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<AddBookToPlaylistRequest>,
) -> Result<ApiResponse<AddBookToPlaylistResponse>, ServiceError> {
    todo!()
}

pub async fn remove_from_playlist(
    State(audio_book_service): State<AudioBooksService>,
    claim: Claims,
    ValidatedRequest(request): ValidatedRequest<RemoveBookFromPlaylistRequest>,
) -> Result<ApiResponse<RemoveBookFromPlaylistResponse>, ServiceError> {
    todo!()
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
