use std::time::Duration;

use sqlx::{Pool, Postgres};

use crate::adapters::jwt::Claims;
use crate::entities::user::UserEntity;
use crate::errors;
use crate::errors::repository_error::RepositoryError;
use crate::errors::service_error::ServiceError;
use crate::{
    adapters::authentication::{
        CreateUserRequest, ForgottenPasswordRequest, ForgottenPasswordResponse, LoginRequest,
        LoginResponse, RefreshTokenRequest, RefreshTokenResponse, SetNewPasswordRequest,
        SetNewPasswordResponse, VerifyAccountRequest, VerifyAccountResponse,
    },
    errors::auth_error::AuthenticationError,
    repositories::user_repository::{UserRepository, UserRepositoryTrait},
    services::helper_service::{ServiceHelpers, ServiceHelpersTrait},
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
    user_helper_service: ServiceHelpers,
}

impl AuthenticationService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
            user_helper_service: ServiceHelpers::init(),
        }
    }
}
pub trait AuthenticationServiceTrait {
    fn create_account(
        &self,
        request: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn login(
        &self,
        request: &LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, ServiceError>> + Send;

    fn forgotten_password(
        &self,

        request: &ForgottenPasswordRequest,
    ) -> impl std::future::Future<Output = Result<ForgottenPasswordResponse, ServiceError>> + Send;

    fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<SetNewPasswordResponse, ServiceError>> + Send;

    fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, ServiceError>> + Send;

    fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResponse, ServiceError>> + Send;
}

impl AuthenticationServiceTrait for AuthenticationService {
    async fn create_account(&self, request: &CreateUserRequest) -> Result<(), ServiceError> {
        if self
            .user_repository
            .find_by_email(&request.email)
            .await
            .is_some()
        {
            return Err(RepositoryError::DuplicateRecord.into());
        }

        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        let user = CreateUserRequest {
            password: password_hash,
            first_name: request.first_name.to_owned(),
            email: request.email.to_owned(),
            last_name: request.last_name.to_owned(),
        };

        self.user_repository
            .create_user(user)
            .await
            .map_err(ServiceError::from)
    }

    async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, ServiceError> {
        let Some(user) = self.user_repository.find_by_email(&request.email).await else {
            return Err(AuthenticationError::WrongCredentials.into());
        };

        let valid_password = self
            .user_helper_service
            .validate_password(&request.password, &user.password)?;
        if !valid_password {
            return Err(AuthenticationError::WrongCredentials.into());
        }

        let access_token = Claims::builder()
            .subject("access_token")
            .email(&user.email)
            .user_identifier(user.identifier)
            .validity(Duration::from_secs(15 * 60 /*15 minutes */))
            .build()?;

        let refresh_token = Claims::builder()
            .subject("refresh_token")
            .email(&user.email)
            .user_identifier(user.identifier)
            .validity(Duration::from_secs(7 * 60 * 60 /*7 hours */))
            .build()?;

        Ok(LoginResponse {
            access_token: access_token.generate_token()?,
            refresh_token: refresh_token.generate_token()?,
            refresh_token_exp: refresh_token.exp,
            iat: access_token.iat,
            exp: access_token.exp,
            refresh_token_iat: refresh_token.iat,
        })
    }

    async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, ServiceError> {
        let user = self.user_repository.find_by_email(&request.email).await;
        if user.is_none() {
            return Err(AuthenticationError::WrongCredentials.into());
        };

        tokio::task::spawn(async move { todo!("send account retrival email") });

        let UserEntity {
            email, identifier, ..
        } = user.unwrap();

        let token = Claims::builder()
            .subject("forgotten_password")
            .email(&email)
            .user_identifier(identifier)
            .validity(Duration::from_secs(15 * 60 /*15 minutes */))
            .build_and_sign()?;

        Ok(ForgottenPasswordResponse { token })
    }

    async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> Result<SetNewPasswordResponse, ServiceError> {
        let new_password = self.user_helper_service.hash_password(&request.password)?;

        if self
            .user_repository
            .find_by_identifier(&claims.user_identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationError::InvalidToken.into());
        };

        self.user_repository
            .update_password(&claims.user_identifier, &new_password)
            .await?;

        Ok(SetNewPasswordResponse {})
    }

    async fn verify_account(
        &self,
        claims: &Claims,
        _request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError> {
        if self
            .user_repository
            .find_by_identifier(&claims.user_identifier)
            .await
            .is_none()
        {
            return Err(errors::service_error::ServiceError::AuthenticationError(
                AuthenticationError::InvalidToken,
            ));
        };

        //todo: validate account credentials
        self.user_repository
            .update_account_status(&claims.user_identifier)
            .await?;
        Ok(VerifyAccountResponse {})
    }

    async fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, ServiceError> {
        let access_token = Claims::builder()
            .subject("access_token")
            .email(&request.email)
            .user_identifier(request.user_identifier)
            .validity(Duration::from_secs(15 * 60 /*15 minutes */))
            .build()?;

        let refresh_token = Claims::builder()
            .subject("refresh_token")
            .email(&request.email)
            .user_identifier(request.user_identifier)
            .validity(Duration::from_secs(7 * 60 * 60 /*7 hours */))
            .build()?;

        Ok(LoginResponse {
            access_token: access_token.generate_token()?,
            refresh_token: refresh_token.generate_token()?,
            refresh_token_exp: refresh_token.exp,
            refresh_token_iat: refresh_token.iat,
            iat: access_token.iat,
            exp: access_token.exp,
        })
    }
}
