use std::{fs, io, path::Path, time::Duration};

use crate::{adapters::jwt::Claims, errors::service_error::ServiceError};
use aers_email_client::{
    ConfirmEmailTemplate, Email, EmailClient, ForgottenPasswordTemplate, PasswordUpdatedTemplate,
};
use aers_utils::extract_env;
use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;

const EMAIL_CONFIRMATION_EXPIRY: Duration = Duration::from_secs(2 * 60 * 60); // 2 hours

#[derive(Clone)]
pub struct ServiceHelpers {}

impl ServiceHelpers {
    pub fn init() -> Self {
        Self {}
    }
}

pub trait ServiceHelpersTrait {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError>;
    fn validate_password(&self, raw_password: &str, hash: &str) -> Result<bool, ServiceError>;
    fn delete_file_if_exists(path: &str) -> io::Result<()>;
    fn send_confirmation_email(
        &self,
        user_email: &str,
        identifier: Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn generate_otp(&self, user_email: &str) -> Result<String, ServiceError>;
}

impl ServiceHelpersTrait for ServiceHelpers {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError> {
        hash(raw_password.trim(), DEFAULT_COST).map_err(|err| {
            log::error!("operation failed due to {}", err);
            ServiceError::OperationFailed
        })
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        verify(password, hash).map_err(|err| {
            log::error!("operation failed due to {}", err);
            ServiceError::OperationFailed
        })
    }

    fn delete_file_if_exists(path: &str) -> io::Result<()> {
        let file_path = Path::new(path);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    async fn send_confirmation_email(
        &self,
        user_email: &str,
        identifier: Uuid,
    ) -> Result<(), ServiceError> {
        let claim = Claims::builder()
            .subject("confirm_account")
            .email(&user_email)
            .user_identifier(&identifier)
            .validity(EMAIL_CONFIRMATION_EXPIRY)
            .build_and_sign()?;

        let frontend_base_url: String = extract_env("FRONTEND_BASE_URL").map_err(|err| {
            log::error!("Failed to extract FRONTEND_BASE_URL: {}", err);
            ServiceError::OperationFailed
        })?;

        let verification_link = format!("{frontend_base_url}/verify?token={}", claim);

        let template = ConfirmEmailTemplate::new(user_email, &verification_link);
        let email = Email::builder()
            .subject("Confirm your account")
            .to(user_email)
            .template(template)
            .from("admin@eckko.oapp")
            .build();

        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send confirmation email due to: {}", err);
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    async fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), ServiceError> {
        let template = ForgottenPasswordTemplate::new(otp);

        let email = Email::builder()
            .subject("Forgotten Password")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send forgotten password email due to: {}", err);
            ServiceError::OperationFailed
        })?;

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send forgotten password email due to: {}", err);
            ServiceError::OperationFailed
        })?;

        Ok(())
    }
    async fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> Result<(), ServiceError> {
        let email = Email::builder()
            .subject("Password Updated")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {}", err);
            ServiceError::OperationFailed
        })?;

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {}", err);
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    async fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> Result<(), ServiceError> {
        let template = aers_email_client::WelcomeTemplate::new(user_name);
        let email = Email::builder()
            .subject("Welcome to Eckko")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send welcome email due to: {}", err);
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    fn generate_otp(&self, user_email: &str) -> Result<String, ServiceError> {
        let otp = aers_utils::generate_otp();
        //TODO: save the OTP in the database or cache for verification later

        log::info!("Generated OTP for {}: {}", user_email, otp);
        Ok(otp)
    }
}
