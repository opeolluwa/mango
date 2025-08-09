use std::fmt::Debug;

use redis::AsyncTypedCommands;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    errors::service_error::ServiceError,
    events::{channels::EventChannel, message::Event, redis::RedisClient},
};

pub struct EventPrducer<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    channel: EventChannel,
    message: Event<T>,
}

impl<T> EventPrducer<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    pub fn new(channel: &EventChannel, message: T) -> Self {
        Self {
            channel: EventChannel::from(channel.to_string()),
            message: Event::new(message),
        }
    }
    pub async fn send(&self) -> Result<(), ServiceError> {
        let mut redis_client = RedisClient::new().await?;

        let message_as_str = serde_json::to_string(&self.message).map_err(|err| {
            log::error!(
                "failed to serialize {:#?} as string due to {}",
                self.message,
                err
            );
            ServiceError::SerdeJsonError(err)
        })?;

        redis_client
            .get_connection()
            .publish(self.channel.to_string(), message_as_str)
            // .publish(self.channel.to_string(), message_as_str)
            .await
            .map_err(ServiceError::from)?;

        Ok(())
    }

   
}
