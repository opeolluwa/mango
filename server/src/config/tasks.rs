use crate::{

    events::subscriber::{EventSubscriber, EventSubscriberExt},
};

pub struct AppBackgroundTasks {}

impl AppBackgroundTasks {
    pub fn init(db: std::sync::Arc<sqlx::PgPool>) {
        tokio::spawn(async move {
            if let Err(err) = EventSubscriber::start_redis_listener(db).await {
                tracing::error!("Redis listener failed: {err}");
            }
        });
    }
}
