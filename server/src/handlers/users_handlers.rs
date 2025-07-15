use axum::extract::State;
use axum_typed_multipart::TypedMultipart;

use crate::{
    adapters::{
        api_request::AuthenticatedRequest,
        api_response::{ApiResponse, ApiResponseBuilder},
        audio_books::UpdateProfilePicture,
        authentication::SetNewPasswordRequest,
        jwt::Claims,
        users::{PartialUserProfile, UserDto},
    },
    errors::{service_error::ServiceError, user_service_error::UserServiceError},
    services::user_service::UserService,
};

use crate::services::user_service::UserServiceTrait;

pub async fn retrieve_information(
    State(user_service): State<UserService>,
    claims: Claims,
) -> Result<ApiResponse<UserDto>, UserServiceError> {
    let user_data = user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_password(
    State(user_service): State<UserService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<SetNewPasswordRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    user_service
        .update_password(&data, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_profile_picture(
    State(user_service): State<UserService>,
    claims: Claims,
    request: TypedMultipart<UpdateProfilePicture>,
) -> Result<ApiResponse<UserDto>, ServiceError> {
    user_service
        .update_profile_picture(request, &claims.user_identifier)
        .await?;

    let updated_profile = user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(updated_profile)
        .message("profile updated successfully")
        .build())
}

pub async fn update_profile(
    State(user_service): State<UserService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<PartialUserProfile>,
) -> Result<ApiResponse<UserDto>, ServiceError> {
    user_service
        .update_profile(&data, &claims.user_identifier)
        .await?;

    let updated_profile = user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(updated_profile)
        .message("profile updated successfully")
        .build())
}
