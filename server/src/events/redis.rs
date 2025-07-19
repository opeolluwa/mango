use redis::{
    AsyncCommands,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    errors::service_error::ServiceError, events::channels::RedisMessageChannel,
    shared::extract_env::extract_env,
};

pub struct RedisClient {
    connection_manager: ConnectionManager,
}

impl RedisClient {
    pub async fn new() -> Result<Self, ServiceError> {
        let redis_connection_url: String =
            extract_env("REDIS_CONNECTION_URL").map_err(ServiceError::from)?;
        let redis_client =
            redis::Client::open(redis_connection_url).map_err(ServiceError::RedisError)?;

        let config = ConnectionManagerConfig::new()
            .set_number_of_retries(5)
            .set_automatic_resubscription();
        let connection_manager =
            redis::aio::ConnectionManager::new_with_config(redis_client, config)
                .await
                .map_err(ServiceError::RedisError)?;

        Ok(Self { connection_manager })
    }
}

pub trait RedisClientExt {
    fn blacklist_refresh_token(
        &mut self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn save_refresh_token(
        &mut self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn fetch_refresh_token(
        &mut self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<Option<String>, ServiceError>> + Send;

    fn get_token_ttl(
        &mut self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<u64, ServiceError>>;

    fn publish_message<T: Serialize + DeserializeOwned + std::fmt::Debug>(
        &self,
        channel: &RedisMessageChannel,
        message: &T,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>>;

    fn consume_message(
        &self,
        channel: &RedisMessageChannel,
        message: impl ToString,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>>;
}

impl RedisClientExt for RedisClient {
    async fn blacklist_refresh_token(&mut self, token: &str) -> Result<(), ServiceError> {
        let key = &format!("blacklist_token:{}", token);
        let stored_token = self.fetch_refresh_token(token).await?;
        if stored_token.is_some() {
            let key = format!("refresh_token:{}", stored_token.unwrap());
            let ttl = self.get_token_ttl(&key).await?;
            let () = self
                .connection_manager
                .set_ex(key, token, ttl)
                .await
                .map_err(ServiceError::from)?;
        }
        let refresh_token_validity_in_minutes: u64 =
            extract_env("REFRESH_TOKEN_TTL_IN_MINUTES").unwrap_or(420);
        let validity_secs = refresh_token_validity_in_minutes * 60;

        let _: () = self
            .connection_manager
            .set_ex(key, token, validity_secs)
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

    async fn save_refresh_token(&mut self, token: &str) -> Result<(), ServiceError> {
        let key = format!("refresh_token:{}", token);
        let refresh_token_validity_in_minutes: u64 =
            extract_env("REFRESH_TOKEN_TTL_IN_MINUTES").unwrap_or(420);
        let validity_secs = refresh_token_validity_in_minutes * 60;

        let _: () = self
            .connection_manager
            .set_ex(key, token, validity_secs)
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

    async fn fetch_refresh_token(&mut self, token: &str) -> Result<Option<String>, ServiceError> {
        let key = &format!("refresh_token:{}", token);
        let result: Option<String> = self
            .connection_manager
            .get(key)
            .await
            .map_err(ServiceError::from)?;

        Ok(result)
    }

    async fn get_token_ttl(&mut self, key: &str) -> Result<u64, ServiceError> {
        let result: u64 = self
            .connection_manager
            .ttl(key)
            .await
            .map_err(ServiceError::from)?;

        Ok(result)
    }

    async fn publish_message<T>(
        &self,
        channel: &RedisMessageChannel,
        message: &T,
    ) -> Result<(), ServiceError>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        let message_as_str = serde_json::to_string(message).map_err(|err| {
            log::error!(
                "failed to serialize {:#?} as string due to {}",
                message,
                err
            );
            ServiceError::SerdeJsonError(err)
        })?;

        // let rr = self
        //     .connection_manager
        //     .publish(channel.to_string(), message_as_str)
        //     .await;
        todo!()
    }

    async fn consume_message(
        &self,
        channel: &RedisMessageChannel,
        message: impl ToString,
    ) -> Result<(), ServiceError> {
        todo!()
    }
}
