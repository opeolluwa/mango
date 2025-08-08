use crate::errors::app_error::AppError;
use crate::errors::service_error::ServiceError;
use crate::events::channels::RedisMessageChannel;
use crate::events::message::{ConvertDocumentMessage, ConvertPcmMessage, RedisMessage};
use crate::shared::extract_env::extract_env;
use aers_audify::Audify;
use aers_wav_mp3_converter::WavToMp3Converter;
use futures_util::StreamExt;

#[derive(Debug, Default)]
pub struct RedisWorker {
    channel: RedisMessageChannel,
    message: String,
}

// Private helper functions for processing each channel type
impl RedisWorker {
    /// Process file uploaded events
    async fn convert_document_to_audio(
        message: &RedisMessage<ConvertDocumentMessage>,
    ) -> Result<(), ServiceError> {
        let config_path = "resources/models/en_US-libritts_r-medium.onnx.json";
        let audify_client = Audify::new(config_path);

        audify_client
            .synthesize_pdf(
                &message.payload.document_path,
                &message.payload.wav_output_path,
            )
            .map_err(|err| {
                log::error!("Audify synthesis failed: {}", err);
                ServiceError::OperationFailed
            })?;
        Ok(())
    }



    /// Process MP3 converted events
    async fn convert_wav_to_mp3(message: &RedisMessage<ConvertPcmMessage>) -> Result<(), ServiceError> {
        let mp3_export = WavToMp3Converter::new()
            .convert_file(&message.payload.wav_input_file)
            .map_err(|err| {
                log::error!("WAV to MP3 conversion failed: {}", err);
                ServiceError::OperationFailed
            })?;
        Ok(())
    }

    /// Process email events
    async fn upload_mp3_audio_to_cloud(message: &str) -> Result<(), AppError> {
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
        // match channel {
        //     RedisMessageChannel::FileUploaded => Self::process_file_uploaded(message_str).await,
        //     RedisMessageChannel::FileConverted => Self::process_file_converted(message_str).await,
        //     RedisMessageChannel::Mp3Converted => Self::process_mp3_converted(message_str).await,
        //     RedisMessageChannel::Email => Self::process_email(message_str).await,
        //     RedisMessageChannel::Default => Self::process_default(message_str).await,
        //     RedisMessageChannel::ConvertDocument => todo!(),
        //     RedisMessageChannel::DocumentConverted => todo!(),
        // }

        Ok(())
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
