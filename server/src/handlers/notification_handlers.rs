use axum::extract::State;

use crate::{
    adapters::{api_response::ApiResponse, jwt::Claims},
    errors::service_error::ServiceError,
    services::notification_service::NotifiactionService,
};

pub async fn listen_for_new_notifications(
    State(_notification_service): State<NotifiactionService>,
    _claims: Claims,
) -> Result<ApiResponse<()>, ServiceError> {
    Ok(ApiResponse::builder()
        .data(())
        .message("listening for new notification")
        .build())
}
