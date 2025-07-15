use std::path::Path;

use aers_imagekit_client::ImagekitClient;
use aers_utils::generate_file_name;
use axum_typed_multipart::TypedMultipart;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::AERS_FILE_UPLOAD_PATH;
use crate::adapters::audio_books::UpdateProfilePicture;
use crate::adapters::authentication::SetNewPasswordRequest;
use crate::adapters::users::{PartialUserProfile, UserDto};
use crate::errors::service_error::ServiceError;
use crate::errors::user_service_error::UserServiceError;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};
use crate::services::helper_service::{ServiceHelpers, ServiceHelpersTrait};
use crate::shared::extract_env::extract_env;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
    user_helper_service: ServiceHelpers,
}

impl UserService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
            user_helper_service: ServiceHelpers::init(),
        }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<UserDto, UserServiceError>;

    async fn update_password(
        &self,

        request: &SetNewPasswordRequest,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError>;

    async fn update_profile_picture(
        &self,
        request: TypedMultipart<UpdateProfilePicture>,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError>;

    async fn update_profile(
        &self,
        request: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError>;
}

impl UserServiceTrait for UserService {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<UserDto, UserServiceError> {
        self.user_repository
            .retrieve_information(&user_identifier)
            .await
    }

    async fn update_password(
        &self,

        request: &SetNewPasswordRequest,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        self.user_repository
            .update_password(user_identifier, &password_hash)
            .await?;

        Ok(())
    }

    async fn update_profile_picture(
        &self,
        TypedMultipart(UpdateProfilePicture { image }): TypedMultipart<UpdateProfilePicture>,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        // tokio::task::spawn(async move {
        let file_name = image
            .metadata
            .file_name
            .clone()
            .unwrap_or(generate_file_name());

        let temp_dir = Path::new(AERS_FILE_UPLOAD_PATH);
        let file_path = temp_dir.join(format!(
            "{time_stamp}_{file_name}",
            time_stamp = chrono::Local::now().timestamp()
        ));

        // create file object
        if let Err(err) = image.contents.persist(&file_path) {
            log::error!("error processing file due to {}", err);
            return Err(ServiceError::OperationFailed);
        }

        let private_key = extract_env::<String>("IMAGEKIT_PRIVATE_KEY").unwrap();
        let public_key = extract_env::<String>("IMAGEKIT_PUBLIC_KEY").unwrap();

        let imagekit_upload_response = ImagekitClient::new(&public_key, &private_key)
            .map_err(|err| {
                log::error!("error creating client due to {}", err);
                ServiceError::OperationFailed
            })?
            .upload_file(&file_path, &file_name)
            .await
            .map_err(|err| {
                log::error!("error creating client due to {}", err);
                ServiceError::OperationFailed
            })?;

        self.user_repository
            .set_avatar_url(user_identifier, &imagekit_upload_response.url)
            .await?;

        Ok(())
    }

    async fn update_profile(
        &self,
        request: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.user_repository
            .update_profile(request, user_identifier)
            .await?;
        Ok(())
    }
}
