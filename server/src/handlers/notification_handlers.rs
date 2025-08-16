use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};

use crate::{
    adapters::{api_response::ApiResponse, jwt::Claims},
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
    _claims: Claims,
) -> Result<ApiResponse<Vec<Notification>>, ServiceError> {
    todo!()
}
