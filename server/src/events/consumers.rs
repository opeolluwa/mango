use crate::errors::app_error::AppError;
use crate::events::channels::RedisMessageChannel;
use crate::shared::extract_env::extract_env;
use futures_util::StreamExt;

#[derive(Debug, Default)]
pub struct RedisWorker {
    channel: RedisMessageChannel,
    message: String,
}

// Private helper functions for processing each channel type
impl RedisWorker {
    /// Process file uploaded events
    async fn process_file_uploaded(message: &str) -> Result<(), AppError> {
        log::info!("Processing file uploaded event: {}", message);
        // TODO: Implement file upload processing logic
        // - Parse the uploaded file information
        // - Validate file format and size
        // - Store file metadata in database
        // - Trigger conversion pipeline if needed
        Ok(())
    }

    /// Process file converted events
    async fn process_file_converted(message: &str) -> Result<(), AppError> {
        log::info!("Processing file converted event: {}", message);
        // TODO: Implement file conversion processing logic
        // - Parse conversion results
        // - Update file status in database
        // - Notify users of completion
        // - Clean up temporary files
        Ok(())
    }

    /// Process MP3 converted events
    async fn process_mp3_converted(message: &str) -> Result<(), AppError> {
        log::info!("Processing MP3 converted event: {}", message);
        // TODO: Implement MP3 conversion processing logic
        // - Parse MP3 conversion results
        // - Update audio book status
        // - Generate audio metadata
        // - Trigger next pipeline step
        Ok(())
    }

    /// Process email events
    async fn process_email(message: &str) -> Result<(), AppError> {
        log::info!("Processing email event: {}", message);
        // TODO: Implement email processing logic
        // - Parse email content
        // - Send confirmation emails
        // - Handle email templates
        // - Track email delivery status
        Ok(())
    }

    /// Process default/unknown channel events
    async fn process_default(message: &str) -> Result<(), AppError> {
        log::warn!("Processing unknown channel event: {}", message);
        // TODO: Implement default processing logic
        // - Log unknown channel events
        // - Handle fallback processing
        // - Report unexpected messages
        Ok(())
    }
}

pub trait RedisWorkerExt {
    fn consume_message(
        channel: &RedisMessageChannel,
        message_str: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
    fn start_redis_listener() -> impl std::future::Future<Output = Result<(), AppError>>;
}

// Implementation of RedisWorkerExt for RedisWorker
impl RedisWorkerExt for RedisWorker {
    async fn consume_message(
        channel: &RedisMessageChannel,
        message_str: &str,
    ) -> Result<(), AppError> {
        match channel {
            RedisMessageChannel::FileUploaded => Self::process_file_uploaded(message_str).await,
            RedisMessageChannel::FileConverted => Self::process_file_converted(message_str).await,
            RedisMessageChannel::Mp3Converted => Self::process_mp3_converted(message_str).await,
            RedisMessageChannel::Email => Self::process_email(message_str).await,
            RedisMessageChannel::Default => Self::process_default(message_str).await,
        }
    }

    async fn start_redis_listener() -> Result<(), AppError> {
        let redis_connection_url = extract_env::<String>("REDIS_CONNECTION_URL")?;

        let redis_client = redis::Client::open(redis_connection_url)
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        let mut pubsub = redis_client
            .get_async_pubsub()
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        pubsub
            .subscribe(&[
                RedisMessageChannel::Mp3Converted.to_string(),
                RedisMessageChannel::FileUploaded.to_string(),
                RedisMessageChannel::Email.to_string(),
                RedisMessageChannel::FileConverted.to_string(),
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

            let message_channel = RedisMessageChannel::from(channel.clone());
            Self::consume_message(&message_channel, &message).await?;
        }

        Ok(())
    }
}
