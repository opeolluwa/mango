use axum::{
    extract::ws::{Message, WebSocket},
    response::Response,
};
use sqlx::{Pool, Postgres};

use crate::{
    adapters::notification::CreateNotification, errors::service_error::ServiceError,
    repositories::notification_repository::NotificationRepository,
    repositories::notification_repository::NotificationRepositoryExt,
};

#[derive(Clone)]
pub struct NotifiactionService {
    repository: NotificationRepository,
}

impl NotifiactionService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            repository: NotificationRepository::init(pool),
        }
    }
    async fn handle_web_socket_connection(&self, mut socket: WebSocket) {
        // while let Some(msg) = socket.recv().await {
        //     let msg = if let Ok(msg) = msg {
        //         msg
        //     } else {
        //         // client disconnected
        //         return;
        //     };

        let msg = Message::text(
            r#"
        {
  "playlist_identifier": null,
  "file_name": "nccf-letter-draft-2.docx",
  "user_identifier": "a2d4eb25-5b46-44ed-8bb7-281e568be7a5",
  "file_path": "/tmp/1755344187_nccf-letter-draft-2.docx.pdf"
}"#,
        );
        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

pub trait NotifiactionServiceExt {
    fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn listen_for_new_notifications(&self) -> impl std::future::Future<Output = Response> + Send;
}

impl NotifiactionServiceExt for NotifiactionService {
    async fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> Result<(), ServiceError> {
        self.repository.create(request).await?;

        Ok(())
    }
    async fn listen_for_new_notifications(&self) -> Response {
        todo!()
        // self.handle_web_socket_connection(socket).await
    }
}
