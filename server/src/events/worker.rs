use std::path::{Path, PathBuf};

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

    fn build_paths(&self, file_path: &Path) -> Result<(String, String), ServiceError> {
        let wav_path_buf = file_path.with_extension("wav");
        let wav_path = wav_path_buf
            .to_str()
            .ok_or(ServiceError::OperationFailed)?
            .to_string();
        let pdf_path = file_path
            .to_str()
            .ok_or(ServiceError::OperationFailed)?
            .to_string();
        Ok((pdf_path, wav_path))
    }

    fn synthesize_pdf_to_wav(
        &self,
        config_path: &str,
        pdf_path: &str,
        wav_path: &str,
    ) -> Result<(), ServiceError> {
        let audify_client = Audify::new(config_path);
        audify_client
            .synthesize_pdf(pdf_path, wav_path)
            .map_err(|err| {
                log::error!("Audify synthesis failed: {err}");
                ServiceError::OperationFailed
            })?;
        Ok(())
    }

    fn ensure_file_exists(&self, path_str: &str) -> Result<(), ServiceError> {
        if !Path::new(path_str).exists() {
            log::error!("WAV file does not exist: {}", path_str);
            return Err(ServiceError::OperationFailed);
        }
        log::info!("WAV file generated: {}", path_str);
        Ok(())
    }

    fn convert_wav_to_mp3_path(&self, wav_path: &str) -> Result<PathBuf, ServiceError> {
        WavToMp3Converter::new()
            .convert_file(&wav_path)
            .map_err(|err| {
                log::error!("WAV to MP3 conversion failed: {err}");
                ServiceError::OperationFailed
            })
    }

    fn build_imagekit_client(&self) -> Result<ImagekitClient, ServiceError> {
        let private_key: String =
            extract_env("IMAGEKIT_PRIVATE_KEY").map_err(ServiceError::from)?;
        let public_key: String = extract_env("IMAGEKIT_PUBLIC_KEY").map_err(ServiceError::from)?;
        ImagekitClient::new(&public_key, &private_key).map_err(|err| {
            log::error!("ImageKit client creation failed: {}", err);
            ServiceError::OperationFailed
        })
    }

    async fn upload_mp3(
        &self,
        client: &ImagekitClient,
        mp3_export: &PathBuf,
        file_name: &str,
    ) -> Result<String, ServiceError> {
        let upload_response = client
            .upload_file(mp3_export, file_name)
            .await
            .map_err(|err| {
                log::error!("MP3 upload failed: {}", err);
                ServiceError::OperationFailed
            })?;
        Ok(upload_response.url)
    }

    fn publish_document_converted(&self, payload: DocumentConverted) {
        tokio::task::spawn(async move {
            let channel = EventChannel::DocumentConvertedToAudio;
            if let Err(err) = EventPrducer::new(&channel, payload).send().await {
                log::error!("failed to publish message to {channel} due to {err}");
            }
        });
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

        if !file_path.exists() {
            log::error!("file {:?} does not exist", file_path);
        }

        let (pdf_path, wav_path) = self.build_paths(file_path)?;
        let config_path = "resources/models/en_US-libritts_r-medium.onnx.json";

        self.synthesize_pdf_to_wav(config_path, &pdf_path, &wav_path)?;
        self.ensure_file_exists(&wav_path)?;

        let mp3_export = self.convert_wav_to_mp3_path(&wav_path)?;
        let imagekit_client = self.build_imagekit_client()?;
        let url = self
            .upload_mp3(&imagekit_client, &mp3_export, file_name)
            .await?;

        let payload = DocumentConverted {
            playlist_identifier: *playlist_identifier,
            file_name: file_name.to_string(),
            user_identifier: *user_identifier,
            url,
        };

        self.publish_document_converted(payload);
        Ok(())
    }

    fn log_message(&self, message: &str) {
        log::debug!("got message {message}");
    }
}
