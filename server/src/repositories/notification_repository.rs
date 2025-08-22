use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    adapters::{notification::CreateNotification, pagination::PaginationParams},
    entities::notifications::Notification,
    errors::repository_error::RepositoryError,
};

#[derive(Clone)]
pub struct NotificationRepository {
    pool: Arc<Pool<Postgres>>,
}

impl NotificationRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}
pub trait NotificationRepositoryExt {
    fn create(
        &self,
        notification: &CreateNotification,
    ) -> impl std::future::Future<Output = Result<Uuid, RepositoryError>> + Send;

    fn mark_read(
        &self,
        user_identifier: &Uuid,
        notification_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn fetch_all(
        &self,
        user_identifier: &Uuid,
        pagination: PaginationParams,
    ) -> impl std::future::Future<Output = Result<Vec<Notification>, RepositoryError>> + Send;

    fn fetch_one(
        &self,
        notification_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Option<Notification>> + Send;
}

impl NotificationRepositoryExt for NotificationRepository {
    async fn create(&self, notification: &CreateNotification) -> Result<Uuid, RepositoryError> {
        let identifier = Uuid::new_v4();
        sqlx::query("INSERT INTO notifications (identifier, subject, body, user_identifier) VALUES($1, $2, $3, $4)")
        .bind(identifier)
        .bind(&notification.subject)
        .bind(&notification.description)
        .bind(notification.user_identifier)
        .execute(self.pool.as_ref())
        .await.map_err(RepositoryError::from)?;

        Ok(identifier)
    }

    async fn fetch_all(
        &self,
        user_identifier: &Uuid,
        pagination: PaginationParams,
    ) -> Result<Vec<Notification>, RepositoryError> {
        let limit = pagination.page.unwrap_or_default() - 1;
        let offset = pagination.per_page.unwrap_or_default();

        let notifications = sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications WHERE user_identifier = $1 LIMIT $2 OFFSET $3",
        )
        .bind(user_identifier)
        .bind(limit.to_string())
        .bind(offset.to_string())
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(RepositoryError::SqlxError)?;

        Ok(notifications)
    }

    async fn mark_read(
        &self,
        user_identifier: &Uuid,
        notification_identifier: &Uuid,
    ) -> Result<(), RepositoryError> {
        sqlx::query("UPDATE notifications WHERE user_identifier = $1  AND identifier = $2")
            .bind(user_identifier)
            .bind(notification_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::SqlxError)?;

        Ok(())
    }

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<Notification> {
        let result =
            sqlx::query_as::<_, Notification>("SELECT * FROM notifications WHERE identifier = $1")
                .bind(notification_identifier)
                .fetch_optional(self.pool.as_ref())
                .await
                .map_err(RepositoryError::SqlxError)
                .ok()
                .flatten();
        result
    }
}
