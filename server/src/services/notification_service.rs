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
}

pub trait NotifiactionServiceExt {
    fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl NotifiactionServiceExt for NotifiactionService {
    async fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> Result<(), ServiceError> {
        self.repository.create(request).await?;

        Ok(())
    }
}
