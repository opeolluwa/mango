use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};

use crate::{
    adapters::{api_response::ApiResponse, jwt::Claims, pagination::{PaginatedResponse, PaginationParams}},
    entities::notifications::Notification,
    errors::service_error::ServiceError,
    services::notification_service::{NotifiactionService, NotificationServiceExt},
};

pub async fn listen_for_new_notifications(
    State(notification_service): State<NotifiactionService>,
    // _claims: Claims,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| {
        let service = notification_service.clone();
        async move { service.handle_web_socket_connection(socket).await }
    })
}

pub async fn fetch_notification(
    State(notification_service): State<NotifiactionService>,
    claims: Claims,
    pagination: &PaginationParams,
) -> Result<ApiResponse<PaginatedResponse<Vec<Notification>>>, ServiceError> {
    let notifications = notification_service
        .fetch_notifications(&claims, pagination)
        .await?;

    Ok(ApiResponse::builder()
        .data(notifications)
        .message("fetch notifications")
        .build())
}
