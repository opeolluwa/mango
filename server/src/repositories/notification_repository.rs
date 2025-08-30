use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    adapters::{notification::CreateNotification, pagination::PaginationParams},
    entities::{
        common::RowCount,
        notifications::{Notification, PaginatedNotification},
    },
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
        pagination: &PaginationParams,
    ) -> impl std::future::Future<Output = Result<PaginatedNotification, RepositoryError>> + Send;

    fn fetch_one(
        &self,
        notification_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Option<Notification>> + Send;

    fn count_unread(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<RowCount, RepositoryError>> + Send;

    fn get_latest_unread_notifications(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> impl std::future::Future<Output = Result<PaginatedNotification, RepositoryError>> + Send;
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
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, RepositoryError> {
        // let query = r#"
        //     SELECT
        //         JSONB_AGG(TO_JSONB(u)) AS notifications,
        //         COUNT(identifier) AS total
        //     FROM (
        //         SELECT *
        //         FROM notifications
        //         WHERE user_identifier = $1
        //         ORDER BY created_at DESC
        //         LIMIT $2
        //         OFFSET $3
        //     ) u
        // "#;

        let query = r#"
    
            SELECT *
            FROM notifications
            WHERE user_identifier = $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
     
    "#;

        let page = pagination.page.unwrap_or(1);
        let per_page = pagination.per_page.unwrap_or(10);

        let offset = (page - 1) * per_page;
        let limit = per_page;

        let notifications = sqlx::query_as::<_, Notification>(query)
            .bind(user_identifier)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(RepositoryError::SqlxError)?;

        let RowCount { count: total } = sqlx::query_as::<_, RowCount>(
            "SELECT COUNT(identifier) FROM notifications WHERE user_identifier = $1",
        )
        .bind(user_identifier)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(PaginatedNotification {
            notifications,
            total,
        })
    }

    async fn mark_read(
        &self,
        user_identifier: &Uuid,
        notification_identifier: &Uuid,
    ) -> Result<(), RepositoryError> {
        sqlx::query("UPDATE notifications SET is_read = true  WHERE user_identifier = $1  AND identifier = $2")
            .bind(user_identifier)
            .bind(notification_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::SqlxError)?;

        Ok(())
    }

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<Notification> {
        sqlx::query_as::<_, Notification>("SELECT * FROM notifications WHERE identifier = $1")
            .bind(notification_identifier)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(RepositoryError::SqlxError)
            .ok()
            .flatten()
    }

    async fn count_unread(&self, user_identifier: &Uuid) -> Result<RowCount, RepositoryError> {
        let row_count = sqlx::query_as::<_, RowCount>(
            "SELECT COUNT(identifier) FROM notifications WHERE user_identifier = $1 AND is_read = false",
        )
        .bind(user_identifier)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(row_count)
    }

    async fn get_latest_unread_notifications(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, RepositoryError> {
        todo!()
    }
}
