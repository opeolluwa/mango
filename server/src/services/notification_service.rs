use axum::{
    extract::ws::{Message, WebSocket},
    response::Response,
};
use sqlx::{Pool, Postgres};

use crate::{
    adapters::notification::CreateNotification,
    entities::notifications::Notification,
    errors::service_error::ServiceError,
    repositories::notification_repository::{NotificationRepository, NotificationRepositoryExt},
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
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
    pub async fn handle_web_socket_connection(&self, mut socket: WebSocket) {
        let (mut sender, mut receiver) = socket.split();

        //         let msg = Message::text(
        //             r#"
        //         {
        //   "playlist_identifier": null,
        //   "file_name": "nccf-letter-draft-2.docx",
        //   "user_identifier": "a2d4eb25-5b46-44ed-8bb7-281e568be7a5",
        //   "file_path": "/tmp/1755344187_nccf-letter-draft-2.docx.pdf"
        // }"#,
        //         );
        //         if socket.send(msg).await.is_err() {
        //             // client disconnected
        //             return;
        //         }

        tokio::spawn(Self::read_message_from_worker(receiver));
        tokio::spawn(Self::send_mesage_to_client(sender));
    }
    pub async fn read_message_from_worker(receiver: SplitStream<WebSocket>) {}
    pub async fn send_mesage_to_client(sender: SplitSink<WebSocket, Message>) {}

    //push message to client
    pub async fn notify() {}
}

pub trait NotificationServiceExt {
    fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn listen_for_new_notifications(&self) -> impl std::future::Future<Output = Response> + Send;

    fn get_latest_unred_notifications()
    -> impl std::future::Future<Output = Result<Vec<Notification>, ServiceError>> + Send;
}

impl NotificationServiceExt for NotifiactionService {
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

    async fn get_latest_unred_notifications() -> Result<Vec<Notification>, ServiceError> {
        todo!()
    }
}
