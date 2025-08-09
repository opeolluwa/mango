use aers_audify::Audify;
use aers_wav_mp3_converter::WavToMp3Converter;

use crate::{
    errors::service_error::ServiceError,
    events::message::{ConvertDocumentMessage, ConvertWavToMp3Message, Event},
};

pub struct EventWorker {}

impl EventWorker {
    pub fn new() -> Self {
        Self {}
    }
}
pub trait EventWorkerExt {
    fn convert_document_to_audio(
        &self,
        message: &Event<ConvertDocumentMessage>,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn convert_wav_to_mp3(
        &self,
        message: &Event<ConvertWavToMp3Message>,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn upload_mp3_audio_to_cloud(
        &self,
        message: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn log_message(&self, message: &str);
}

impl EventWorkerExt for EventWorker {
    async fn convert_document_to_audio(
        &self,
        message: &Event<ConvertDocumentMessage>,
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

    async fn convert_wav_to_mp3(
        &self,
        message: &Event<ConvertWavToMp3Message>,
    ) -> Result<(), ServiceError> {
        let mp3_export = WavToMp3Converter::new()
            .convert_file(&message.payload.wav_input_file)
            .map_err(|err| {
                log::error!("WAV to MP3 conversion failed: {}", err);
                ServiceError::OperationFailed
            })?;
        Ok(())
    }

    /// Process email events
    async fn upload_mp3_audio_to_cloud(&self, message: &str) -> Result<(), ServiceError> {
        log::info!("Processing email event: {}", message);
        // TODO: Implement email processing logic
        // - Parse email content
        // - Send confirmation emails
        // - Handle email templates
        // - Track email delivery status
        Ok(())
    }

    fn log_message(&self, message: &str) {
        log::debug!("got message {}", message);
    }
}
