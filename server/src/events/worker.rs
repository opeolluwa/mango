use std::path::Path;

use aers_audify::Audify;
use aers_imagekit_client::ImagekitClient;
use aers_wav_mp3_converter::WavToMp3Converter;

use crate::{
    errors::service_error::ServiceError,
    events::{
        channels::EventChannel,
        message::{ConvertDocument, DocumentConverted, Event},
        producer::EventPrducer,
    },
    shared::extract_env::extract_env,
};

pub struct EventWorker {}

impl Default for EventWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl EventWorker {
    pub fn new() -> Self {
        Self {}
    }
}
pub trait EventWorkerExt {
    fn convert_document_to_audio(
        &self,
        message: &Event<ConvertDocument>,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn log_message(&self, message: &str);
}

impl EventWorkerExt for EventWorker {
    async fn convert_document_to_audio(
        &self,
        message: &Event<ConvertDocument>,
    ) -> Result<(), ServiceError> {
        let ConvertDocument {
            playlist_identifier,
            file_name,
            user_identifier,
            file_path,
        } = &message.payload;

        let config_path = "resources/models/en_US-libritts_r-medium.onnx.json";
        let audify_client = Audify::new(config_path);

        if !message.payload.file_path.exists() {
            log::error!("file {:?} does not exist", file_path);
        }

        let wav_path_buf = file_path.with_extension("wav");
        let wav_path = wav_path_buf.to_str().unwrap();
        let pdf_path = message.payload.file_path.to_str().unwrap();

        audify_client
            .synthesize_pdf(pdf_path, wav_path)
            .map_err(|err| {
                log::error!("Audify synthesis failed: {err}");
                ServiceError::OperationFailed
            })?;

        if !Path::new(&wav_path).exists() {
            log::error!("WAV file does not exist: {}", wav_path);
            return Err(ServiceError::OperationFailed);
        } else {
            log::info!("WAV file generated: {}", wav_path);
        }

        let mp3_export = WavToMp3Converter::new()
            .convert_file(&wav_path)
            .map_err(|err| {
                log::error!("WAV to MP3 conversion failed: {err}");
                ServiceError::OperationFailed
            })?;

        let private_key = extract_env::<String>("IMAGEKIT_PRIVATE_KEY").unwrap();
        let public_key = extract_env::<String>("IMAGEKIT_PUBLIC_KEY").unwrap();

        let imagekit_client = ImagekitClient::new(&public_key, &private_key).map_err(|err| {
            log::error!("ImageKit client creation failed: {}", err);
            ServiceError::OperationFailed
        })?;

        let upload_response = imagekit_client
            .upload_file(&mp3_export, &file_name)
            .await
            .map_err(|err| {
                log::error!("MP3 upload failed: {}", err);
                ServiceError::OperationFailed
            })?;

        let file_converted_event = DocumentConverted {
            playlist_identifier: *playlist_identifier,
            file_name: file_name.to_string(),
            user_identifier: *user_identifier,
            url: upload_response.url,
        };

        tokio::task::spawn(async move {
            let channel = EventChannel::DocumentConvertedToAudio;
            let message = file_converted_event;
            if let Err(err) = EventPrducer::new(&channel, message).send().await {
                log::error!("failed to publish message to {channel} due to {err}");
            }
        });
        Ok(())
    }

    fn log_message(&self, message: &str) {
        log::debug!("got message {message}");
    }
}
