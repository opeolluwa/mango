use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::Response,
};

use crate::{
    adapters::{api_response::ApiResponse, jwt::Claims},
    errors::service_error::ServiceError,
    services::notification_service::NotifiactionService,
    services::notification_service::NotifiactionServiceExt,
};

pub async fn listen_for_new_notifications(
    // State(notification_service): State<NotifiactionService>,
    // _claims: Claims,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }

        for i in 1..5 {
            if socket
                .send(Message::Text(format!("Hi {i} times!").into()))
                .await
                .is_err()
            {
                println!("client abruptly disconnected");
                return;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}
