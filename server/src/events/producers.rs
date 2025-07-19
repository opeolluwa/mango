use std::fmt::Debug;

use redis::ToRedisArgs;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    identifier: Uuid,
    payload: T,
}


pub fn publish_message<T>(channel: &str, message: T)
where
    T: Serialize + DeserializeOwned + Debug,
{
    let message = RedisMessage {
        identifier: Uuid::new_v4(),
        payload: message,
    };

}