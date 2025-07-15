use std::fmt::Debug;

use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    errors::service_error::ServiceError,
    events::{channels::RedisMessageChannel, producers::RedisMessage},
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

    pub fn publish_message<T>(
        &self,
        channel: &RedisMessageChannel,
        message: &RedisMessage<T>,
    ) -> Result<(), ServiceError>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        // let connection = self
        //     .connection_manager
        //     .publish(channel.to_string(), );
        // let _ = message;
        todo!()
    }

    pub fn consume_message() {}
}

// pub trait RedisClientExt {
//     fn extract_conn(&self) -> redis::Connection;
// }

// impl RedisClientExt for RedisClient {
//     fn extract_conn(&self) -> redis::Connection {
//         self.redis_connection
//     }
// }
