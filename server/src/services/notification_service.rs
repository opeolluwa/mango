use axum::{
    extract::ws::{Message, WebSocket},
    response::Response,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    adapters::{
        jwt::Claims,
        notification::CreateNotification,
        pagination::{PaginatedResponse, PaginationParams},
    },
    entities::{common::RowCount, notifications::Notification},
    errors::service_error::ServiceError,
    repositories::notification_repository::{NotificationRepository, NotificationRepositoryExt},
};
use futures_util::{sink::SinkExt, stream::StreamExt};

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
    pub async fn handle_web_socket_connection(&self, socket: WebSocket) {
        let (mut outgoing, mut incoming) = socket.split();

        // send messages to connected client
        tokio::spawn(async move {
            if let Err(err) = outgoing.send(Message::Text("sample essage".into())).await {
                log::error!("{err}");
            }
        });

        // get message from worker
        tokio::spawn(async move {
            while let Some(message) = incoming.next().await {
                // send the message
                log::info!("got incoming message {message:#?}");
            }
        });
    }

    //push message to client
    pub async fn notify() {}
}

pub trait NotificationServiceExt {
    fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> impl std::future::Future<Output = Result<Uuid, ServiceError>> + Send;

    fn listen_for_new_notifications(&self) -> impl std::future::Future<Output = Response> + Send;

    fn get_latest_unread_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> impl std::future::Future<Output = Result<Vec<Notification>, ServiceError>> + Send;

    fn fetch_one(
        &self,
        notification_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Option<Notification>> + Send;

    fn fetch_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> impl std::future::Future<
        Output = Result<PaginatedResponse<Vec<Notification>>, ServiceError>,
    > + Send;

    fn count_unread(
        &self,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<RowCount, ServiceError>> + Send;

    fn mark_read(
        &self,
        claims: &Claims,
        notification_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl NotificationServiceExt for NotifiactionService {
    async fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> Result<Uuid, ServiceError> {
        let record_id = self.repository.create(request).await?;
        Ok(record_id)
    }
    async fn listen_for_new_notifications(&self) -> Response {
        todo!()
        // self.handle_web_socket_connection(socket).await
    }

    async fn get_latest_unread_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> Result<Vec<Notification>, ServiceError> {
        todo!()
    }

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<Notification> {
        self.repository.fetch_one(notification_identifier).await
    }

    async fn fetch_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<Vec<Notification>>, ServiceError> {
        let records = self
            .repository
            .fetch_all(&claims.user_identifier, &pagination)
            .await?;

        let paginated_result: PaginatedResponse<Vec<Notification>> = PaginatedResponse {
            records: records.notifications,
            page: pagination.page(),
            per_page: pagination.per_page(),
            total_count: records.total as u64,
            total_pages: records.total as u32 / pagination.per_page(),
        };
        Ok(paginated_result)
    }

    async fn count_unread(&self, claims: &Claims) -> Result<RowCount, ServiceError> {
        let result = self
            .repository
            .count_unread(&claims.user_identifier)
            .await?;

        Ok(result)
    }

    async fn mark_read(
        &self,
        claims: &Claims,
        notification_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.repository
            .mark_read(&claims.user_identifier, notification_identifier)
            .await?;
        Ok(())
    }
}
