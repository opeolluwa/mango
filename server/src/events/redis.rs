use std::fmt::Debug;

use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

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

    pub async fn publish_message<T>(
        &mut self,
        channel: &RedisMessageChannel,
        message: &RedisMessage<T>,
    ) -> Result<(), ServiceError>
    where
        T: Serialize + DeserializeOwned + Debug,
    {
        let payload = serde_json::to_string(&message).map_err(ServiceError::SerdeJsonError)?;
        // self.connection_manager.publish() (channel.to_string(), payload)
        //     .await
        //     .map_err(ServiceError::RedisError)?;

        Ok(())
    }

    pub async fn consume_message<T, F>(
        &mut self,
        channel: &RedisMessageChannel,
        mut handler: F,
    ) -> Result<(), ServiceError>
    where
        T: Serialize + DeserializeOwned + Debug + Send + 'static,
        F: FnMut(RedisMessage<T>) -> Result<(), ServiceError> + Send + 'static,
    {
        use redis::AsyncCommands;
        use redis::aio::PubSub;

        let mut pubsub = self.connection_manager.subscribe(channel.to_string()).await.map_err(ServiceError::RedisError)?;

        // tokio::spawn(async move {
        //     loop {
        //         let msg = match pubsub.on_message().next().await {
        //             Some(msg) => msg,
        //             None => continue,
        //         };

        //         if let Ok(payload): Result<String, _> = msg.get_payload() {
        //             match serde_json::from_str::<RedisMessage<T>>(&payload) {
        //                 Ok(message) => {
        //                     if let Err(err) = handler(message) {
        //                         tracing::error!("Redis message handler error: {:?}", err);
        //                     }
        //                 }
        //                 Err(e) => tracing::error!("Failed to deserialize message: {:?}", e),
        //             }
        //         }
        //     }
        // });

        Ok(())
    }
}

