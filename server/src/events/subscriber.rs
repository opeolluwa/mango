use std::fmt::Debug;
use std::sync::Arc;

use crate::errors::app_error::AppError;
use crate::errors::service_error::ServiceError;
use crate::events::channels::EventChannel;

use crate::events::message::{ConvertDocument, DocumentConverted, Event};
use crate::events::worker::{EventWorker, EventWorkerExt};
use crate::shared::extract_env::extract_env;
use futures_util::StreamExt;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct EventSubscriber {}

impl EventSubscriber {
    pub fn new<T>(channel: &str, message: &str) -> Result<(EventChannel, Event<T>), ServiceError>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let channel = EventChannel::from(channel.to_string());
        let message: T = serde_json::from_str(message).map_err(|err| {
            log::error!("failed to extract event data from {channel} due to {err}");
            ServiceError::OperationFailed
        })?;

        Ok((channel, Event::new(message)))
    }
}
pub trait EventSubscriberExt {
    fn consume_message(
        channel: &str,
        message: &str,
        pool: Arc<Pool<Postgres>>,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
    fn start_redis_listener(
        pool: Arc<Pool<Postgres>>,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
    fn parse_message<T: Debug + Serialize + DeserializeOwned>(message: &str)
    -> Result<T, AppError>;
}

// Implementation ofEventSubscriberExt forEventSubscriber
impl EventSubscriberExt for EventSubscriber {
    fn parse_message<T>(message: &str) -> Result<T, AppError>
    where
        T: Debug + Serialize + DeserializeOwned,
    {
        let message: T = serde_json::from_str(message).map_err(|err| {
            log::error!("failed to extract event data from due to: {err}");
            AppError::OperationFailed(err.to_string())
        })?;

        Ok(message)
    }
    async fn consume_message(
        channel: &str,
        message: &str,
        pool: Arc<Pool<Postgres>>,
    ) -> Result<(), AppError> {
        let channel = EventChannel::from(channel.to_string());
        let worker = EventWorker::new(pool);

        match channel {
            EventChannel::DocumentConvertedToAudio => {
                let message = Self::parse_message::<DocumentConverted>(message)?;

                if let Err(err) = worker.process_document_converted(&message).await {
                    log::error!("failed to process event due to {err}");
                }
            }

            EventChannel::ConvertDocumentToAudio => {
                let message = Self::parse_message::<ConvertDocument>(message)?;
                if let Err(err) = worker.convert_document_to_audio(&message).await {
                    log::error!("failed to process event due to err {err}");
                };
            }

            _ => {
                log::warn!(
                    "worker for channel {channel} not found, discarding message {message:#?}"
                )
            }
        }

        Ok(())
    }

    async fn start_redis_listener(pool: Arc<Pool<Postgres>>) -> Result<(), AppError> {
        let redis_connection_url = extract_env::<String>("REDIS_CONNECTION_URL")?;

        let redis_client = redis::Client::open(redis_connection_url)
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        let mut pubsub = redis_client
            .get_async_pubsub()
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        pubsub
            .subscribe(&[
                EventChannel::ConvertDocumentToAudio.to_string(),
                EventChannel::DocumentConvertedToAudio.to_string(),
                EventChannel::Default.to_string(),
            ])
            .await
            .map_err(|err| AppError::OperationFailed(err.to_string()))?;

        let mut stream = pubsub.on_message();
        log::info!("Redis pubsub listener started");

        while let Some(msg) = stream.next().await {
            let channel = msg.get_channel_name().to_string();
            let message: String = msg
                .get_payload()
                .unwrap_or_else(|_| "<Invalid Payload>".to_string());

            Self::consume_message(&channel, &message, pool.clone()).await?;
        }

        Ok(())
    }
}
