use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    adapters::{
        api_request::AuthenticatedRequest,
        api_response::{ApiResponse, ApiResponseBuilder},
        jwt::Claims,
        playlist::CreatePlaylistRequest,
    },
    entities::playlist::Playlist,
    errors::service_error::ServiceError,
    services::playlist_service::{PlaylistService, PlaylistServiceExt},
};

pub async fn create_playlist(
    State(playlist_service): State<PlaylistService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<CreatePlaylistRequest>,
) -> Result<ApiResponse<Playlist>, ServiceError> {
    let playlist_identifier = playlist_service
        .create_new_playlist(&data, &claims.user_identifier)
        .await?;

    let playlist = playlist_service
        .fetch_one(&playlist_identifier, &claims.user_identifier)
        .await?
        .ok_or(ServiceError::OperationFailed)?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("playlist created successfully")
        .data(playlist)
        .build())
}

pub async fn retrieve_playlist(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Path(playlist_identifier): Path<Uuid>,
) -> Result<ApiResponse<Playlist>, ServiceError> {
    let playlist = playlist_service
        .fetch_one(&playlist_identifier, &claims.user_identifier)
        .await?
        .ok_or(ServiceError::OperationFailed)?;

    Ok(ApiResponseBuilder::new()
        .message("playlist fetched successfully")
        .data(playlist)
        .build())
}

pub async fn delete_playlist(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Path(playlist_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    playlist_service
        .delete_playlist(&playlist_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("playlist fetched successfully")
        .build())
}

pub async fn update_playlist(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Path(playlist_identifier): Path<Uuid>,
) -> Result<ApiResponse<Playlist>, ServiceError> {
    playlist_service
        .update_playlist(&playlist_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("playlist updated successfully")
        .build())
}
