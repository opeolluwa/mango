use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use aers_audify::Audify;
use aers_imagekit_client::ImagekitClient;
use aers_wav_mp3_converter::WavToMp3Converter;
use sqlx::{Pool, Postgres};

use crate::{
    adapters::notification::CreateNotification,
    errors::service_error::ServiceError,
    events::{
        channels::EventChannel,
        message::{ConvertDocument, DocumentConverted},
        producer::EventPrducer,
        websocket::send_websocket_msg,
    },
    services::{
        audio_book_service::{AudioBooksService, AudioBooksServiceExt},
        notification_service::{NotifiactionService, NotificationServiceExt},
    },
    shared::extract_env::extract_env,
};

pub struct EventWorker {
    notification_service: NotifiactionService,
    audio_book_service: AudioBooksService,
}

impl EventWorker {
    const CONFIG_PATH: &'static str = "resources/models/en_US-libritts_r-medium.onnx.json";

    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            notification_service: NotifiactionService::init(&pool),
            audio_book_service: AudioBooksService::init(&pool),
        }
    }

    /// Returns `(pdf_path, wav_path)` as strings.
    fn build_paths(&self, file_path: &Path) -> Result<(String, String), ServiceError> {
        let pdf_path = file_path
            .to_str()
            .ok_or(ServiceError::OperationFailed)?
            .to_string();

        let wav_path = file_path
            .with_extension("wav")
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
        Audify::new(config_path)
            .synthesize_pdf(pdf_path, wav_path)
            .map_err(|err| {
                log::error!("Audify synthesis failed: {err}");
                ServiceError::OperationFailed
            })
    }

    fn ensure_file_exists(&self, path_str: &str) -> Result<(), ServiceError> {
        if !Path::new(path_str).exists() {
            log::error!("File not found: {}", path_str);
            return Err(ServiceError::OperationFailed);
        }
        Ok(())
    }

    fn convert_wav_to_mp3(&self, wav_path: &str) -> Result<PathBuf, ServiceError> {
        WavToMp3Converter::new()
            .convert_file(wav_path)
            .map_err(|err| {
                log::error!("WAV to MP3 conversion failed: {err}");
                ServiceError::OperationFailed
            })
    }

    fn imagekit_client(&self) -> Result<ImagekitClient, ServiceError> {
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
        mp3_path: &Path,
        file_name: &str,
    ) -> Result<String, ServiceError> {
        client
            .upload_file(mp3_path, file_name)
            .await
            .map_err(|err| {
                log::error!("MP3 upload failed: {}", err);
                ServiceError::OperationFailed
            })
            .map(|res| res.url)
    }

    fn publish_document_converted(&self, payload: DocumentConverted) {
        tokio::task::spawn(async move {
            let channel = EventChannel::DocumentConvertedToAudio;
            if let Err(err) = EventPrducer::new(&channel, payload).send().await {
                log::error!("Failed to publish message to {channel}: {err}");
            }
        });
    }
}

pub trait EventWorkerExt {
    fn convert_document_to_audio(
        &self,
        message: &ConvertDocument,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn log_message(&self, message: &str);

    fn process_document_converted(
        &self,
        message: &DocumentConverted,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl EventWorkerExt for EventWorker {
    async fn convert_document_to_audio(
        &self,
        message: &ConvertDocument,
    ) -> Result<(), ServiceError> {
        log::info!("conversion started");
        let ConvertDocument {
            playlist_identifier,
            file_name,
            user_identifier,
            file_path,
        } = &message;

        if !file_path.exists() {
            log::error!("File {:?} does not exist", file_path);
            return Err(ServiceError::OperationFailed);
        }

        let (pdf_path, wav_path) = self.build_paths(file_path)?;
        log::info!("generating wav file");
        self.synthesize_pdf_to_wav(Self::CONFIG_PATH, &pdf_path, &wav_path)?;
        self.ensure_file_exists(&wav_path)?;

        log::info!("generating mp3");
        let mp3_path = self.convert_wav_to_mp3(&wav_path)?;
        let imagekit_client = self.imagekit_client()?;
        let url = self
            .upload_mp3(&imagekit_client, &mp3_path, file_name)
            .await?;

        self.publish_document_converted(DocumentConverted {
            playlist_identifier: *playlist_identifier,
            file_name: file_name.clone(),
            user_identifier: *user_identifier,
            url,
        });

        Ok(())
    }

    fn log_message(&self, message: &str) {
        log::debug!("Got message: {message}");
    }

    async fn process_document_converted(
        &self,
        message: &DocumentConverted,
    ) -> Result<(), ServiceError> {
        let converted_file = message.clone();

        let audio_service = self.audio_book_service.clone();
        tokio::task::spawn(async move {
            if let Err(err) = audio_service.processs_file_converted(&converted_file).await {
                log::error!("Failed to process converted file: {err}");
            }
        });

        let notification_service = self.notification_service.clone();
        let request = CreateNotification {
            user_identifier: message.user_identifier,
            subject: "File converted".to_string(),
            description: format!("Your file has been converted: {}", message.url),
        };

        tokio::task::spawn(async move {
            let Some(identifier) = notification_service
                .create_new_notification(&request)
                .await
                .ok()
            else {
                log::error!("Failed to send notification");
                return;
            };

            let notification = notification_service.fetch_one(&identifier).await.unwrap();

            send_websocket_msg("ws://localhost:5006/notifications/listen", notification);
        });

        Ok(())
    }
}
