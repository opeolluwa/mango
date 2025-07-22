use std::fmt::{Debug, Display};

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

impl<T> Display for RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:#?}", self.identifier, self.payload)
    }
}
