use aers_utils::generate_otp;
use chrono::{Local, TimeDelta};

const OTP_VALIDITY: TimeDelta = TimeDelta::minutes(5);
use crate::{
    errors::service_error::ServiceError,
    repositories::otp_repository::{OtpRepository, OtpRepositoryExt},
};

#[derive(Debug, Clone)]
pub struct OtpService {
    otp_repository: OtpRepository,
}

impl OtpService {
    pub fn init(pool: &sqlx::Pool<sqlx::Postgres>) -> Self {
        Self {
            otp_repository: OtpRepository::init(pool),
        }
    }
}

pub trait OtpServiceExt {
    fn new_otp_for_user(
        &self,
        user_identifier: &str,
    ) -> impl std::future::Future<Output = Result<String, ServiceError>> + Send;

    fn validate_otp_for_user(
        &self,
        user_identifier: &str,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<bool, ServiceError>> + Send;
}

impl OtpServiceExt for OtpService {
    async fn new_otp_for_user(&self, user_identifier: &str) -> Result<String, ServiceError> {
        let otp = generate_otp();
        self.otp_repository
            .new_with_user(user_identifier, &otp)
            .await
            .map_err(ServiceError::from)?;

        Ok(otp)
    }

    async fn validate_otp_for_user(
        &self,
        user_identifier: &str,
        otp: &str,
    ) -> Result<bool, ServiceError> {
        if let Some(stored_otp) = self
            .otp_repository
            .find_latest_by_user(user_identifier)
            .await
            .map_err(ServiceError::from)?
        {
            let now = Local::now();

            let is_not_expired = now - stored_otp.created_at <= OTP_VALIDITY;
            let is_match = stored_otp.code == otp;
            Ok(is_match && is_not_expired)
        } else {
            Ok(false)
        }
    }
}
