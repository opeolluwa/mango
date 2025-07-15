use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    adapters::{
        authentication::CreateUserRequest,
        users::{PartialUserProfile, UserDto},
    },
    entities::user::UserEntity,
    errors::{
        repository_error::RepositoryError, service_error::ServiceError,
        user_service_error::UserServiceError,
    },
};

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<Pool<Postgres>>,
}

impl UserRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait UserRepositoryTrait {
    fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Option<UserEntity>> + Send;

    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Option<UserEntity>> + Send;

    fn update_account_status(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn update_password(
        &self,
        identifier: &Uuid,
        new_password: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn create_user(
        &self,
        user: CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<(), UserServiceError>> + Send;

    fn retrieve_information(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<UserDto, UserServiceError>> + Send;

    fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> impl std::future::Future<Output = Result<(), UserServiceError>>;

    fn update_profile(
        &self,
        request: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>>;
}

impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, user: CreateUserRequest) -> Result<(), UserServiceError> {
        sqlx::query(
    "INSERT INTO users (identifier, first_name, last_name, email, password) VALUES ($1, $2, $3, $4, $5)"
)
.bind(uuid::Uuid::new_v4())
.bind(user.first_name)
.bind(user.last_name)
.bind(user.email)
.bind(user.password)
.execute(self.pool.as_ref())
.await
.map_err(|err| UserServiceError::OperationFailed(err.to_string()))?;

        Ok(())
    }
    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<UserEntity> {
        sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE identifier = $1")
            .bind(identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }

    async fn find_by_email(&self, email: &str) -> Option<UserEntity> {
        sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }

    async fn update_account_status(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        let _ = sqlx::query_as::<_, UserEntity>(
            "UPDATE users SET is_active = $1  WHERE identifier = $2",
        )
        .bind(true)
        .bind(identifier.to_string())
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|err| UserServiceError::OperationFailed(err.to_string()));

        Ok(())
    }

    async fn update_password(
        &self,
        identifier: &Uuid,
        new_password: &str,
    ) -> Result<(), ServiceError> {
        let _ = sqlx::query_as::<_, UserEntity>(
            "UPDATE users SET password = $1  WHERE identifier  = $2",
        )
        .bind(new_password)
        .bind(identifier)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|err| UserServiceError::OperationFailed(err.to_string()));

        Ok(())
    }
    async fn retrieve_information(&self, identifier: &Uuid) -> Result<UserDto, UserServiceError> {
        sqlx::query_as::<_, UserDto>(r#"SELECT * FROM users  WHERE identifier = $1"#)
            .bind(identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(UserServiceError::from)
    }

    async fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> Result<(), UserServiceError> {
        sqlx::query(r#"UPDATE users SET avatar = $2 WHERE identifier = $1"#)
            .bind(user_identifier)
            .bind(avatar_url)
            .execute(self.pool.as_ref())
            .await
            .map_err(UserServiceError::from)?;

        Ok(())
    }

    async fn update_profile(
        &self,
        PartialUserProfile {
            email,
            first_name,
            last_name,
        }: &PartialUserProfile,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        let user = self
            .find_by_identifier(user_identifier)
            .await
            .ok_or(RepositoryError::RecordNotFound)
            .map_err(ServiceError::from)?;

        sqlx::query(r#"UPDATE users SET email =$1, first_name = $2, last_name =$3 WHERE user_identifier =$4"#).bind(email.clone().unwrap_or(user.email)).bind(first_name.clone(). unwrap_or(user.first_name)).bind(last_name.clone().unwrap_or(user.last_name)).bind(user_identifier).execute(self.pool.as_ref()).await.map_err(UserServiceError::from)?;

        Ok(())
    }
}
