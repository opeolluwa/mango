use std::str::FromStr;
use std::sync::Arc;

use sqlx::Postgres;
use sqlx::pool::Pool;
use uuid::Uuid;

use crate::entities::otp::Otp;
use crate::errors::repository_error::RepositoryError;

#[derive(Debug, Clone)]
pub struct OtpRepository {
    pool: Arc<Pool<Postgres>>,
}

impl OtpRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait OtpRepositoryExt {
    fn new_with_user(
        &self,
        user_identifier: &str,
        code: &str,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn find_latest_by_user(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Otp>, RepositoryError>> + Send;

    fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Otp>, RepositoryError>> + Send;

    fn delete_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}

impl OtpRepositoryExt for OtpRepository {
    async fn new_with_user(
        &self,
        user_identifier: &str,
        code: &str,
    ) -> Result<(), RepositoryError> {
        let otp_identifier = Uuid::new_v4();
        let user_identifier = Uuid::from_str(user_identifier)
            .map_err(|err| RepositoryError::OperationFailed(err.to_string()))?;

        sqlx::query(r#"INSERT INTO one_time_passwords (identifier, user_identifier, code) VALUES ($1,$2,$3)"#).bind(otp_identifier).bind(user_identifier).bind(code).execute(self.pool.as_ref()).await.map_err(RepositoryError::SqlxError)?;

        Ok(())
    }

    async fn find_latest_by_user(
        &self,
        user_identifier: &Uuid,
    ) -> Result<Option<Otp>, RepositoryError> {
        sqlx::query_as::<_, Otp>(
        r#"SELECT * FROM one_time_passwords WHERE user_identifier = $1 ORDER BY created_at DESC LIMIT 1"#,
    ).bind(user_identifier)
    .fetch_optional(self.pool.as_ref())
    .await.map_err(RepositoryError::from)
    }

    async fn delete_by_identifier(&self, identifier: &Uuid) -> Result<(), RepositoryError> {
        sqlx::query(r#"DELETE FROM one_time_passwords WHERE identifier = $1"#)
            .bind(identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::from)?;

        Ok(())
    }

    async fn find_by_identifier(&self, identifier: &Uuid) -> Result<Option<Otp>, RepositoryError> {
        sqlx::query_as::<_, Otp>(r#"SELECT * FROM one_time_passwords WHERE identifier = $1"#)
            .bind(identifier)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(RepositoryError::from)
    }
}
